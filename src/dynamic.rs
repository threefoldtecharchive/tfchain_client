use crate::client::RuntimeClient;
use crate::runtimes::{
    v115::types::{
        V115Contract, V115ContractResources, V115Farm, V115FarmingPolicy, V115Node, V115Twin,
    },
    v123::types::{
        V123Contract, V123ContractResources, V123Farm, V123FarmingPolicy, V123Node, V123Twin,
    },
};
use crate::types::{Contract, ContractResources, Farm, FarmPolicy, Hash, Node, Twin};
use subxt::storage::DynamicStorageAddress;
use subxt::{
    dynamic::Value,
    // events::Events,
    rpc::types::{BlockNumber, NumberOrHex},
    OnlineClient,
    PolkadotConfig,
};

#[derive(Debug)]
pub enum Error {
    ErrorDecodingTwin,
}

pub struct DynamicClient {
    api: OnlineClient<PolkadotConfig>,
}

impl DynamicClient {
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let api = OnlineClient::from_url(url).await?;
        Ok(DynamicClient { api })
    }
}

#[async_trait::async_trait]
impl RuntimeClient for DynamicClient {
    /// Get all events in a block.
    // async fn events(
    //     &self,
    //     block: Option<Hash>,
    // ) -> Result<Events<PolkadotConfig>, Box<dyn std::error::Error>> {
    //     let storage_address = subxt::dynamic::storage("System", "Events", vec![Value::from(block)]);
    //     let events = self
    //         .api
    //         .storage()
    //         .at(block)
    //         .await?
    //         .fetch_or_default(&storage_address)
    //         .await?
    //         .to_value()?;
    //     Ok(events.try_into().unwrap())
    // }

    /// Get the hash of a block at the given height. Note that in this case, block is actually the
    /// height rather than the hash to query at.
    async fn hash_at_height(
        &self,
        block: Option<u32>,
    ) -> Result<Option<Hash>, Box<dyn std::error::Error>> {
        Ok(self
            .api
            .rpc()
            .block_hash(block.map(|block| BlockNumber::from(NumberOrHex::from(block))))
            .await?)
    }

    /// Get the on chain timestamp of the block, in seconds since the UNIX epoch.
    async fn timestamp(&self, block: Option<Hash>) -> Result<u64, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage("Timestamp", "now", vec![]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .to_value()?;

        Ok(result.as_u128().map_or(0, |x| x as u64))
    }

