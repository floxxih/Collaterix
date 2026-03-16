from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Optional, List
from enum import Enum

app = FastAPI(title="StellarLend API", version="0.1.0")

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Models
class LoanStatus(str, Enum):
    ACTIVE = "active"
    REPAID = "repaid"
    LIQUIDATED = "liquidated"
    DEFAULTED = "defaulted"

class LoanCreate(BaseModel):
    borrower_address: str
    lender_address: str
    collateral_amount: int
    loan_amount: int
    interest_rate: int
    duration: int

class LoanResponse(BaseModel):
    id: int
    borrower_address: str
    lender_address: str
    collateral_amount: int
    loan_amount: int
    interest_rate: int
    collateral_ratio: int
    start_time: int
    due_time: int
    repaid_amount: int
    status: LoanStatus
    health_factor: int

class PoolSupply(BaseModel):
    supplier_address: str
    asset_address: str
    amount: int

class PoolResponse(BaseModel):
    asset_address: str
    total_supplied: int
    total_borrowed: int
    base_interest_rate: int
    utilization_rate: int
    apy: float

class UserPositionResponse(BaseModel):
    user_address: str
    supplied: int
    borrowed: int
    collateral: int
    health_factor: int

# Routes
@app.get("/")
async def root():
    return {
        "name": "StellarLend API",
        "version": "0.1.0",
        "description": "Decentralized lending protocol on Stellar"
    }

@app.post("/api/loans", response_model=LoanResponse)
async def create_loan(loan: LoanCreate):
    """Create a new collateralized loan"""
    # TODO: Integrate with Stellar smart contract
    raise HTTPException(status_code=501, detail="Not implemented")

@app.get("/api/loans/{loan_id}", response_model=LoanResponse)
async def get_loan(loan_id: int):
    """Get loan details by ID"""
    # TODO: Query from database/blockchain
    raise HTTPException(status_code=404, detail="Loan not found")

@app.get("/api/loans", response_model=List[LoanResponse])
async def list_loans(
    status: Optional[LoanStatus] = None,
    borrower: Optional[str] = None,
    limit: int = 50,
    offset: int = 0
):
    """List loans with optional filters"""
    # TODO: Query from database
    return []

@app.post("/api/pools/supply")
async def supply_liquidity(supply: PoolSupply):
    """Supply liquidity to a lending pool"""
    # TODO: Integrate with Stellar smart contract
    raise HTTPException(status_code=501, detail="Not implemented")

@app.post("/api/pools/withdraw")
async def withdraw_liquidity(withdraw: PoolSupply):
    """Withdraw liquidity from a lending pool"""
    # TODO: Integrate with Stellar smart contract
    raise HTTPException(status_code=501, detail="Not implemented")

@app.get("/api/pools/{asset_address}", response_model=PoolResponse)
async def get_pool(asset_address: str):
    """Get lending pool details"""
    # TODO: Query from database/blockchain
    raise HTTPException(status_code=404, detail="Pool not found")

@app.get("/api/pools", response_model=List[PoolResponse])
async def list_pools():
    """List all lending pools"""
    # TODO: Query from database
    return []

@app.get("/api/positions/{user_address}", response_model=UserPositionResponse)
async def get_user_position(user_address: str):
    """Get user's lending/borrowing position"""
    # TODO: Query from database/blockchain
    raise HTTPException(status_code=404, detail="Position not found")

@app.post("/api/loans/{loan_id}/repay")
async def repay_loan(loan_id: int, amount: int):
    """Repay a loan"""
    # TODO: Integrate with Stellar smart contract
    raise HTTPException(status_code=501, detail="Not implemented")

@app.post("/api/loans/{loan_id}/liquidate")
async def liquidate_loan(loan_id: int, liquidator_address: str):
    """Liquidate an undercollateralized loan"""
    # TODO: Integrate with Stellar smart contract
    raise HTTPException(status_code=501, detail="Not implemented")

@app.get("/api/health")
async def health_check():
    """API health check"""
    return {"status": "healthy"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
