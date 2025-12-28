// Adapters module for chain RPC clients and DEX adapters.

pub trait ChainRpcClient {
    fn get_gas_price(&self) -> u64;
    fn submit_transaction(&self, tx: &str) -> bool;
}

pub trait DexAdapter {
    fn get_quote(&self, sell_token: &crate::models::Token, buy_token: &crate::models::Token, amount: rust_decimal::Decimal) -> rust_decimal::Decimal;
    fn execute_swap(&self, sell_token: &crate::models::Token, buy_token: &crate::models::Token, amount: rust_decimal::Decimal) -> rust_decimal::Decimal;
}

pub mod rpc;
pub mod dex;
