#![no_std]

mod error;
mod loan;
mod pool;
mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::error::Error;
use crate::types::{LendingPool, Loan, UserPosition};

#[contract]
pub struct Collaterix;

#[contractimpl]
impl Collaterix {
    /// Initialize the lending protocol
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }
        storage::write_admin(&env, &admin);
        Ok(())
    }

    /// Supply liquidity to a lending pool
    pub fn supply(env: Env, supplier: Address, asset: Address, amount: i128) -> Result<(), Error> {
        supplier.require_auth();
        pool::supply_liquidity(&env, &supplier, &asset, amount)
    }

    /// Withdraw liquidity from a lending pool
    pub fn withdraw(
        env: Env,
        supplier: Address,
        asset: Address,
        amount: i128,
    ) -> Result<(), Error> {
        supplier.require_auth();
        pool::withdraw_liquidity(&env, &supplier, &asset, amount)
    }

    /// Create a new collateralized loan
    pub fn borrow(
        env: Env,
        borrower: Address,
        lender: Address,
        collateral_amount: i128,
        loan_amount: i128,
        interest_rate: u32,
        duration: u64,
    ) -> Result<u64, Error> {
        borrower.require_auth();
        loan::create_loan(
            &env,
            &borrower,
            &lender,
            collateral_amount,
            loan_amount,
            interest_rate,
            duration,
        )
    }

    /// Repay a loan
    pub fn repay(env: Env, borrower: Address, loan_id: u64, amount: i128) -> Result<(), Error> {
        borrower.require_auth();
        loan::repay_loan(&env, loan_id, amount)
    }

    /// Liquidate an undercollateralized loan
    pub fn liquidate(env: Env, liquidator: Address, loan_id: u64) -> Result<(), Error> {
        liquidator.require_auth();
        loan::liquidate_loan(&env, loan_id, &liquidator)
    }

    /// Get loan details
    pub fn get_loan(env: Env, loan_id: u64) -> Result<Loan, Error> {
        storage::read_loan(&env, loan_id).ok_or(Error::LoanNotFound)
    }

    /// Get lending pool details
    pub fn get_pool(env: Env, asset: Address) -> Result<LendingPool, Error> {
        storage::read_pool(&env, &asset).ok_or(Error::PoolNotFound)
    }

    /// Get user position
    pub fn get_position(env: Env, user: Address) -> Result<UserPosition, Error> {
        storage::read_user_position(&env, &user).ok_or(Error::Unauthorized)
    }

    /// Check loan health factor
    pub fn check_health(env: Env, loan_id: u64) -> Result<u32, Error> {
        loan::check_loan_health(&env, loan_id)
    }

    /// Add supported asset (admin only)
    pub fn add_asset(env: Env, admin: Address, asset: Address) -> Result<(), Error> {
        admin.require_auth();
        let stored_admin = storage::read_admin(&env);
        if admin != stored_admin {
            return Err(Error::Unauthorized);
        }
        storage::add_supported_asset(&env, &asset);
        Ok(())
    }
}

#[cfg(test)]
mod test;
