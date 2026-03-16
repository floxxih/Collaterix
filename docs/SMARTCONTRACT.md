# Smart Contract Documentation

## Overview

StellarLend smart contracts are written in Rust using the Soroban SDK. They handle all core protocol logic including lending pools, loans, and liquidations.

## Contract Interface

### Initialization

```rust
pub fn init(env: Env, admin: Address) -> Result<(), Error>
```

Initialize the lending protocol with an admin address.

**Parameters:**
- `env`: Soroban environment
- `admin`: Address of protocol administrator

**Returns:** `Result<(), Error>`

**Errors:**
- `AlreadyInitialized`: Contract already initialized

---

### Supply Liquidity

```rust
pub fn supply(
    env: Env,
    supplier: Address,
    asset: Address,
    amount: i128,
) -> Result<(), Error>
```

Supply assets to a lending pool to earn interest.

**Parameters:**
- `supplier`: Address supplying liquidity (requires auth)
- `asset`: Asset contract address
- `amount`: Amount to supply (must be > 0)

**Returns:** `Result<(), Error>`

**Errors:**
- `InvalidAmount`: Amount <= 0
- `AssetNotSupported`: Asset not whitelisted

**Events:** `LiquiditySupplied(supplier, asset, amount)`

---

### Withdraw Liquidity

```rust
pub fn withdraw(
    env: Env,
    supplier: Address,
    asset: Address,
    amount: i128,
) -> Result<(), Error>
```

Withdraw supplied assets from a lending pool.

**Parameters:**
- `supplier`: Address withdrawing (requires auth)
- `asset`: Asset contract address
- `amount`: Amount to withdraw

**Returns:** `Result<(), Error>`

**Errors:**
- `InvalidAmount`: Amount <= 0 or exceeds supplied
- `InsufficientLiquidity`: Pool doesn't have available liquidity
- `PoolNotFound`: Asset pool doesn't exist

**Events:** `LiquidityWithdrawn(supplier, asset, amount)`

---

### Borrow

```rust
pub fn borrow(
    env: Env,
    borrower: Address,
    lender: Address,
    collateral_amount: i128,
    loan_amount: i128,
    interest_rate: u32,
    duration: u64,
) -> Result<u64, Error>
```

Create a collateralized loan.

**Parameters:**
- `borrower`: Borrower address (requires auth)
- `lender`: Address of lending pool
- `collateral_amount`: Collateral in base units
- `loan_amount`: Loan amount requested
- `interest_rate`: Annual rate in basis points (500 = 5%)
- `duration`: Loan duration in seconds

**Returns:** `Result<u64, Error>` - Loan ID

**Errors:**
- `InvalidAmount`: Amount <= 0
- `InvalidInterestRate`: Rate > 100%
- `InsufficientCollateral`: Collateral ratio < 120%
- `InsufficientLiquidity`: Pool can't fulfill loan

**Events:** `LoanCreated(loan_id, borrower, amount, collateral)`

**Collateral Requirements:**
- Minimum collateral ratio: 120%
- Formula: `(collateral_amount / loan_amount) * 100 >= 120`

---

### Repay Loan

```rust
pub fn repay(
    env: Env,
    borrower: Address,
    loan_id: u64,
    amount: i128,
) -> Result<(), Error>
```

Repay a loan partially or fully.

**Parameters:**
- `borrower`: Borrower address (requires auth)
- `loan_id`: ID of loan to repay
- `amount`: Repayment amount (including interest)

**Returns:** `Result<(), Error>`

**Errors:**
- `LoanNotFound`: Invalid loan ID
- `LoanAlreadyRepaid`: Loan already fully repaid
- `InvalidAmount`: Amount <= 0

**Events:** `LoanRepaid(loan_id, amount, remaining)`

**Interest Calculation:**
```
interest = (principal * rate * duration) / (BASIS_POINTS * SECONDS_PER_YEAR)
total_due = principal + interest
```

---

### Liquidate Loan

```rust
pub fn liquidate(
    env: Env,
    liquidator: Address,
    loan_id: u64,
) -> Result<(), Error>
```

Liquidate an undercollateralized loan.

**Parameters:**
- `liquidator`: Address performing liquidation (requires auth)
- `loan_id`: ID of loan to liquidate

**Returns:** `Result<(), Error>`

**Errors:**
- `LoanNotFound`: Invalid loan ID
- `LiquidationNotAllowed`: Loan not in Active status
- `CollateralBelowThreshold`: Health factor >= 110%

**Events:** `LoanLiquidated(loan_id, liquidator, collateral_claimed)`

**Liquidation Conditions:**
- Health factor < 110%
- Loan status = Active
- Formula: `(collateral_value / loan_value) * 100 < 110`

---

### Get Loan

```rust
pub fn get_loan(env: Env, loan_id: u64) -> Result<Loan, Error>
```

Retrieve loan details.

