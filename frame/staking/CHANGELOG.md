# CHANGELOG(v2.0.0.alpha.3)

## Core

Some concepts should have some explaination for the changing from substrate

### power

power is a mixture of ring and oxor.

+ *XOR*: `power = xor_ratio * POWER_COUNT / 2`
+ *OXOR*: `power = oxor_ratio * POWER_COUNT / 2`

We use `currency_to_power` and `power_of` to calculcate `power`.

### rebond

We doesn't support `rebond` currently now.

### withdraw

What should happen after all balances being unbonded?(the locked balance)


## Moudle
### delete `withdraw_unbond`

+ **withdraw_unbond**: Remove all associated data of a stash account from the staking system.

Oracol has `active_balance` and `active_deposit_balance`, we calculate `normal_balance` by `active_balance - active_deposit_balance`, the `normal_balance` is **free to transfer**, so we don't need the `withdraw_unbond` function actually.

### delete `slashable_balance_of`

+ **slashable_balance_of**: The total balance that can be slashed from a stash account as of right now.

We use `power_of` and `stake_of` instead of `slashable_balance_of`:

+ **power_of**: The total power that can be slashed from a stash account as of right now.
+ **stake_of**: The `active_xor` and `active_oxor` from a stash account.

**For if an account is slashale:**

Just use `power_of`, if the return `power` is zero, the target account is not slashable.

**For the amount of slashable balances:**

The slashable balances actually mean `active-ring` and `active-oxor` in oracol's staking
process, we can use `Staking::ledger(controller)` to get a `StakingLedger` which contains
the `active-ring` and `active-oxor` the `controller` have.

## Structs

### Exposure

A snapshot of the stake backing a single validator in the system.

> oracol

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, XorBalance, OxorBalance>
where
	XorBalance: HasCompact,
	OxorBalance: HasCompact,
{
	#[codec(compact)]
	pub own_xor_balance: XorBalance,
	#[codec(compact)]
	pub own_oxor_balance: OxorBalance,
	pub own_power: Power,
	pub total_power: Power,
	pub others: Vec<IndividualExposure<AccountId, XorBalance, OxorBalance>>,
}
```

> substrate

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, Balance: HasCompact> {
	/// The total balance backing this validator.
	#[codec(compact)]
	pub total: Balance,
	/// The validator's own stash that is exposed.
	#[codec(compact)]
	pub own: Balance,
	/// The portions of nominators stashes that are exposed.
	pub others: Vec<IndividualExposure<AccountId, Balance>>,
}
```

### IndividualExposure

The amount of exposure (to slashing) than an individual nominator has.

> oracol

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]
pub struct IndividualExposure<AccountId, XorBalance, OxorBalance>
where
	XorBalance: HasCompact,
	OxorBalance: HasCompact,
{
	who: AccountId,
	#[codec(compact)]
	xor_balance: XorBalance,
	#[codec(compact)]
	oxor_balance: OxorBalance,
	power: Power,
}
```

> substrate
```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]
pub struct IndividualExposure<AccountId, Balance: HasCompact> {
	/// The stash account of the nominator in question.
	who: AccountId,
	/// Amount of funds exposed.
	#[codec(compact)]
	value: Balance,
}
```


### StakingLedger

The ledger of a (bonded) stash.

+ annotated `rebond`

Currently we don't have this requirement.

> oracol
```rust
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, RuntimeDebug)]
pub struct StakingLedger<AccountId, XorBalance, OxorBalance, BlockNumber, Timestamp>
where
	XorBalance: HasCompact,
	OxorBalance: HasCompact,
{
	pub stash: AccountId,
  #[codec(compact)]
	pub active_xor: XorBalance,
  #[codec(compact)]
	pub active_deposit_xor: XorBalance,
	#[codec(compact)]
	pub active_oxor: OxorBalance,
	pub deposit_items: Vec<TimeDepositItem<XorBalance, Timestamp>>,
	pub xor_staking_lock: StakingLock<XorBalance, BlockNumber>,
	pub oxor_staking_lock: StakingLock<OxorBalance, BlockNumber>,
}
```

> substrate

```rust
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct StakingLedger<AccountId, Balance: HasCompact> {
	pub stash: AccountId,
	/// The total amount of the stash's balance that we are currently accounting for.
	/// It's just `active` plus all the `unlocking` balances.
	#[codec(compact)]
	pub total: Balance,
	/// The total amount of the stash's balance that will be at stake in any forthcoming
	/// rounds.
	#[codec(compact)]
	pub active: Balance,
	/// Any balance that is becoming free, which may eventually be transferred out
	/// of the stash (assuming it doesn't get slashed first).
	pub unlocking: Vec<UnlockChunk<Balance>>,
}
```
