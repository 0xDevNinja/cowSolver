/*!
Chain RPC clients implementation

This module defines concrete implementations of `ChainRpcClient` trait for
various blockchains. Each client encapsulates network settings and provides
methods to query gas prices and submit transactions.
*/

use super::ChainRpcClient;
use crate::models::ChainId;
use rust_decimal::Decimal;
use std::error::Error;

/// A dummy RPC client that can be used for testing or as a placeholder.
/// Returns zero gas price and acknowledges transaction submissions.
pub struct DummyRpcClient;

impl ChainRpcClient for DummyRpcClient {
    fn gas_price(&self, _chain: ChainId) -> Result<Decimal, Box<dyn Error>> {
        // TODO: integrate with actual RPC endpoints to fetch gas price.
        Ok(Decimal::ZERO)
    }

    fn submit_transaction(&self, _chain: ChainId, _data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        // TODO: integrate with actual RPC endpoints to submit the transaction.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ChainId;
    use rust_decimal::Decimal;

    #[test]
    fn dummy_rpc_client_returns_zero_gas_price() {
        let client = DummyRpcClient;
        let price = client.gas_price(ChainId::EthereumMainnet).unwrap();
        assert_eq!(price, Decimal::ZERO);
    }

    #[test]
    fn dummy_rpc_client_submits_tx() {
        let client = DummyRpcClient;
        let result = client.submit_transaction(ChainId::EthereumMainnet, vec![0u8; 32]);
        assert!(result.is_ok());
    }
}
