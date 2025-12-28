use rust_decimal::Decimal;
use rust_decimal::prelude::*;

/// Normalizes an integer token amount (e.g. in smallest units) to a Decimal value
/// using the token's number of decimals.
pub fn normalize_amount(amount: u128, decimals: u32) -> Decimal {
    let factor = Decimal::from(1u64).checked_div(Decimal::from(10u64.pow(decimals))).unwrap();
    Decimal::from(amount) * factor
}

/// Multiplies two decimals safely.
pub fn mul(a: Decimal, b: Decimal) -> Decimal {
    a * b
}

/// Computes slippage given expected and actual amounts.
/// Returns None if expected is zero to avoid division by zero.
pub fn slippage(expected: Decimal, actual: Decimal) -> Option<Decimal> {
    if expected.is_zero() {
        None
    } else {
        Some((expected - actual) / expected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Decimal::from_i64(2).unwrap();
        let b = Decimal::from_i64(3).unwrap();
        assert_eq!(mul(a, b), Decimal::from_i64(6).unwrap());
    }

    #[test]
    fn test_slippage() {
        let expected = Decimal::from_i64(100).unwrap();
        let actual = Decimal::from_i64(90).unwrap();
        let slip = slippage(expected, actual).unwrap();
        assert_eq!(slip, Decimal::new(1, 1));
    }
}
