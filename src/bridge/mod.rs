use crate::models::{Settlement, ChainId, Order, OrderType, Token};
use crate::models::settlement::Transfer;
use rust_decimal::Decimal;
use std::time::{UNIX_EPOCH, Duration};

/// Error type for bridging operations.
#[derive(Debug)]
pub enum BridgeError {
    /// Invalid target chain specified for the bridge.
    InvalidChain,
    /// The bridge execution failed with an associated message.
    ExecutionFailed(String),
}

/// The `BridgeAdapter` trait defines a common interface for cross-chain settlement bridges.
///
/// Implementations of this trait are responsible for taking a settlement that was
/// computed on one chain (the source) and executing the necessary transactions to
/// realize that settlement on the specified `target_chain`. This might involve
/// interacting with bridge contracts, waiting for confirmations, and handling
/// any errors returned by the underlying bridge infrastructure.
pub trait BridgeAdapter {
    /// Bridge a settlement to a target chain.
    ///
    /// # Parameters
    /// - `settlement`: The settlement to bridge.
    /// - `target_chain`: The chain to which the settlement should be executed.
    ///
    /// # Returns
    /// - `Ok(())` if the bridging operation succeeded.
    /// - `Err(BridgeError)` if the operation failed for some reason.
    fn bridge_settlement(&self, settlement: &Settlement, target_chain: ChainId) -> Result<(), BridgeError>;
}

/// A dummy bridge implementation used for testing and development.
///
/// This adapter performs no action and always returns `Ok(())`. It can be
/// replaced with a real bridge implementation once a specific bridging
/// protocol (e.g., Hop, Connext, Wormhole) is chosen for production.
#[derive(Default)]
pub struct DummyBridge;

impl BridgeAdapter for DummyBridge {
    fn bridge_settlement(&self, _settlement: &Settlement, _target_chain: ChainId) -> Result<(), BridgeError> {
        // In a real implementation, this method would contain logic to
        // interact with bridge contracts and handle cross-chain messaging.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_order(id: u64) -> Order {
        let sell_token = Token::new("0xaaa", "AAA", "AAA Token", 18, ChainId::EthereumMainnet);
        let buy_token = Token::new("0xbbb", "BBB", "BBB Token", 18, ChainId::EthereumMainnet);
        Order::new(
            id,
            "0xowner",
            sell_token,
            buy_token,
            Decimal::ONE,
            Decimal::ONE,
            UNIX_EPOCH + Duration::from_secs(1000),
            OrderType::Limit,
        )
    }

    fn dummy_settlement() -> Settlement {
        let orders = vec![dummy_order(1), dummy_order(2)];
        let transfers: Vec<Transfer> = Vec::new();
        Settlement::new(Decimal::ONE, orders, transfers, ChainId::EthereumMainnet)
    }

    #[test]
    fn test_dummy_bridge_settlement_ok() {
        let bridge = DummyBridge::default();
        let settlement = dummy_settlement();
        let result = bridge.bridge_settlement(&settlement, ChainId::EthereumMainnet);
        assert!(result.is_ok());
    }
}
