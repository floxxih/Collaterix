# StellarLend Style Guide

## General Principles

- Write clear, readable code
- Favor simplicity over cleverness
- Document complex logic
- Keep functions focused and small
- Write tests for new functionality

## Rust (Smart Contracts)

### Formatting
```rust
// Use cargo fmt for automatic formatting
cargo fmt

// Run clippy for linting
cargo clippy
```

### Naming Conventions
- Functions: `snake_case`
- Structs/Enums: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`

### Error Handling
```rust
// Use Result types
pub fn create_loan(...) -> Result<u64, Error> {
    // Check conditions
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    // ...
}
```

### Documentation
```rust
/// Creates a new collateralized loan
/// 
/// # Arguments
/// * `borrower` - Address of the borrower
/// * `loan_amount` - Amount to borrow
/// 
/// # Returns
/// * `Result<u64, Error>` - Loan ID or error
pub fn create_loan(...) -> Result<u64, Error> {
    // implementation
}
```

## TypeScript/JavaScript (Frontend)

### Formatting
- Use Prettier for consistent formatting
- 2 spaces for indentation
- Semicolons required

### Components
```typescript
// Use functional components with TypeScript
interface LoanCardProps {
  loan: Loan;
  onRepay: (loanId: number) => void;
}

export const LoanCard: React.FC<LoanCardProps> = ({ loan, onRepay }) => {
  // implementation
};
```

### File Organization
```
components/
  LoanCard/
    LoanCard.tsx
    LoanCard.test.tsx
    LoanCard.module.css
```

## Python (Backend)

### Formatting
```python
# Use Black for formatting
black src/

# Use isort for imports
isort src/
```

### Type Hints
```python
from typing import List, Optional

def get_loans(
    status: Optional[str] = None,
    limit: int = 50
) -> List[LoanResponse]:
    """Retrieve loans with optional filters."""
    # implementation
```

### Docstrings
```python
def calculate_interest(principal: int, rate: int, duration: int) -> int:
    """
    Calculate interest accrued on a loan.
    
    Args:
        principal: Initial loan amount
        rate: Interest rate in basis points
        duration: Loan duration in seconds
    
    Returns:
        Total interest amount
    """
    # implementation
```

## Git Commits

### Format
```
type(scope): subject

body (optional)

footer (optional)
```

### Examples
```
feat(contracts): add flash loan support
fix(api): correct health factor calculation
docs(readme): update deployment instructions
refactor(pool): optimize interest calculation
test(loan): add liquidation test cases
```

## Testing

### Smart Contracts
```rust
#[test]
fn test_create_loan_with_valid_collateral() {
    // Arrange
    let env = Env::default();
    // ...
    
    // Act
    let result = contract.borrow(...);
    
    // Assert
    assert!(result.is_ok());
}
```

### Frontend
```typescript
describe('LoanCard', () => {
  it('renders loan details correctly', () => {
    // Test implementation
  });
});
```

### Backend
```python
def test_create_loan():
    """Test loan creation with valid parameters."""
    # Arrange
    loan_data = {...}
    
    # Act
    response = client.post("/api/loans", json=loan_data)
    
    # Assert
    assert response.status_code == 200
```

## Code Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No unnecessary complexity
- [ ] Error handling included
- [ ] Security considerations addressed
- [ ] Performance implications considered
