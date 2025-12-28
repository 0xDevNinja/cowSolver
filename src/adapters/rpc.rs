use super::ChainRpcClient;

/// Dummy RPC client that returns a zero gas price and always succeeds.
#[derive(Default)]
pub struct DummyRpcClient;

impl ChainRpcClient for DummyRpcClient {
    fn get_gas_price(&self) -> u64 {
        0
    }

    fn submit_transaction(&self, _tx: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_rpc_get_gas_price() {
        let client = DummyRpcClient::default();
        assert_eq!(client.get_gas_price(), 0);
    }

    #[test]
    fn test_dummy_rpc_submit_transaction() {
        let client = DummyRpcClient::default();
        assert!(client.submit_transaction("0xdeadbeef"));
    }
}
