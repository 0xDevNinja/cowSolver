use crate::models::{Order, Settlement};

/// Trait defining a pluggable solver strategy.
///
/// Strategies implement this trait to produce a settlement from a set of orders. Different
/// strategies may employ various algorithms to match orders, route through AMMs, or adjust
/// pricing and slippage parameters.
pub trait Strategy {
    /// Compute a settlement for a batch of orders.
    ///
    /// Returns `Some(Settlement)` if a valid settlement is found, or `None` if the strategy
    /// cannot produce a settlement for the given orders.
    fn solve(&self, orders: &[Order]) -> Option<Settlement>;
}

/// Baseline strategy implementations.
pub mod baseline;

/// Advanced strategy implementations.
pub mod advanced;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OrderType, Token, ChainId};
    use rust_decimal::Decimal;
    use std::time::{UNIX_EPOCH, Duration};

    /// Dummy strategy used to test the trait interface.
    struct DummyStrategy;

    impl Strategy for DummyStrategy {
        fn solve(&self, _orders: &[Order]) -> Option<Settlement> {
            None
        }
    }

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
    fn test_dummy_strategy_returns_none() {
        let strategy = DummyStrategy;
        let orders = vec![dummy_order(1), dummy_order(2)];
        let result = strategy.solve(&orders);
        assert!(result.is_none());
    }
}
