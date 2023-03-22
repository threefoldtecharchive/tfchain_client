pub use subxt::events::Events;
pub use subxt::PolkadotConfig;

use crate::types::{
    Contract, ContractResources, Farm, FarmPolicy, Hash, Node, RuntimeEvents, Twin,
};

/// The expected amount of seconds per block.
const BLOCK_TIME_SECONDS: i64 = 6;

/// This is the general set of methods which are available on the individual runtime libraries. In
/// general, methods and types here will adhere to the latest format on the grid, as to have all
/// available data. It is up to the individual runtimes to modify the data and access to the data
/// such that it is compatible with this trait.
///
/// All methods have a `block` argument, which is an [`Option<Hash>`]. If this is set, the method
/// will be called on that specific block, i.e. it will return data from that block. If not set,
/// the latest block will be used.
///
/// # Compatibility
///
/// It is up to the user to ensure that the actual implementation understands how to encode or
/// decode the data at the given block, or switch to an appropriate client if that is not the case.
#[async_trait::async_trait]
pub trait RuntimeClient {
    /// Get all events in a block.
    async fn events(
        &self,
        block: Option<Hash>,
    ) -> Result<Vec<RuntimeEvents>, Box<dyn std::error::Error>>;

    /// Get the hash of a block at the given height. Note that in this case, block is actually the
    /// height rather than the hash to query at.
    async fn hash_at_height(
        &self,
        block: Option<u32>,
    ) -> Result<Option<Hash>, Box<dyn std::error::Error>>;

    /// Get the on chain timestamp of the block, in seconds since the UNIX epoch.
    async fn timestamp(&self, block: Option<Hash>) -> Result<u64, Box<dyn std::error::Error>>;

    /// Get the twin referenced by this ID.
    async fn twin(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<Twin>, Box<dyn std::error::Error>>;

    /// Get the amount of twins on the grid.
    async fn twin_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>>;

    /// Get the farm referenced by this ID.
    async fn farm(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<Farm>, Box<dyn std::error::Error>>;

    /// Get the payout address of the farm referenced by this ID.
    async fn farm_payout_address(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<String>, Box<dyn std::error::Error>>;

    /// Get the amount of farms on the grid.
    async fn farm_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>>;

    /// Get the node referenced by this ID.
    async fn node(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<Node>, Box<dyn std::error::Error>>;

    /// Get the amount of nodes on the grid.
    async fn node_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>>;

    /// Get the contract referenced by this ID.
    async fn contract(
        &self,
        id: u64,
        block: Option<Hash>,
    ) -> Result<Option<Contract>, Box<dyn std::error::Error>>;

    /// Get the resources of the contract referenced by this ID.
    async fn contract_resources(
        &self,
        id: u64,
        block: Option<Hash>,
    ) -> Result<Option<ContractResources>, Box<dyn std::error::Error>>;

    /// Get the amount of contracts on the grid.
    async fn contract_count(&self, block: Option<Hash>) -> Result<u64, Box<dyn std::error::Error>>;

    /// Get the farming policy referenced by this ID.
    async fn farming_policy(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<FarmPolicy>, Box<dyn std::error::Error>>;

    /// Get the amount of farming policies on the grid.
    async fn farming_policy_count(
        &self,
        block: Option<Hash>,
    ) -> Result<u32, Box<dyn std::error::Error>>;
}

/// Find the height of the chain at the given timestamp.
///
/// This method takes any client, since we assume that the basic storage does not change, and is
/// therefore consistent across multiple chain versions.
pub async fn height_at_timestamp(
    client: &dyn RuntimeClient,
    ts: i64,
) -> Result<u32, Box<dyn std::error::Error>> {
    let latest_ts = (client.timestamp(None).await? / 1000) as i64;
    if latest_ts < ts {
        panic!(
            "can't fetch block for future timestamp {} vs latest {}",
            ts, latest_ts
        );
    }
    let mut height = 1;
    let mut last_height = 1;
    loop {
        let hash = match client.hash_at_height(Some(height)).await? {
            Some(hash) => hash,
            None => {
                height = (height + last_height) / 2;
                continue;
            }
        };
        let block_time = (client.timestamp(Some(hash)).await? / 1000) as i64;
        let time_delta = ts - block_time;
        let block_delta = time_delta / BLOCK_TIME_SECONDS;
        if block_delta == 0 {
            if time_delta >= 0 {
                return Ok(height + 1);
            } else {
                return Ok(height);
            }
        }
        if (height as i64 + block_delta) < 0 {
            panic!(
                "negative height search (height {} delta {})",
                height, block_delta
            );
        }

        last_height = height;

        height = (height as i64 + block_delta) as u32;
    }
}