**Returns:** `Loan` struct or `LoanNotFound` error

---

### Get Pool

```rust
pub fn get_pool(env: Env, asset: Address) -> Result<LendingPool, Error>
```

Retrieve lending pool details.

**Returns:** `LendingPool` struct or `PoolNotFound` error

---

### Get User Position

```rust
pub fn get_position(env: Env, user: Address) -> Result<UserPosition, Error>
```

Retrieve user's lending/borrowing position.

**Returns:** `UserPosition` struct or `Unauthorized` error

---

### Check Health Factor

```rust
pub fn check_health(env: Env, loan_id: u64) -> Result<u32, Error>
```

Check loan health factor (collateral ratio).

**Returns:** Health factor as percentage (e.g., 150 = 150%)

---

### Add Supported Asset (Admin Only)

```rust
pub fn add_asset(
    env: Env,
    admin: Address,
    asset: Address,
) -> Result<(), Error>
```

Whitelist an asset for lending/borrowing.

**Parameters:**
- `admin`: Admin address (requires auth)
- `asset`: Asset contract address to whitelist

**Errors:**
- `Unauthorized`: Caller is not admin

---

## Data Structures

### Loan

```rust
pub struct Loan {
    pub borrower: Address,
    pub lender: Address,
    pub collateral_amount: i128,
    pub loan_amount: i128,
    pub interest_rate: u32,
    pub collateral_ratio: u32,
    pub start_time: u64,
    pub due_time: u64,
    pub repaid_amount: i128,
    pub status: LoanStatus,
}
```

### LoanStatus

```rust
pub enum LoanStatus {
    Active,      // Loan is active
    Repaid,      // Fully repaid
    Liquidated,  // Liquidated due to low collateral
    Defaulted,   // Defaulted (future use)
}
```

### LendingPool

```rust
pub struct LendingPool {
    pub asset: Address,
    pub total_supplied: i128,
    pub total_borrowed: i128,
    pub base_interest_rate: u32,
    pub utilization_rate: u32,
}
```

### UserPosition

```rust
pub struct UserPosition {
    pub user: Address,
    pub supplied: i128,
    pub borrowed: i128,
    pub collateral: i128,
    pub health_factor: u32,
}
```

---

## Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 1 | AlreadyInitialized | Contract already initialized |
| 2 | NotInitialized | Contract not initialized |
| 3 | Unauthorized | Caller not authorized |
| 4 | InvalidAmount | Amount is zero or negative |
| 5 | InsufficientCollateral | Collateral ratio too low |
| 6 | InsufficientLiquidity | Pool lacks liquidity |
| 7 | LoanNotFound | Loan ID doesn't exist |
| 8 | LoanAlreadyRepaid | Loan already repaid |
| 9 | LoanNotDue | Loan not yet due |
| 10 | CollateralBelowThreshold | Collateral above liquidation threshold |
| 11 | InvalidInterestRate | Interest rate out of bounds |
| 12 | InvalidCollateralRatio | Collateral ratio invalid |
| 13 | LiquidationNotAllowed | Loan cannot be liquidated |
| 14 | PoolNotFound | Lending pool doesn't exist |
| 15 | AssetNotSupported | Asset not whitelisted |

---

## Building & Testing

### Build

```bash
# Development build
cargo build

# Optimized WASM build
cargo build --release --target wasm32-unknown-unknown
```

### Test

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_borrow_with_collateral

# Run with output
cargo test -- --nocapture
```

### Deploy

```bash
# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_lend.wasm \
  --network testnet \
  --source ACCOUNT_SECRET

# Initialize
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  --source ACCOUNT_SECRET \
  -- init \
  --admin ADMIN_PUBLIC_KEY
```

---

## Security Considerations

1. **Collateral Checks**: Always validate collateral ratio >= 120%
2. **Authorization**: All mutating functions require caller authentication
3. **Integer Overflow**: Soroban SDK provides overflow protection
4. **Reentrancy**: Not a concern with Soroban's execution model
5. **Price Oracles**: Future enhancement for accurate asset pricing

---

## Gas Optimization

- Minimize storage reads/writes
- Use efficient data structures
- Batch operations where possible
- Cache frequently accessed values

---

## Events (Future Enhancement)

Events for off-chain indexing:
- `LiquiditySupplied`
- `LiquidityWithdrawn`
- `LoanCreated`
- `LoanRepaid`
- `LoanLiquidated`

---

## Integration Example

```rust
use stellar_lend::StellarLendClient;

// Create client
let client = StellarLendClient::new(&env, &contract_id);

// Supply liquidity
client.supply(&supplier, &asset, &1000000);

// Borrow
let loan_id = client.borrow(
    &borrower,
    &pool_address,
    &1500000,  // collateral (150% ratio)
    &1000000,  // loan amount
    &500,      // 5% interest
    &2592000,  // 30 days
);

// Check health
let health = client.check_health(&loan_id);
```
