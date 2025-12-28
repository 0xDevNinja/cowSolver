use crate::models::{order::Order, settlement::Settlement};
use std::time::SystemTime;

/// Represents a batch of orders to be processed by the solver engine.
/// A batch consists of a list of `Order`s and a timestamp indicating when it was created.
#[derive(Debug, Clone)]
pub struct BatchAuction {
    pub orders: Vec<Order>,
    pub timestamp: SystemTime,
}

impl BatchAuction {
    /// Create a new `BatchAuction` from a list of orders and a timestamp.
    pub fn new(orders: Vec<Order>, timestamp: SystemTime) -> Self {
        Self { orders, timestamp }
    }
}

/// A trait representing the behaviour of a solver.
/// Implementations of this trait will take a batch of orders and produce a settlement.
pub trait Solver {
    /// Process a batch of orders and return a `Settlement`.
    fn process_batch(&self, batch: &BatchAuction) -> Settlement;
}

/// Solver engine module provides a default implementation of the `Solver` trait.
pub mod engine;
