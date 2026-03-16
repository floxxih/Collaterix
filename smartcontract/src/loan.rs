use crate::error::Error;
use crate::pool::{calculate_interest, update_utilization_rate};
use crate::storage;
use crate::types::{Loan, LoanStatus};
use soroban_sdk::{Address, Env};

const MIN_COLLATERAL_RATIO: u32 = 120; // 120%
const LIQUIDATION_THRESHOLD: u32 = 110; // 110%

pub fn create_loan(
    env: &Env,
    borrower: &Address,
    lender: &Address,
    collateral_amount: i128,
    loan_amount: i128,
    interest_rate: u32,
    duration: u64,
) -> Result<u64, Error> {
    if loan_amount <= 0 || collateral_amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    if interest_rate > 10000 {
        return Err(Error::InvalidInterestRate);
    }

    let collateral_ratio = calculate_collateral_ratio(collateral_amount, loan_amount);
    if collateral_ratio < MIN_COLLATERAL_RATIO {
        return Err(Error::InsufficientCollateral);
    }

    let loan_id = storage::increment_loan_counter(env);
    let current_time = env.ledger().timestamp();

    let loan = Loan {
        borrower: borrower.clone(),
        lender: lender.clone(),
        collateral_amount,
        loan_amount,
        interest_rate,
        collateral_ratio,
        start_time: current_time,
        due_time: current_time + duration,
        repaid_amount: 0,
        status: LoanStatus::Active,
    };

    storage::write_loan(env, loan_id, &loan);
    Ok(loan_id)
}

pub fn repay_loan(env: &Env, loan_id: u64, amount: i128) -> Result<(), Error> {
    let mut loan = storage::read_loan(env, loan_id).ok_or(Error::LoanNotFound)?;

    if loan.status != LoanStatus::Active {
        return Err(Error::LoanAlreadyRepaid);
    }

    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    let duration = env.ledger().timestamp() - loan.start_time;
    let interest = calculate_interest(loan.loan_amount, loan.interest_rate, duration);
    let total_due = loan.loan_amount + interest;

    loan.repaid_amount += amount;

    if loan.repaid_amount >= total_due {
        loan.status = LoanStatus::Repaid;
    }

    storage::write_loan(env, loan_id, &loan);
    Ok(())
}

pub fn liquidate_loan(env: &Env, loan_id: u64, liquidator: &Address) -> Result<(), Error> {
    let mut loan = storage::read_loan(env, loan_id).ok_or(Error::LoanNotFound)?;

    if loan.status != LoanStatus::Active {
        return Err(Error::LiquidationNotAllowed);
    }

    let health_factor = calculate_health_factor(&loan);
    if health_factor >= LIQUIDATION_THRESHOLD {
        return Err(Error::CollateralBelowThreshold);
    }

    loan.status = LoanStatus::Liquidated;
    storage::write_loan(env, loan_id, &loan);

    Ok(())
}

pub fn calculate_collateral_ratio(collateral: i128, loan: i128) -> u32 {
    if loan == 0 {
        return 0;
    }
    ((collateral as u64 * 100) / loan as u64) as u32
}

pub fn calculate_health_factor(loan: &Loan) -> u32 {
    calculate_collateral_ratio(loan.collateral_amount, loan.loan_amount)
}

pub fn check_loan_health(env: &Env, loan_id: u64) -> Result<u32, Error> {
    let loan = storage::read_loan(env, loan_id).ok_or(Error::LoanNotFound)?;

    Ok(calculate_health_factor(&loan))
}
