use super::{Match, OrderMatchingStrategy};
use crate::models::{Order};

/// A matcher that finds circular rings of three orders where the sell token of each order
/// matches the buy token of the next order, forming a closed loop.
pub struct RingMatcher;

impl OrderMatchingStrategy for RingMatcher {
    fn match_orders(&self, orders: &[Order]) -> Vec<Match> {
        let mut matches = Vec::new();
        let n = orders.len();
        if n < 3 {
            return matches;
        }
        for i in 0..n {
            for j in 0..n {
                if j == i { continue; }
                for k in 0..n {
                    if k == i || k == j { continue; }
                    let o1 = &orders[i];
                    let o2 = &orders[j];
                    let o3 = &orders[k];
                    if o1.sell_token.address == o2.buy_token.address
                        && o2.sell_token.address == o3.buy_token.address
                        && o3.sell_token.address == o1.buy_token.address
                    {
                        let orders_vec = vec![o1.clone(), o2.clone(), o3.clone()];
                        matches.push(Match { orders: orders_vec });
                    }
                }
            }
        }
        use std::collections::HashSet;
        let mut unique = HashSet::new();
        matches.retain(|m| {
            let mut ids: Vec<u64> = m.orders.iter().map(|o| o.id).collect();
            ids.sort();
            if unique.contains(&ids) {
                false
            } else {
                unique.insert(ids);
                true
            }
        });
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Order, Token, OrderType, ChainId};
    use rust_decimal::prelude::*;

    fn dummy_order(id: u64, owner: &str, sell: &str, buy: &str) -> Order {
        Order::new(
            id,
            owner.to_string(),
            Token::new(
                sell.to_string(),
                sell.to_string(),
                sell.to_string(),
                18,
                ChainId::EthereumMainnet,
            ),
            Token::new(
                buy.to_string(),
                buy.to_string(),
                buy.to_string(),
                18,
                ChainId::EthereumMainnet,
            ),
            Decimal::from_i64(100).unwrap(),
            Decimal::from_i64(50).unwrap(),
            None,
            OrderType::Market,
        )
    }

    #[test]
    fn test_ring_matcher_three_orders() {
        let order1 = dummy_order(1, "Alice", "A", "B");
        let order2 = dummy_order(2, "Bob", "B", "C");
        let order3 = dummy_order(3, "Carol", "C", "A");
        let matcher = RingMatcher;
        let matches = matcher.match_orders(&[order1.clone(), order2.clone(), order3.clone()]);
        assert_eq!(matches.len(), 1);
        let m = &matches[0];
        assert_eq!(m.orders.len(), 3);
    }

    #[test]
    fn test_ring_matcher_no_ring() {
        let order1 = dummy_order(1, "Alice", "A", "B");
        let order2 = dummy_order(2, "Bob", "C", "D");
        let order3 = dummy_order(3, "Carol", "E", "F");
        let matcher = RingMatcher;
        let matches = matcher.match_orders(&[order1, order2, order3]);
        assert!(matches.is_empty());
    }
}
