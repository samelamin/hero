// common types and constants used in both pallet tests and runtime
#![cfg_attr(not(feature = "std"), no_std)]
pub mod constants {
  //copied from runtime/type_alias_and_consts.rs
  pub type Balance = u128;
  pub const MICROHERO: Balance = 1_000_000_000_000; // 10−6 	0.000001
  pub const MILLIHERO: Balance = 1_000 * MICROHERO; // 10−3 	0.001
  pub const HERO: Balance = 1_000 * MILLIHERO;

  /// pallet-bridge settings
  /// Additional fee charged when moving native tokens to target chains (in HEROs).
  pub const NATIVE_TOKEN_TRANSFER_FEE: Balance = 2000 * HERO;

  /// Additional fee charged when moving NFTs to target chains (in HEROs).
  pub const NFT_TOKEN_TRANSFER_FEE: Balance = 20 * HERO;
}