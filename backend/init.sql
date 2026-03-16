-- StellarLend Database Schema

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    address VARCHAR(56) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS loans (
    id BIGINT PRIMARY KEY,
    borrower_address VARCHAR(56) NOT NULL,
    lender_address VARCHAR(56) NOT NULL,
    collateral_amount BIGINT NOT NULL,
    loan_amount BIGINT NOT NULL,
    interest_rate INTEGER NOT NULL,
    collateral_ratio INTEGER NOT NULL,
    start_time BIGINT NOT NULL,
    due_time BIGINT NOT NULL,
    repaid_amount BIGINT DEFAULT 0,
    status VARCHAR(20) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (borrower_address) REFERENCES users(address),
    FOREIGN KEY (lender_address) REFERENCES users(address)
);

CREATE TABLE IF NOT EXISTS lending_pools (
    asset_address VARCHAR(56) PRIMARY KEY,
    total_supplied BIGINT DEFAULT 0,
    total_borrowed BIGINT DEFAULT 0,
    base_interest_rate INTEGER NOT NULL,
    utilization_rate INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_positions (
    user_address VARCHAR(56) PRIMARY KEY,
    supplied BIGINT DEFAULT 0,
    borrowed BIGINT DEFAULT 0,
    collateral BIGINT DEFAULT 0,
    health_factor INTEGER DEFAULT 100,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_address) REFERENCES users(address)
);

CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(64) UNIQUE NOT NULL,
    user_address VARCHAR(56) NOT NULL,
    tx_type VARCHAR(20) NOT NULL,
    amount BIGINT,
    loan_id BIGINT,
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_address) REFERENCES users(address)
);

-- Indexes for performance
CREATE INDEX idx_loans_borrower ON loans(borrower_address);
CREATE INDEX idx_loans_lender ON loans(lender_address);
CREATE INDEX idx_loans_status ON loans(status);
CREATE INDEX idx_transactions_user ON transactions(user_address);
CREATE INDEX idx_transactions_type ON transactions(tx_type);
CREATE INDEX idx_transactions_timestamp ON transactions(timestamp);

-- Updated timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_loans_updated_at BEFORE UPDATE ON loans
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_pools_updated_at BEFORE UPDATE ON lending_pools
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_positions_updated_at BEFORE UPDATE ON user_positions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
