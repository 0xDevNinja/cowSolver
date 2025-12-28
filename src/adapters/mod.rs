/*!
Adapters Layer

Provides chain RPC clients and DEX integrations used by the solver.
*/

pub mod rpc;
pub mod dex;

use crate::models::{ChainId, Token};
use rust_decimal::Decimal;
use std::error::Error;

/// A trait for chain-specific RPC clients.
pub trait ChainRpcClient {
    /// Returns the current gas price on the given chain.
    fn gas_price(&self, chain: ChainId) -> Result<Decimal, Box<dyn Error>>;
    /// Submits a settlement or transaction to the chain.
    fn submit_transaction(&self, chain: ChainId, data: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

/// A trait for DEX adapters providing price quotes and swap execution.
pub trait DexAdapter {
    /// Returns a price quote for swapping `sell_amount` of `sell_token` into `buy_token`.
    fn get_quote(&self, sell_token: &Token, buy_token: &Token, sell_amount: Decimal) -> Result<Decimal, Box<dyn Error>>;
    /// Executes a swap and returns the actual amount received.
    fn execute_swap(&self, sell_token: &Token, buy_token: &Token, sell_amount: Decimal) -> Result<Decimal, Box<dyn Error>>;
}
