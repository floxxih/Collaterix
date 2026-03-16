use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoanStatus {
    Active,
    Repaid,
    Liquidated,
    Defaulted,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loan {
    pub borrower: Address,
    pub lender: Address,
    pub collateral_amount: i128,
    pub loan_amount: i128,
    pub interest_rate: u32,    // basis points (e.g., 500 = 5%)
    pub collateral_ratio: u32, // percentage (e.g., 150 = 150%)
    pub start_time: u64,
    pub due_time: u64,
    pub repaid_amount: i128,
    pub status: LoanStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LendingPool {
    pub asset: Address,
    pub total_supplied: i128,
    pub total_borrowed: i128,
    pub base_interest_rate: u32, // basis points
    pub utilization_rate: u32,   // percentage
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPosition {
    pub user: Address,
    pub supplied: i128,
    pub borrowed: i128,
    pub collateral: i128,
    pub health_factor: u32, // percentage (below 100 = liquidatable)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterestAccrual {
    pub principal: i128,
    pub accrued_interest: i128,
    pub last_update: u64,
}
