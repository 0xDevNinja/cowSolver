//! Solver engine implementation for batch auctions.

use super::{BatchAuction, Solver};
use crate::models::{ChainId, Settlement, Transfer};
use rust_decimal::Decimal;
use std::error::Error;

/// A default solver engine that uses a simple uniform clearing price to match orders.
pub struct SolverEngine {}

impl SolverEngine {
    /// Create a new solver engine instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Compute a uniform clearing price from the batch auction orders.
    ///
    /// This simplistic implementation computes the total sell and buy amounts
    /// across all orders and divides them to get an average price. Real
    /// implementations would use more sophisticated price discovery mechanisms.
    fn compute_clearing_price(&self, auction: &BatchAuction) -> Decimal {
        let total_sell = auction
            .orders
            .iter()
            .fold(Decimal::ZERO, |acc, order| acc + order.sell_amount);
        let total_buy = auction
            .orders
            .iter()
            .fold(Decimal::ZERO, |acc, order| acc + order.buy_amount);

        if total_buy.is_zero() {
            Decimal::ZERO
        } else {
            total_sell / total_buy
        }
    }
}

impl Solver for SolverEngine {
    fn solve(&self, auction: &BatchAuction) -> Result<Settlement, Box<dyn Error>> {
        // Compute clearing price
        let price = self.compute_clearing_price(auction);

        // TODO: In real implementation, generate transfers based on matching algorithm
        let transfers: Vec<Transfer> = Vec::new();

        // Determine settlement chain; fall back to EthereumMainnet if unspecified
        let chain = auction
            .settlement_chain
            .clone()
            .unwrap_or(ChainId::EthereumMainnet);

        // Build settlement with computed price and original orders
        let settlement = Settlement::new(price, auction.orders.clone(), transfers, chain);

        Ok(settlement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Order, OrderType, Token};
    use rust_decimal::Decimal;

    #[test]
    fn test_compute_clearing_price() {
        let sell_token = Token::new(
            "0xTokenA".into(),
            "TKA".into(),
            "Token A".into(),
            18,
            ChainId::EthereumMainnet,
        );
        let buy_token = Token::new(
            "0xTokenB".into(),
            "TKB".into(),
            "Token B".into(),
            18,
            ChainId::EthereumMainnet,
        );

        let order1 = Order::new(
            "1".into(),
            "Alice".into(),
            sell_token.clone(),
            buy_token.clone(),
            Decimal::new(10, 0),
            Decimal::new(5, 0),
            0,
            OrderType::Limit,
        );
        let order2 = Order::new(
            "2".into(),
            "Bob".into(),
            sell_token,
            buy_token,
            Decimal::new(20, 0),
            Decimal::new(10, 0),
            0,
            OrderType::Limit,
        );

        let auction = BatchAuction {
            orders: vec![order1, order2],
            clearing_price: None,
            settlement_chain: None,
            submitted_at: None,
        };
        let engine = SolverEngine::new();
        let price = engine.compute_clearing_price(&auction);
        assert_eq!(price, Decimal::new(30, 0) / Decimal::new(15, 0));
    }

    #[test]
    fn test_solve_returns_settlement() {
        let sell_token = Token::new(
            "0xTokenA".into(),
            "TKA".into(),
            "Token A".into(),
            18,
            ChainId::EthereumMainnet,
        );
        let buy_token = Token::new(
            "0xTokenB".into(),
            "TKB".into(),
            "Token B".into(),
            18,
            ChainId::EthereumMainnet,
        );

        let order = Order::new(
            "1".into(),
            "Alice".into(),
            sell_token.clone(),
            buy_token.clone(),
            Decimal::new(10, 0),
            Decimal::new(5, 0),
            0,
            OrderType::Limit,
        );
        let auction = BatchAuction {
            orders: vec![order],
            clearing_price: None,
            settlement_chain: Some(ChainId::EthereumMainnet),
            submitted_at: None,
        };
        let engine = SolverEngine::new();
        let settlement = engine.solve(&auction).unwrap();
        assert!(!settlement.orders.is_empty());
    }
}
