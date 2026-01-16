# Chainstore Fallback

Forest includes an **optional chainstore fallback** mechanism that allows the node to fetch missing blocks from the network when they are absent from the local database. This behaviour mirrors the Lotus node's `LOTUS_ENABLE_CHAINSTORE_FALLBACK` feature and can help keep the chainstore in sync when running on archival or non-validating nodes.

## Enabling Fallback

To activate the fallback mechanism, set the environment variable `FOREST_ENABLE_CHAINSTORE_FALLBACK` to `true` when starting the node. For example:

```
FOREST_ENABLE_CHAINSTORE_FALLBACK=true forest
```

When fallback is enabled, the chainstore attempts to read blocks from the local store. If a block is not found, the fallback implementation invokes Bitswap via the `BitswapRequestManager` to fetch the missing block from peers. Once retrieved, the block is cached in the local store and returned to the caller. If the environment variable is unset or set to any other value, the chainstore operates in strict mode and returns an error when data is missing.

## Considerations

- **Network traffic:** Fetching data from peers incurs additional network traffic and latency. Enable fallback primarily for archival or non-validating nodes where occasional missing data is acceptable.
- **Performance:** Fallback may slow down block validation since the node must wait for network retrieval. Ensure your node monitors logs and performance metrics when using this feature.
- **Experimental status:** This feature is experimental and may evolve. Review updates to ensure compatibility with future protocol or network changes.

## Implementation Details

The fallback logic is implemented in the file `src/chain/store/blockstore_fallback.rs`. It provides a `FallbackBlockstore` wrapper that checks the `FOREST_ENABLE_CHAINSTORE_FALLBACK` environment variable and invokes Bitswap if necessary. See the source for more details on error handling and integration with the existing `ChainStore`.
