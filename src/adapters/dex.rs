use super::DexAdapter;
use rust_decimal::Decimal;
use crate::models::Token;

/// Dummy DEX adapter that returns zero quotes and swap amounts.
#[derive(Default)]
pub struct DummyDexAdapter;

impl DexAdapter for DummyDexAdapter {
    fn get_quote(&self, _sell_token: &Token, _buy_token: &Token, _amount: Decimal) -> Decimal {
        Decimal::ZERO
    }

    fn execute_swap(&self, _sell_token: &Token, _buy_token: &Token, _amount: Decimal) -> Decimal {
        Decimal::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Token, ChainId};
    use rust_decimal::prelude::*;

    fn dummy_token(symbol: &str) -> Token {
        Token::new("0xAAA", symbol, &format!("{} Token", symbol), 18, ChainId::EthereumMainnet)
    }

    #[test]
    fn test_dummy_dex_get_quote() {
        let dex = DummyDexAdapter::default();
        let sell_token = dummy_token("AAA");
        let buy_token = dummy_token("BBB");
        let amount = Decimal::ONE;
        let quote = dex.get_quote(&sell_token, &buy_token, amount);
        assert_eq!(quote, Decimal::ZERO);
    }

    #[test]
    fn test_dummy_dex_execute_swap() {
        let dex = DummyDexAdapter::default();
        let sell_token = dummy_token("AAA");
        let buy_token = dummy_token("BBB");
        let amount = Decimal::ONE;
        let out = dex.execute_swap(&sell_token, &buy_token, amount);
        assert_eq!(out, Decimal::ZERO);
    }
}
