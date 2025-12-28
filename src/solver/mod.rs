use crate::models::{Order, Settlement, ChainId};
use rust_decimal::Decimal;
use std::time::SystemTime;
use std::error::Error;

/// Represents a batch auction consisting of a set of orders and optional metadata.
#[derive(Debug, Clone)]
pub struct BatchAuction {
    /// Orders submitted for this batch auction.
    pub orders: Vec<Order>,
    /// Optional clearing price computed by a solver.
    pub clearing_price: Option<Decimal>,
    /// Optional target chain on which settlement should occur.
    pub settlement_chain: Option<ChainId>,
    /// Timestamp when the auction was submitted.
    pub submitted_at: Option<SystemTime>,
}

impl BatchAuction {
    /// Create a new batch auction from a list of orders.
    pub fn new(orders: Vec<Order>) -> Self {
        Self {
            orders,
            clearing_price: None,
            settlement_chain: None,
            submitted_at: Some(SystemTime::now()),
        }
    }
}

/// A trait representing a solver that can process a batch auction and return a settlement.
pub trait Solver {
    /// Solve a batch auction and produce a settlement.
    fn solve(&self, auction: &BatchAuction) -> Result<Settlement, Box<dyn Error>>;
}

/// Solver engine module providing implementations of the Solver trait.
pub mod engine;
