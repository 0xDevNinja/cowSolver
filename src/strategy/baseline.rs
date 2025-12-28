use super::Strategy;
use crate::models::{Order, Settlement, OrderType, Token, ChainId};
use rust_decimal::Decimal;

/// Baseline strategy implementation.
///
/// This simple strategy iterates through orders and attempts to match them using
/// straightforward heuristics (e.g., direct pair matching) without exploring
/// advanced optimization techniques. It returns `None` for now as a stub.
#[derive(Default)]
pub struct BaselineStrategy;

impl Strategy for BaselineStrategy {
    fn solve(&self, _orders: &[Order]) -> Option<Settlement> {
        // TODO: implement baseline solving algorithm (e.g., pair matching and uniform pricing)
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{UNIX_EPOCH, Duration};

    fn dummy_order(id: u64) -> Order {
        let sell_token = Token::new("0xaaa", "AAA", "AAA Token", 18, ChainId::EthereumMainnet);
        let buy_token = Token::new("0xbbb", "BBB", "BBB Token", 18, ChainId::EthereumMainnet);
        Order::new(
            id,
            "0xowner",
            sell_token,
            buy_token,
            Decimal::ONE,
            Decimal::ONE,
            UNIX_EPOCH + Duration::from_secs(1000),
            OrderType::Limit,
        )
    }

    #[test]
    fn test_baseline_strategy_returns_none() {
        let strategy = BaselineStrategy::default();
        let orders = vec![dummy_order(1), dummy_order(2)];
        let result = strategy.solve(&orders);
        assert!(result.is_none());
    }
}
