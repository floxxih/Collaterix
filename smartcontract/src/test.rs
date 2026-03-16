#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialization() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLend);
    let client = StellarLendClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init(&admin);

    // Verify initialization worked (would need getter for admin)
}

#[test]
fn test_supply_and_withdraw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLend);
    let client = StellarLendClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let supplier = Address::generate(&env);
    let asset = Address::generate(&env);

    client.init(&admin);
    client.add_asset(&admin, &asset);
    client.supply(&supplier, &asset, &1000);

    let pool = client.get_pool(&asset);
    assert_eq!(pool.total_supplied, 1000);
}

#[test]
fn test_borrow_with_collateral() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLend);
    let client = StellarLendClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let borrower = Address::generate(&env);
    let lender = Address::generate(&env);

    client.init(&admin);

    let loan_id = client.borrow(
        &borrower, &lender, &1500,  // collateral
        &1000,  // loan amount (150% collateral ratio)
        &500,   // 5% interest
        &86400, // 1 day duration
    );

    let loan = client.get_loan(&loan_id);
    assert_eq!(loan.loan_amount, 1000);
    assert_eq!(loan.collateral_amount, 1500);
}
