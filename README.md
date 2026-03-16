# Collaterix

> **Decentralized lending protocol on Stellar with collateralized loans**

```
   ____      _ _       _            _      
  / ___|___ | | | __ _| |_ ___ _ __(_)_  __
 | |   / _ \| | |/ _` | __/ _ \ '__| \ \/ /
 | |__| (_) | | | (_| | ||  __/ |  | |>  < 
  \____\___/|_|_|\__,_|\__\___|_|  |_/_/\_\
                                           
```

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Built on Stellar](https://img.shields.io/badge/Built%20on-Stellar%20Soroban-purple)](https://soroban.stellar.org)
[![CI/CD](https://img.shields.io/github/actions/workflow/status/floxxih/Collaterix/ci.yml?branch=master&label=CI%2FCD)](https://github.com/floxxih/Collaterix/actions)
[![Coverage](https://img.shields.io/codecov/c/github/floxxih/Collaterix?label=Coverage)](https://codecov.io/gh/floxxih/Collaterix)
[![Version](https://img.shields.io/github/v/release/floxxih/Collaterix?label=Version)](https://github.com/floxxih/Collaterix/releases)
[![Stellar Network](https://img.shields.io/badge/Network-Stellar%20Testnet-brightgreen)](https://stellar.org)

## The Problem

Traditional lending requires trust and intermediaries:
- **Centralized Risk**: Banks and platforms control access to credit
- **Credit Barriers**: Traditional credit systems exclude many people
- **High Fees**: Intermediaries charge significant fees
- **Lack of Transparency**: Opaque terms and conditions

## The Solution

Collaterix is a **decentralized lending protocol** where:
- Loans are **secured by collateral** in Soroban smart contracts
- **Permissionless**: Anyone can lend or borrow
- **Transparent**: All terms encoded on-chain
- **Liquidation protection** for lenders through overcollateralization
- **Competitive rates** determined by supply and demand

---

## Features

| Feature | Description |
|---------|-------------|
| **Collateralized Loans** | Borrow against crypto assets with 120%+ collateral ratio |
| **Liquidity Pools** | Supply assets to earn interest from borrowers |
| **Dynamic Interest Rates** | Rates adjust based on pool utilization |
| **Liquidation Engine** | Automated liquidation of undercollateralized loans |
| **Health Factor Monitoring** | Real-time tracking of loan health |
| **Multi-Asset Support** | Support for various Stellar assets |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      FRONTEND (Next.js)                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │  Supply  │  │  Borrow  │  │  Repay   │  │Dashboard │       │
│  │  Liquidity│  │  Assets  │  │  Loan    │  │          │       │
│  └────┬─────┘  └─────┬────┘  └─────┬────┘  └────┬─────┘       │
└───────┼──────────────┼─────────────┼────────────┼──────────────┘
        │              │             │            │
        ▼              ▼             ▼            ▼
┌─────────────────────────────────────────────────────────────────┐
│                   SOROBAN SMART CONTRACTS                        │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                  Collaterix Protocol                       │   │
│  │  • init()          • supply()        • withdraw()        │   │
│  │  • borrow()        • repay()         • liquidate()       │   │
│  │  • get_loan()      • get_pool()      • check_health()    │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                   Lending Pools                           │   │
│  │  • Track liquidity    • Calculate interest               │   │
│  │  • Manage utilization • Update rates                     │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│                      BACKEND SERVICES                            │
│  ┌────────────┐  ┌─────────────┐  ┌──────────────┐             │
│  │  Indexer   │  │   REST API  │  │ Liquidation  │             │
│  │ (Events)   │  │  (FastAPI)  │  │     Bot      │             │
│  └─────┬──────┘  └──────┬──────┘  └──────┬───────┘             │
│        ▼                ▼                ▼                      │
│  ┌──────────────────────────────────────────────────────┐      │
│  │            PostgreSQL + Redis Cache                  │      │
│  └──────────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────────┘
```

---

## How It Works

### Supplying Liquidity
1. Connect your wallet (Freighter)
2. Choose an asset and amount to supply
3. Earn interest based on utilization rate
4. Withdraw anytime if liquidity available

### Borrowing
1. Deposit collateral (min 120% of loan value)
2. Specify loan amount and duration
3. Pay interest rate based on pool utilization
4. Maintain health factor above 110% to avoid liquidation

### Liquidations
- Loans become liquidatable when health factor < 110%
- Liquidators can repay debt and claim collateral
- Incentivizes maintaining healthy collateral ratios

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| **Blockchain** | Stellar Soroban (Rust) |
| **Frontend** | Next.js 14, TypeScript, Tailwind CSS |
| **Wallet** | Freighter Wallet Integration |
| **Backend** | FastAPI (Python), PostgreSQL, Redis |
| **Infrastructure** | Docker, Docker Compose |

---

## Getting Started

### Prerequisites

- Node.js v18+
- Rust & Cargo (for contracts)
- Freighter Wallet Extension
- Soroban CLI
- Docker & Docker Compose

### Installation

```bash
# Clone the repository
git clone https://github.com/floxxih/Collaterix.git
cd Collaterix

# Setup Smart Contracts
cd smartcontract
cargo build --release --target wasm32-unknown-unknown

# Deploy to testnet
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/collaterix.wasm --network testnet

# Setup Frontend
cd ../frontend
npm install
npm run dev

# Setup Backend (with Docker)
cd ../backend
docker-compose up -d
```

---

## Key Concepts

### Health Factor
```
Health Factor = (Collateral Value / Borrowed Value) * 100
```
- Above 120%: Healthy loan
- 110-120%: At risk
- Below 110%: Liquidatable

### Interest Rate
Interest accrues based on:
- Base rate: 5%
- Utilization multiplier: Higher utilization = higher rates
- Calculated per second for precision

### Collateral Ratio
```
Collateral Ratio = (Collateral Amount / Loan Amount) * 100
```
- Minimum: 120%
- Recommended: 150%+
- Liquidation threshold: 110%

---

## Documentation

- [Architecture Guide](./docs/ARCHITECTURE.md)
- [Smart Contract Documentation](./docs/SMARTCONTRACT.md)
- [API Reference](./docs/API.md)
- [Frontend Guide](./docs/FRONTEND.md)

---

## Security Considerations

- Smart contracts should be audited before mainnet deployment
- Collateral ratios protect lenders from volatility
- Liquidation mechanisms ensure protocol solvency
- Oracle integration needed for accurate price feeds (future)

---

## Roadmap

- [x] Core lending/borrowing functionality
- [x] Liquidation engine
- [x] Basic frontend interface
- [ ] Price oracle integration
- [ ] Flash loans
- [ ] Governance token
- [ ] Multi-collateral loans
- [ ] Mobile app

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

**Quick Start for Contributors:**
1. Pick an issue labeled `good-first-issue`
2. Fork the repo
3. Create a feature branch
4. Submit a PR

---

## License

MIT License - see [LICENSE](./LICENSE) for details.

---

## Disclaimer

This is experimental software. Use at your own risk. Not financial advice.

---

*Built with care for the Stellar ecosystem*
