use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InsufficientCollateral = 5,
    InsufficientLiquidity = 6,
    LoanNotFound = 7,
    LoanAlreadyRepaid = 8,
    LoanNotDue = 9,
    CollateralBelowThreshold = 10,
    InvalidInterestRate = 11,
    InvalidCollateralRatio = 12,
    LiquidationNotAllowed = 13,
    PoolNotFound = 14,
    AssetNotSupported = 15,
}
