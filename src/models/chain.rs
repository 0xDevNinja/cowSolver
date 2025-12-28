use serde::{Deserialize, Serialize};

/// Supported blockchain network identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChainId {
    /// Ethereum mainnet (chain ID 1).
    EthereumMainnet,
    /// Arbitrum One rollup chain.
    ArbitrumOne,
    /// Optimism rollup chain.
    Optimism,
    /// Polygon (formerly Matic) PoS chain.
    Polygon,
    /// Binance Smart Chain.
    BinanceSmartChain,
    /// Gnosis Chain (formerly xDai).
    GnosisChain,
}

impl ChainId {
    /// Returns a lowercase string identifier for the chain.
    pub fn as_str(&self) -> &'static str {
        match self {
            ChainId::EthereumMainnet => "ethereum",
            ChainId::ArbitrumOne => "arbitrum",
            ChainId::Optimism => "optimism",
            ChainId::Polygon => "polygon",
            ChainId::BinanceSmartChain => "binance-smart-chain",
            ChainId::GnosisChain => "gnosis",
        }
    }
}

impl std::fmt::Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn chainid_serialization_roundtrip() {
        let id = ChainId::Optimism;
        let json = serde_json::to_string(&id).unwrap();
        let des: ChainId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, des);
    }

    #[test]
    fn display_outputs_expected_string() {
        assert_eq!(ChainId::EthereumMainnet.to_string(), "ethereum");
        assert_eq!(ChainId::Polygon.to_string(), "polygon");
    }
}