    /// Get the twin referenced by this ID.
    async fn twin(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<Twin>, Box<dyn std::error::Error>> {
        let storage_address =
            subxt::dynamic::storage("TfgridModule", "Twins", vec![Value::u128(id.into())]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .into_encoded();

        let twinv115: Result<V115Twin, codec::Error> =
            codec::decode_from_bytes(result.clone().into());
        if let Ok(twin) = twinv115 {
            Ok(Some(twin.into()))
        } else {
            let twinv123: V123Twin = codec::decode_from_bytes(result.into())?;
            Ok(Some(twinv123.into()))
        }
    }

    /// Get the amount of twins on the grid.
    async fn twin_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage("TfgridModule", "TwinID", vec![]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .to_value()?;

        Ok(result.as_u128().map_or(0, |x| x as u32))
    }

    /// Get the farm referenced by this ID.
    async fn farm(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<Farm>, Box<dyn std::error::Error>> {
        let storage_address =
            subxt::dynamic::storage("TfgridModule", "Farms", vec![Value::u128(id.into())]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .into_encoded();

        let farmv115: Result<V115Farm, codec::Error> =
            codec::decode_from_bytes(result.clone().into());
        if let Ok(farm) = farmv115 {
            Ok(Some(farm.into()))
        } else {
            let farmv123: V123Farm = codec::decode_from_bytes(result.into())?;
            Ok(Some(farmv123.into()))
        }
    }

    /// Get the payout address of the farm referenced by this ID.
    async fn farm_payout_address(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let storage_address = subxt::dynamic::storage(
            "TfgridModule",
            "FarmPayoutV2AddressByFarmID",
            vec![Value::u128(id.into())],
        );
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .into_encoded();

        Ok(Some(codec::decode_from_bytes(result.into())?))
    }

    /// Get the amount of farms on the grid.
    async fn farm_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage("TfgridModule", "FarmID", vec![]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .to_value()?;

        Ok(result.as_u128().map_or(0, |x| x as u32))
    }

    /// Get the node referenced by this ID.
    async fn node(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<Node>, Box<dyn std::error::Error>> {
        let storage_address =
            subxt::dynamic::storage("TfgridModule", "Nodes", vec![Value::u128(id.into())]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .into_encoded();

        let nodev115: Result<V115Node, codec::Error> =
            codec::decode_from_bytes(result.clone().into());
        if let Ok(node) = nodev115 {
            Ok(Some(node.into()))
        } else {
            let nodev123: V123Node = codec::decode_from_bytes(result.into())?;
            Ok(Some(nodev123.into()))
        }
    }

    /// Get the amount of nodes on the grid.
    async fn node_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage("TfgridModule", "NodeID", vec![]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .to_value()?;

        Ok(result.as_u128().map_or(0, |x| x as u32))
    }

    /// Get the contract referenced by this ID.
    async fn contract(
        &self,
        id: u64,
        block: Option<Hash>,
    ) -> Result<Option<Contract>, Box<dyn std::error::Error>> {
        let storage_address = subxt::dynamic::storage(
            "SmartContractModule",
            "Contracts",
            vec![Value::u128(id.into())],
        );
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .into_encoded();

        let cv115: Result<V115Contract, codec::Error> =
            codec::decode_from_bytes(result.clone().into());
        if let Ok(contract) = cv115 {
            Ok(Some(contract.into()))
        } else {
            let cv123: V123Contract = codec::decode_from_bytes(result.into())?;
            Ok(Some(cv123.into()))
        }
    }

    /// Get the resources of the contract referenced by this ID.
    async fn contract_resources(
        &self,
        id: u64,
        block: Option<Hash>,
    ) -> Result<Option<ContractResources>, Box<dyn std::error::Error>> {
        let storage_address = subxt::dynamic::storage(
            "SmartContractModule",
            "NodeContractResources",
            vec![Value::u128(id.into())],
        );
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .into_encoded();

        let crv115: Result<V115ContractResources, codec::Error> =
            codec::decode_from_bytes(result.clone().into());
        if let Ok(resources) = crv115 {
            Ok(Some(resources.into()))
        } else {
            let crv123: V123ContractResources = codec::decode_from_bytes(result.into())?;
            Ok(Some(crv123.into()))
        }
    }

    /// Get the amount of contracts on the grid.
    async fn contract_count(&self, block: Option<Hash>) -> Result<u64, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage("SmartContractModule", "ContractID", vec![]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .to_value()?;

        Ok(result.as_u128().map_or(0, |x| x as u64))
    }

    /// Get the farming policy referenced by this ID.
    async fn farming_policy(
        &self,
        id: u32,
        block: Option<Hash>,
    ) -> Result<Option<FarmPolicy>, Box<dyn std::error::Error>> {
        let storage_address = subxt::dynamic::storage(
            "TfgridModule",
            "FarmingPolicies",
            vec![Value::u128(id.into())],
        );
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .into_encoded();

        let fpv115: Result<V115FarmingPolicy, codec::Error> =
            codec::decode_from_bytes(result.clone().into());
        if let Ok(policy) = fpv115 {
            Ok(Some(policy.into()))
        } else {
            let fpv123: V123FarmingPolicy = codec::decode_from_bytes(result.into())?;
            Ok(Some(fpv123.into()))
        }
    }

    /// Get the amount of farming policies on the grid.
    async fn farming_policy_count(
        &self,
        block: Option<Hash>,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage("TfgridModule", "FarmingPolicyID", vec![]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch_or_default(&storage_address)
            .await?
            .to_value()?;

        Ok(result.as_u128().map_or(0, |x| x as u32))
    }
}
