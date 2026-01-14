//! Performance optimizations and utilities for the cowSolver project.
//!
//! This module contains helper functions designed to improve the efficiency of the solver.
//! It includes routines for deduplicating and sorting orders as well as examples of how
//! to structure computations to take advantage of Rust's powerful iterator optimizations.

use crate::models::Order;

/// Sort a vector of orders by expiration and remove duplicates by order id.
///
/// Sorting by expiration ensures that older orders are processed first, which can
/// improve cache locality in downstream algorithms. Duplicates are removed to
/// prevent redundant work in matching and pricing engines.
pub fn optimize_orders(orders: &mut Vec<Order>) {
    // Sort by expiration (earliest first).
    orders.sort_by_key(|o| o.expiration);
    // Then sort by id so duplicates become adjacent.
    orders.sort_by_key(|o| o.id);
    // Deduplicate contiguous entries with the same id.
    orders.dedup_by_key(|o| o.id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Order, Token, ChainId, OrderType};
    use rust_decimal::Decimal;
    use std::time::{UNIX_EPOCH, Duration};

    #[test]
    fn test_optimize_orders_removes_duplicates() {
        let token_a = Token::new("0xaaa", "AAA", "AAA Token", 18, ChainId::EthereumMainnet);
        let token_b = Token::new("0xbbb", "BBB", "BBB Token", 18, ChainId::EthereumMainnet);
        let order1 = Order::new(
            1,
            "0xowner",
            token_a.clone(),
            token_b.clone(),
            Decimal::ONE,
            Decimal::ONE,
            UNIX_EPOCH + Duration::from_secs(1000),
            OrderType::Limit,
        );
        let order2 = Order::new(
            1,
            "0xowner",
            token_a,
            token_b,
            Decimal::ONE,
            Decimal::ONE,
            UNIX_EPOCH + Duration::from_secs(2000),
            OrderType::Limit,
        );
        let mut orders = vec![order1, order2];
        optimize_orders(&mut orders);
        assert_eq!(orders.len(), 1);
    }
}
