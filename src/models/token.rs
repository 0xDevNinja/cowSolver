use serde::{Deserialize, Serialize};

use super::chain::ChainId;

/// Represents a token on a specific blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Token {
    /// The on-chain address of the token contract.
    pub address: String,
    /// The symbol (e.g., ETH, DAI).
    pub symbol: String,
    /// The human‑readable name of the token.
    pub name: String,
    /// Number of decimal places used by the token.
    pub decimals: u8,
    /// The chain on which this token resides.
    pub chain_id: ChainId,
}

impl Token {
    /// Creates a new token instance.
    pub fn new(
        address: impl Into<String>,
        symbol: impl Into<String>,
        name: impl Into<String>,
        decimals: u8,
        chain_id: ChainId,
    ) -> Self {
        Self {
            address: address.into(),
            symbol: symbol.into(),
            name: name.into(),
            decimals,
            chain_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn token_serialization_roundtrip() {
        let token = Token::new(
            "0x0000000000000000000000000000000000000000",
            "ETH",
            "Ether",
            18,
            ChainId::EthereumMainnet,
        );
        let json = serde_json::to_string(&token).expect("serialize");
        let deserialized: Token = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(token, deserialized);
    }
}
