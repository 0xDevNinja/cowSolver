use crate::matching::{Match, OrderMatchingStrategy};
use crate::models::{Order};

/// A simple pair matcher that scans a list of orders and returns matches for
/// complementary buy/sell token pairs. This matcher is intended as an example
/// implementation of the `OrderMatchingStrategy` trait. It does not perform
/// any economic analysis—orders are matched solely on token pairs and order
/// types and assumes that all orders are of the same size for simplicity.
pub struct PairMatcher;

impl OrderMatchingStrategy for PairMatcher {
    /// Iterate through the provided slice of orders, finding pairs of orders
    /// where the sell token of one matches the buy token of the other and
    /// vice‑versa. Each match contains exactly two orders. If there are no
    /// complementary orders, an empty vector is returned.
    fn match_orders(&self, orders: &[Order]) -> Vec<Match> {
        let mut matches = Vec::new();
        for i in 0..orders.len() {
            for j in (i + 1)..orders.len() {
                let o1 = &orders[i];
                let o2 = &orders[j];
                if o1.sell_token == o2.buy_token && o1.buy_token == o2.sell_token {
                    matches.push(Match {
                        orders: vec![o1.clone(), o2.clone()],
                    });
                }
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::PairMatcher;
    use crate::matching::{Match, OrderMatchingStrategy};
    use crate::models::{Order, OrderType, Token, ChainId};
    use rust_decimal::prelude::*;
    use std::time::{SystemTime, Duration};

    /// Helper to construct a dummy order for testing.
    fn dummy_order(
        id: u64,
        owner: &str,
        sell_token: &str,
        buy_token: &str,
        sell_amount: i64,
        buy_amount: i64,
    ) -> Order {
        Order::new(
            id,
            owner.to_string(),
            Token::new(sell_token.into(), "SELL".into(), 18, ChainId::EthereumMainnet),
            Token::new(buy_token.into(), "BUY".into(), 18, ChainId::EthereumMainnet),
            Decimal::from_i64(sell_amount).unwrap(),
            Decimal::from_i64(buy_amount).unwrap(),
            SystemTime::now() + Duration::from_secs(300),
            OrderType::Market,
        )
    }

    #[test]
    fn test_pair_matcher_matches_two_orders() {
        let order1 = dummy_order(1, "Alice", "0xAAA", "0xBBB", 100, 50);
        let order2 = dummy_order(2, "Bob", "0xBBB", "0xAAA", 50, 100);
        let matcher = PairMatcher;
        let matches = matcher.match_orders(&[order1.clone(), order2.clone()]);
        assert_eq!(matches.len(), 1);
        let m = &matches[0];
        assert_eq!(m.orders.len(), 2);
        // Ensure the match contains both orders
        assert!(m.orders.iter().any(|o| o.id == order1.id));
        assert!(m.orders.iter().any(|o| o.id == order2.id));
    }

    #[test]
    fn test_pair_matcher_no_match() {
        let order1 = dummy_order(1, "Alice", "0xAAA", "0xBBB", 100, 50);
        let order2 = dummy_order(2, "Charlie", "0xCCC", "0xDDD", 75, 75);
        let matcher = PairMatcher;
        let matches = matcher.match_orders(&[order1, order2]);
        assert!(matches.is_empty());
    }
}
