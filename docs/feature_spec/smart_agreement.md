# Smart Agreement Spec

### AgreementInfo
```rust
pub trait AgreementInfo {
    /// parties who are partaking in an agreement
    /// - Must have atleast one buyer and seller
    type Participants;
	/// Party which is buying service or product
	type Buyer;
	/// Party providing service or product
	type Seller;
    /// Identfication of a particular agreement for query lookup
    type AgreementId;
    /// An amount of currency to Lock into an Escrow until a Service has been
    /// completed.
    type Amount;
    /// A hash of the Agreement Data.
    type AgreementDataHash;

	/// If possible get the buyer from the participants
	fn try_get_buyer(&self) ->
        Result<Participant<AccountId>, sp_runtime::DispatchError>;
	/// If possible get the seller from the participants
	fn try_get_seller(&self) ->
        Result<Participant<AccountId>, sp_runtime::DispatchError>;
	/// Get the Amount of currency in this agreement
	fn amount(&self) -> Self::Amount;
	/// The hash of the scale_encoded bytes of the agreement.
	fn agreement_data_hash(&self) -> Self::AgreementDataHash;
	/// The Identifier of this Agreement amongst all the agreements
	fn agreement_id(&self) -> Self::AgreementId;
    /// participants who are engaged in this agreement
	fn participants(&self) -> Self::Participants;
}
```

### ServiceAgreement
```rust
/// Particular type of agreement
pub struct ServiceAgreement<AccountId> {
	pub participants: Vec<Participant<AccountId>>,
	pub agreement_id: H256,
	pub amount: Option<u64>,
	pub agreement_data_hash: H256,
}
```

### Participant
```rust
struct Participant<AccountId> {
    account_id: AccountId,
    role: Role,
}

enum Role {
    /// A party who is receiving a service or product from an agreement.
    Buyer,
    /// A party who is providing a service or product for an agreement.
    Seller,
}

```

### VoteInformation
```rust
pub struct VoteInfo<AccountId> {
    /// Which accounts have voted for a particular proposal
    accounts_voted: Vec<AccountId>,
    /// What is the total number of votes for this Proposal
    total_votes: u32,
}
```

### Events
```rust
pub enum Event<T: Config> {
    AgreementProposed {
        proposer: T::AccountId,
        buyer: T::AccountId,
        seller: T::AccountId,
        agreement_id: AgreementId<T>,
        agreement: ServiceAgreement<T::AccountId>,
    },
    AgreementApproved {
        agreement_id: AgreementId<T>,
        agreement: ServiceAgreement<T::AccountId>,
    },
}
```

### Errors
```rust
pub enum Error<T> {
    /// Thrown when agreement proposer isnt a participant of the proposed agreement
    ProposerNotParticipant,
    /// Total cap on agreements is reached
    MaxAgreementsReached,
    /// Every user can have a capped amount of proposals they can be engaged in
    MaxProposalsExceededForUser,
    /// Thrown when someone is trying to sign/vote on a proposal which they are not in
    ApproverNotValidParticipant,
    /// Thrown when trying to approve a proposal which does not exist
    AgreementNotProposed,
    /// Thrown when a user is trying to double sign/vote for the same proposal
    DoubleVote,
    /// Thrown when trying to sign/vote for a proposal which has already passed
    ApprovalAlreadyAchievedForProposal,
}
```

### Pallet Storage
```rust
/// Query Agreement which has been proposed but not approved
pub type ProposedServiceAgreements<T: Config> =
    StorageMap<_, _, AgreementId<T>, ServiceAgreement<T::AccountId>, OptionQuery>;

/// Query the voting information for a particular proposal.
pub type ProposalVotes<T: Config> =
    	StorageMap<_, _, AgreementId<T>, VoteInfo<T::AccountId>, OptionQuery>;

/// Query the agreements which this account has signed(voted) for.
pub type ProposalVotesForAccount<T: Config> =
		StorageMap<_, _, T::AccountId, Vec<AgreementId<T>>, ValueQuery>;

/// Query the number of proposals for a particular account
pub type NumProposalsForAccount<T: Config> =
		StorageMap<_, _, T::AccountId, u32, ValueQuery>;

/// Query Agreement which is approved and has escrow funds locked
pub type ApprovedServiceAgreements<T: Config> = StorageMap<
    _,
    _,
    AgreementId<T>,
    ServiceAgreement<T::AccountId>,
    OptionQuery,
>;

/// Query total number of agreements ever created used to get AgreementId Hash
pub type AgreementCount<T> = StorageValue<_, u128, ValueQuery>;
```

### Extrinsics
```rust
/// Propose an agreement to be signed and agreed upon.
///
/// Checks:
/// - If proposer is either the seller or buyer
/// - Make sure atleast One Buyer and One Seller
/// - Place in ServiceAgreementProposals
///
pub fn propose_agreement(origin: OriginFor<T>, agreement: ServiceAgreement);

/// Signs the corresponding Agreement proposal.
///
/// Checks:
/// - can only approve if origin is one of the agreement participants
/// - a participant can only vote once
/// - atleast one buyer and seller must vote
/// - Check number of votes for agreement is below the threshold for approval
/// - If num of votes for agreement is met then enact escrow
///
pub fn approve_agreement(agreement_id: AgreementId<T>);

/// Pushes approved agreement through to escrow
/// Checks:
/// - Agreement is approved
/// Action:
/// - Remove Agreement from Proposed Agreements and Cleanup Storage
///
fn start_escrow_on_approved_agreement(agreement_id: AgreementId<T>);
```

### Intrinsics
```rust
/// Check the current vote info to take action and approve proposed agreement
/// or update.
/// Checks:
/// - If agreement proposal already has votes necessary to pass
/// - If agreement proposal has enough votes to be approved
/// Actions:
/// - Increment total_votes for this agreement
/// - Approve the agreement move to approved agreements storage
/// - Throw agreement passed event
fn check_vote_info(
    approver: &T::AccountId,
    agreement_id: &AgreementId<T>,
    vote_info: &mut VoteInfo<T::AccountId>,
    agreement: &ServiceAgreement<T::AccountId>,
);
```

