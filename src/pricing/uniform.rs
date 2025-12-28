use super::PricingEngine;
use crate::models::{Order, Token, ChainId, OrderType};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::time::{UNIX_EPOCH, Duration};

/// A simple pricing engine that computes a uniform clearing price by averaging
/// sell/buy amounts across all orders. This is a baseline strategy and may not
/// produce optimal prices in all markets.
#[derive(Debug, Default)]
pub struct UniformPricingEngine;

impl PricingEngine for UniformPricingEngine {
    fn compute_clearing_price(&self, orders: &[Order]) -> Option<Decimal> {
        // At least two orders are required to compute a clearing price.
        if orders.len() < 2 {
            return None;
        }
        let mut total_sell = Decimal::ZERO;
        let mut total_buy = Decimal::ZERO;
        for order in orders {
            total_sell += order.sell_amount;
            total_buy += order.buy_amount;
        }
        if total_buy.is_zero() {
            None
        } else {
            Some(total_sell / total_buy)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;
    use rust_decimal::prelude::*;

    fn dummy_order(id: u64, sell: i64, buy: i64) -> Order {
        let sell_token = Token::new(
            "0xaaa",
            "AAA",
            "AAA Token",
            18,
            ChainId::EthereumMainnet,
        );
        let buy_token = Token::new(
            "0xbbb",
            "BBB",
            "BBB Token",
            18,
            ChainId::EthereumMainnet,
        );
        Order::new(
            id,
            "0xowner",
            sell_token,
            buy_token,
            Decimal::from_i64(sell).unwrap(),
            Decimal::from_i64(buy).unwrap(),
            UNIX_EPOCH + Duration::from_secs(1000),
            OrderType::Limit,
        )
    }

    #[test]
    fn test_uniform_no_orders() {
        let engine = UniformPricingEngine::default();
        assert!(engine.compute_clearing_price(&[]).is_none());
    }

    #[test]
    fn test_uniform_single_order() {
        let engine = UniformPricingEngine::default();
        let order = dummy_order(1, 10, 20);
        assert!(engine.compute_clearing_price(&[order]).is_none());
    }

    #[test]
    fn test_uniform_multiple_orders() {
        let engine = UniformPricingEngine::default();
        let orders = [dummy_order(1, 10, 10), dummy_order(2, 20, 10)];
        let price = engine.compute_clearing_price(&orders);
        assert_eq!(price, Some(Decimal::from_i64(3).unwrap()));
    }
}
