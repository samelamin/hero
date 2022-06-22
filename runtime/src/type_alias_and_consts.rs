pub use super::{AllPalletsWithSystem, Call, Runtime};
	pub use sp_runtime::{
		generic,
		traits::{self, BlakeTwo256, IdentifyAccount, Verify},
		MultiAddress, MultiSignature,
	};

	/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
	pub type Signature = MultiSignature;

	/// Some way of identifying an account on the chain. We intentionally make it equivalent
	/// to the public key of our transaction signing scheme.
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

	/// Balance of an account.
	pub type Balance = u128;

	/// Index of a transaction in the chain.
	pub type Index = u32;

	/// A hash of some data used by the chain.
	pub type Hash = sp_core::H256;

	/// An index to a block.
	pub type BlockNumber = u32;

	/// The address format for describing accounts.
	pub type Address = MultiAddress<AccountId, ()>;

	/// Block header type as expected by this runtime.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

	/// Block type as expected by this runtime.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;

	/// A Block signed with a Justification
	pub type SignedBlock = generic::SignedBlock<Block>;

	/// BlockId type as expected by this runtime.
	pub type BlockId = generic::BlockId<Block>;

	/// The SignedExtension to the basic transaction logic.
	pub type SignedExtra = (
		frame_system::CheckNonZeroSender<Runtime>,
		frame_system::CheckSpecVersion<Runtime>,
		frame_system::CheckTxVersion<Runtime>,
		frame_system::CheckGenesis<Runtime>,
		frame_system::CheckEra<Runtime>,
		frame_system::CheckNonce<Runtime>,
		frame_system::CheckWeight<Runtime>,
		pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	);

	/// Unchecked extrinsic type as expected by this runtime.
	//pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
	pub type UncheckedExtrinsic =
		fp_self_contained::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

	/// Extrinsic type that has already been checked.
	pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;

	/// Executive: handles dispatch to the various modules.
	pub type Executive = frame_executive::Executive<
		Runtime,
		Block,
		frame_system::ChainContext<Runtime>,
		Runtime,
		AllPalletsWithSystem,
	>;

	/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
	/// the specifics of the runtime. They can then be made to be agnostic over specific formats
	/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
	/// to even the core data structures.
	pub mod opaque {
		use super::*;
		use sp_runtime::{generic, traits::BlakeTwo256};

		pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
		/// Opaque block header type.
		pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
		/// Opaque block type.
		pub type Block = generic::Block<Header, UncheckedExtrinsic>;
		/// Opaque block identifier type.
		pub type BlockId = generic::BlockId<Block>;
	}

	// Below here are constants
	// note these do not include definitions made by the `parameter_types!` macro

	/// This determines the average expected block time that we are targeting.
	/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
	/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
	/// up by `pallet_aura` to implement `fn slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: u64 = 12000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	// Unit = the base number of indivisible units for balances
	pub const UNIT: Balance = 1_000_000_000_000;
	pub const MILLIUNIT: Balance = 1_000_000_000;
	pub const MICROUNIT: Balance = 1_000_000;

	/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
	pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

	pub mod currency {
		use crate::type_alias_and_consts::Balance;

		// Provide a common factor between runtimes based on a supply of tokens.
		pub const SUPPLY_FACTOR: Balance = 100;

		pub const WEI: Balance = 1;
		pub const KILOWEI: Balance = 1_000;
		pub const MEGAWEI: Balance = 1_000_000;
		pub const GIGAWEI: Balance = 1_000_000_000;
		pub const MICROPAID: Balance = 1_000_000_000_000;
		pub const MILLIPAID: Balance = 1_000_000_000_000_000;
		pub const PAID: Balance = 1_000_000_000_000_000_000;
		pub const KILOPAID: Balance = 1_000_000_000_000_000_000_000;

		pub const TRANSACTION_BYTE_FEE: Balance = 10 * MICROPAID * SUPPLY_FACTOR;
		pub const STORAGE_BYTE_FEE: Balance = 100 * MICROPAID * SUPPLY_FACTOR;

		pub const fn deposit(items: u32, bytes: u32) -> Balance {
			items as Balance * 100 * MILLIPAID * SUPPLY_FACTOR +
				(bytes as Balance) * STORAGE_BYTE_FEE
		}
	}
