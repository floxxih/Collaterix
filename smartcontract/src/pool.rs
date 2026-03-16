use crate::error::Error;
use crate::storage;
use crate::types::{LendingPool, UserPosition};
use soroban_sdk::{Address, Env};

const BASIS_POINTS: u32 = 10000;
const SECONDS_PER_YEAR: u64 = 31536000;

pub fn supply_liquidity(
    env: &Env,
    supplier: &Address,
    asset: &Address,
    amount: i128,
) -> Result<(), Error> {
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    if !storage::is_asset_supported(env, asset) {
        return Err(Error::AssetNotSupported);
    }

    let mut pool = storage::read_pool(env, asset).unwrap_or_else(|| create_default_pool(asset));

    pool.total_supplied += amount;
    update_utilization_rate(&mut pool);
    storage::write_pool(env, asset, &pool);

    // Update user position
    let mut position = storage::read_user_position(env, supplier)
        .unwrap_or_else(|| create_default_position(supplier));

    position.supplied += amount;
    storage::write_user_position(env, supplier, &position);

    Ok(())
}

pub fn withdraw_liquidity(
    env: &Env,
    supplier: &Address,
    asset: &Address,
    amount: i128,
) -> Result<(), Error> {
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    let mut pool = storage::read_pool(env, asset).ok_or(Error::PoolNotFound)?;

    let available = pool.total_supplied - pool.total_borrowed;
    if amount > available {
        return Err(Error::InsufficientLiquidity);
    }

    let mut position = storage::read_user_position(env, supplier).ok_or(Error::Unauthorized)?;

    if amount > position.supplied {
        return Err(Error::InvalidAmount);
    }

    pool.total_supplied -= amount;
    position.supplied -= amount;

    update_utilization_rate(&mut pool);
    storage::write_pool(env, asset, &pool);
    storage::write_user_position(env, supplier, &position);

    Ok(())
}

pub fn calculate_interest(principal: i128, interest_rate: u32, duration_seconds: u64) -> i128 {
    let rate_per_second =
        (interest_rate as i128) * (principal / BASIS_POINTS as i128) / SECONDS_PER_YEAR as i128;
    rate_per_second * duration_seconds as i128
}

pub fn update_utilization_rate(pool: &mut LendingPool) {
    if pool.total_supplied == 0 {
        pool.utilization_rate = 0;
    } else {
        pool.utilization_rate =
            ((pool.total_borrowed as u64 * 100) / pool.total_supplied as u64) as u32;
    }
}

fn create_default_pool(asset: &Address) -> LendingPool {
    LendingPool {
        asset: asset.clone(),
        total_supplied: 0,
        total_borrowed: 0,
        base_interest_rate: 500, // 5%
        utilization_rate: 0,
    }
}

fn create_default_position(user: &Address) -> UserPosition {
    UserPosition {
        user: user.clone(),
        supplied: 0,
        borrowed: 0,
        collateral: 0,
        health_factor: 100,
    }
}
