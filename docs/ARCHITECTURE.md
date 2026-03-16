# Collaterix Architecture

## Overview

Collaterix is a decentralized lending protocol built on Stellar's Soroban smart contract platform. It enables permissionless lending and borrowing with collateralized loans.

## System Components

### 1. Smart Contracts (Soroban/Rust)

The core protocol logic lives in Soroban smart contracts:

#### Main Contract: `Collaterix`
- **Initialization**: Set up protocol admin
- **Pool Management**: Handle liquidity pools per asset
- **Loan Management**: Create, track, and manage loans
- **Liquidation**: Process undercollateralized loan liquidations

#### Key Modules:

**error.rs**
- Defines all protocol errors
- Error codes for invalid operations
- Used across all contract functions

**types.rs**
- Core data structures: `Loan`, `LendingPool`, `UserPosition`
- Enums: `LoanStatus`
- Defines on-chain data models

**storage.rs**
- Persistent storage operations
- Key-value storage patterns
- Data retrieval and writing functions

**pool.rs**
- Liquidity pool logic
- Supply/withdraw operations
- Interest rate calculations
- Utilization rate tracking

**loan.rs**
- Loan creation with collateral checks
- Repayment processing
- Liquidation logic
- Health factor calculations

### 2. Backend Services (Python/FastAPI)

#### REST API
- Exposes HTTP endpoints for frontend
- Aggregates blockchain data
- Provides indexing and caching

#### Database (PostgreSQL)
Schema includes:
- `users`: User accounts
- `loans`: Loan records with status
- `lending_pools`: Pool statistics
- `user_positions`: User balances
- `transactions`: Transaction history

#### Cache (Redis)
- Caches frequently accessed data
- Real-time updates
- Reduces blockchain queries

#### Services:
- **Indexer**: Listens to blockchain events
- **API Server**: Handles HTTP requests
- **Liquidation Bot**: Monitors and liquidates unhealthy loans

### 3. Frontend (Next.js/React)

#### Pages:
- **Dashboard**: Overview of user positions
- **Supply**: Add liquidity to pools
- **Borrow**: Create collateralized loans
- **Portfolio**: Manage active loans and supplies

#### Key Features:
- Wallet integration (Freighter)
- Real-time updates
- Transaction signing
- Pool statistics visualization

## Data Flow

### Supplying Liquidity

```
User → Frontend → Freighter Wallet → Smart Contract
                                          ↓
                                    Update Pool State
                                          ↓
Backend Indexer ← Blockchain Event ← Smart Contract
        ↓
   Update Database
        ↓
Frontend (via API) ← User sees confirmation
```

### Borrowing Flow

```
1. User connects wallet
2. Selects collateral and loan amount
3. Frontend validates collateral ratio
4. Transaction sent to smart contract
5. Contract validates and creates loan
6. Event emitted
7. Backend indexes loan
8. Frontend updates UI
```

### Liquidation Flow

```
1. Liquidation bot monitors loan health
2. Detects loan with health factor < 110%
3. Calls liquidate() on smart contract
4. Contract transfers collateral to liquidator
5. Loan marked as liquidated
6. Database updated
7. User notified
```

## Security Architecture

### Smart Contract Level
- Collateral checks before loan creation
- Health factor monitoring
- Authorization checks (require_auth)
- Integer overflow protection
- Reentrancy protection via Soroban design

### Backend Level
- Input validation
- Rate limiting
- SQL injection protection (parameterized queries)
- CORS configuration

### Frontend Level
- Wallet signature verification
- Transaction preview before signing
- Clear error messages
- Input sanitization

## Scalability Considerations

### Soroban Efficiency
- Optimized storage access patterns
- Minimal state reads/writes
- Gas-efficient calculations

### Backend Caching
- Redis for hot data
- Reduces blockchain load
- Improves response times

### Database Indexing
- Indexes on frequently queried fields
- Optimized for read-heavy operations

## Future Enhancements

1. **Oracle Integration**: Price feeds for accurate collateral valuation
2. **Flash Loans**: Uncollateralized loans within single transaction
3. **Governance**: Protocol parameters controlled by token holders
4. **Multi-collateral**: Support multiple assets as collateral
5. **Variable Rates**: Dynamic interest rates based on risk profiles
6. **Insurance Pool**: Protocol insurance against bad debt

## Development Workflow

```
Local Development
    ↓
Smart Contract Changes → cargo build → cargo test
    ↓
Deploy to Testnet → soroban contract deploy
    ↓
Backend Updates → docker-compose up
    ↓
Frontend Updates → npm run dev
    ↓
Integration Testing
    ↓
Testnet Verification
    ↓
Audit (if needed)
    ↓
Mainnet Deployment
```

## Monitoring & Observability

### Metrics to Track:
- Total Value Locked (TVL)
- Total Borrowed
- Number of active loans
- Average interest rates
- Liquidation frequency
- Protocol utilization rate

### Alerts:
- Failed transactions
- Liquidation opportunities
- System health checks
- API response times

## Deployment

### Smart Contracts
```bash
# Build optimized contract
cargo build --release --target wasm32-unknown-unknown

# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_lend.wasm \
  --network testnet

# Initialize contract
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  -- init \
  --admin ADMIN_ADDRESS
```

### Backend
```bash
# Using Docker
cd backend
docker-compose up -d

# Or manually
pip install -r requirements.txt
python src/main.py
```

### Frontend
```bash
cd frontend
npm install
npm run build
npm start
```

## Testing Strategy

### Unit Tests
- Smart contract functions
- Backend API endpoints
- Frontend components

### Integration Tests
- End-to-end user flows
- Contract + backend interaction
- Frontend + backend integration

### Security Tests
- Fuzzing smart contracts
- Penetration testing API
- Wallet integration testing

## Resources

- [Soroban Documentation](https://soroban.stellar.org)
- [Stellar SDK](https://stellar.github.io/js-stellar-sdk/)
- [FastAPI Docs](https://fastapi.tiangolo.com)
- [Next.js Docs](https://nextjs.org/docs)
