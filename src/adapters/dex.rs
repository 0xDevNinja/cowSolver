/*!
DEX adapters implementation

This module defines concrete implementations of the `DexAdapter` trait.
*/

use super::DexAdapter;
use crate::models::{Token, ChainId};
use rust_decimal::Decimal;
use std::error::Error;
use rust_decimal::prelude::*;

/// A dummy DEX adapter that provides no liquidity and returns zero amounts.
/// Useful for testing or as a placeholder until a real DEX integration is implemented.
pub struct DummyDexAdapter;

impl DexAdapter for DummyDexAdapter {
    fn get_quote(&self, _sell_token: &Token, _buy_token: &Token, _sell_amount: Decimal) -> Result<Decimal, Box<dyn Error>> {
        // Return zero as there is no liquidity.
        Ok(Decimal::ZERO)
    }

    fn execute_swap(&self, _sell_token: &Token, _buy_token: &Token, _sell_amount: Decimal) -> Result<Decimal, Box<dyn Error>> {
        // Return zero as there is no liquidity.
        Ok(Decimal::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Token, ChainId, OrderType};
    use rust_decimal::Decimal;
    use rust_decimal::prelude::*;

    fn dummy_token(symbol: &str) -> Token {
        // Use chain id EthereumMainnet for dummy tokens
        Token::new(symbol.to_string(), symbol.to_string(), 18, ChainId::EthereumMainnet)
    }

    #[test]
    fn dummy_dex_adapter_returns_zero_quote() {
        let adapter = DummyDexAdapter;
        let sell = dummy_token("ETH");
        let buy = dummy_token("DAI");
        let quote = adapter.get_quote(&sell, &buy, Decimal::from_i64(1).unwrap()).unwrap();
        assert_eq!(quote, Decimal::ZERO);
    }

    #[test]
    fn dummy_dex_adapter_execute_swap_returns_zero() {
        let adapter = DummyDexAdapter;
        let sell = dummy_token("ETH");
        let buy = dummy_token("DAI");
        let result = adapter.execute_swap(&sell, &buy, Decimal::from_i64(1).unwrap()).unwrap();
        assert_eq!(result, Decimal::ZERO);
    }
}
