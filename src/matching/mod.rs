use crate::models::Order;

/// A grouping of orders that form a valid match (e.g., CoW pair or ring).
#[derive(Debug, Clone)]
pub struct Match {
    /// Orders participating in this match.
    pub orders: Vec<Order>,
}

/// Trait for matching orders into CoWs or rings.
pub trait OrderMatchingStrategy {
    /// Given a slice of orders, produce a list of matches.
    fn match_orders(&self, orders: &[Order]) -> Vec<Match>;
}

/// Simple pair matcher and ring matcher modules.
pub mod pair;
pub mod ring;
