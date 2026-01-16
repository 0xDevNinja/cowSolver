use super::PricingEngine;
use crate::models::{Order, OrderType, Token, ChainId};
use rust_decimal::Decimal;

/// Pricing engine that computes a clearing price as the volume-weighted average
/// of individual order prices. Each order's price is calculated as buy_amount
/// divided by sell_amount and weighted by the sell volume.
#[derive(Debug, Default)]
pub struct VolumeWeightedPricingEngine;

impl PricingEngine for VolumeWeightedPricingEngine {
    fn compute_clearing_price(orders: &[Order]) -> Option<Decimal> {
        // Require at least two orders to determine a clearing price
        if orders.len() < 2 {
            return None;
        }
        let mut weighted_sum = Decimal::ZERO;
        let mut total_sell = Decimal::ZERO;
        for order in orders {
            let sell = order.sell_amount;
            let buy = order.buy_amount;
            // Skip orders with zero sell amount to avoid division by zero
            if sell.is_zero() {
                continue;
            }
            let price = buy / sell;
            weighted_sum += price * sell;
            total_sell += sell;
        }
        if total_sell.is_zero() {
            return None;
        }
        Some(weighted_sum / total_sell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::*;
    use std::time::{UNIX_EPOCH, Duration};

    // Helper to create a dummy order with given sell and buy amounts
    fn dummy_order(id: u64, sell: i64, buy: i64) -> Order {
        let sell_token = Token::new("0xAAA", "AAA", "AAA Token", 18, ChainId::EthereumMainnet);
        let buy_token = Token::new("0xBBB", "BBB", "BBB Token", 18, ChainId::EthereumMainnet);
        Order::new(
            id,
            "0xowner",
            sell_token,
            buy_token,
            Decimal::from_i64(sell).unwrap(),
            Decimal::from_i64(buy).unwrap(),
            UNIX_EPOCH + Duration::from_secs(1_000),
            OrderType::Limit,
        )
    }

    #[test]
    fn test_weighted_average() {
        // Orders with prices 2, 2, and 1, weights 10, 5, 5 respectively.
        let orders = vec![
            dummy_order(1, 10, 20),
            dummy_order(2, 5, 10),
            dummy_order(3, 5, 5),
        ];
        // Expected weighted average: (2*10 + 2*5 + 1*5) / (10+5+5) = 35/20 = 1.75
        let price = VolumeWeightedPricingEngine::compute_clearing_price(&orders).unwrap();
        let expected = Decimal::from_i64(175).unwrap() / Decimal::from_i64(100).unwrap();
        assert_eq!(price, expected);
    }

    #[test]
    fn test_single_or_empty_returns_none() {
        assert!(VolumeWeightedPricingEngine::compute_clearing_price(&[]).is_none());
        let single = vec![dummy_order(1, 10, 20)];
        assert!(VolumeWeightedPricingEngine::compute_clearing_price(&single).is_none());
    }
}
