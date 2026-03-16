use crate::types::{LendingPool, Loan, UserPosition};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Loan(u64),
    LoanCounter,
    Pool(Address),
    UserPosition(Address),
    SupportedAsset(Address),
}

// Admin Functions
pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn read_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

// Loan Functions
pub fn read_loan(env: &Env, loan_id: u64) -> Option<Loan> {
    env.storage().persistent().get(&DataKey::Loan(loan_id))
}

pub fn write_loan(env: &Env, loan_id: u64, loan: &Loan) {
    env.storage()
        .persistent()
        .set(&DataKey::Loan(loan_id), loan);
}

pub fn get_loan_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::LoanCounter)
        .unwrap_or(0)
}

pub fn increment_loan_counter(env: &Env) -> u64 {
    let counter = get_loan_counter(env) + 1;
    env.storage()
        .instance()
        .set(&DataKey::LoanCounter, &counter);
    counter
}

// Pool Functions
pub fn read_pool(env: &Env, asset: &Address) -> Option<LendingPool> {
    env.storage()
        .persistent()
        .get(&DataKey::Pool(asset.clone()))
}

pub fn write_pool(env: &Env, asset: &Address, pool: &LendingPool) {
    env.storage()
        .persistent()
        .set(&DataKey::Pool(asset.clone()), pool);
}

// User Position Functions
pub fn read_user_position(env: &Env, user: &Address) -> Option<UserPosition> {
    env.storage()
        .persistent()
        .get(&DataKey::UserPosition(user.clone()))
}

pub fn write_user_position(env: &Env, user: &Address, position: &UserPosition) {
    env.storage()
        .persistent()
        .set(&DataKey::UserPosition(user.clone()), position);
}

// Supported Asset Functions
pub fn is_asset_supported(env: &Env, asset: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::SupportedAsset(asset.clone()))
}

pub fn add_supported_asset(env: &Env, asset: &Address) {
    env.storage()
        .persistent()
        .set(&DataKey::SupportedAsset(asset.clone()), &true);
}
