use crate::client::RuntimeClient;
use crate::runtimes::{
    v115::types::{
        V115Contract, V115ContractCreatedEvent, V115ContractNruConsumptionReceivedEvent,
        V115ContractResources, V115ContractUpdatedResourcesEvent, V115Farm, V115FarmingPolicy,
        V115Node, V115NodeStoredEvent, V115NodeUpdatedEvent, V115NodeUptimeReportedEvent, V115Twin,
    },
    v123::types::{
        V123Contract, V123ContractCreatedEvent, V123ContractNruConsumptionReceivedEvent,
        V123ContractResources, V123ContractUpdatedResourcesEvent, V123Farm, V123FarmingPolicy,
        V123Node, V123NodeStoredEvent, V123NodeUpdatedEvent, V123NodeUptimeReportedEvent, V123Twin,
    },
};
use crate::types::{
    Contract, ContractResources, Farm, FarmPolicy, Hash, Node, RuntimeEvents, Twin, CONTRACTS,
    CONTRACT_CREATED, CONTRACT_ID, FARMING_POLICIES, FARMING_POLICY_ID, FARMS, FARM_ID,
    FARM_PAYOUT_V2_ADDRESS, NODES, NODE_CONTRACT_RESOURCES, NODE_ID, NODE_STORED, NODE_UPDATED,
    NODE_UPTIME_REPORTED, NRU_CONSUMPTION_RECEIVED, SMART_CONTRACT_MODULE, TFGRID_MODULE,
    TIMESTAMP_MODULE, TIMESTAMP_NOW, TWINS, TWIN_ID, UPDATE_USED_RESOURCES,
};
use subxt::storage::DynamicStorageAddress;
use subxt::{
    dynamic::Value,
    // events::Events,
    rpc::types::{BlockNumber, NumberOrHex},
    OnlineClient,
    PolkadotConfig,
};
use tokio::join;

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
    async fn events(
        &self,
        block: Option<Hash>,
    ) -> Result<Vec<RuntimeEvents>, Box<dyn std::error::Error>> {
        let (meta, runtime_v) = join!(
            self.api.rpc().metadata(block),
            self.api.rpc().runtime_version(block),
        );

        self.api.set_runtime_version(runtime_v?);
        self.api.set_metadata(meta?);

        let b_events = self.api.events().at(block).await?;

        let mut events: Vec<RuntimeEvents> = vec![];
        for event in b_events.iter() {
            if !event.is_ok() {
                continue;
            }
            let evt = event?;

            match (evt.pallet_name(), evt.variant_name()) {
                (TFGRID_MODULE, NODE_STORED) => {
                    if let Ok(Some(evt)) = evt.as_event::<V115NodeStoredEvent>() {
                        events.push(RuntimeEvents::NodeStoredEvent(evt.0.into()));
                    } else if let Ok(Some(evt)) = evt.as_event::<V123NodeStoredEvent>() {
                        events.push(RuntimeEvents::NodeStoredEvent(evt.0.into()));
                    };
                }
                (TFGRID_MODULE, NODE_UPDATED) => {
                    if let Ok(Some(evt)) = evt.as_event::<V115NodeUpdatedEvent>() {
                        events.push(RuntimeEvents::NodeUpdatedEvent(evt.0.into()));
                    } else if let Ok(Some(evt)) = evt.as_event::<V123NodeUpdatedEvent>() {
                        events.push(RuntimeEvents::NodeUpdatedEvent(evt.0.into()));
                    };
                }
                (TFGRID_MODULE, NODE_UPTIME_REPORTED) => {
                    if let Ok(Some(evt)) = evt.as_event::<V115NodeUptimeReportedEvent>() {
                        events.push(RuntimeEvents::NodeUptimeReported(evt.0, evt.1, evt.2));
                    } else if let Ok(Some(evt)) = evt.as_event::<V123NodeUptimeReportedEvent>() {
                        events.push(RuntimeEvents::NodeUptimeReported(evt.0, evt.1, evt.2));
                    };
                }
                (SMART_CONTRACT_MODULE, UPDATE_USED_RESOURCES) => {
                    if let Ok(Some(evt)) = evt.as_event::<V115ContractUpdatedResourcesEvent>() {
                        events.push(RuntimeEvents::ContractUsedResourcesUpdated(evt.0.into()));
                    } else if let Ok(Some(evt)) =
                        evt.as_event::<V123ContractUpdatedResourcesEvent>()
                    {
                        events.push(RuntimeEvents::ContractUsedResourcesUpdated(evt.0.into()));
                    };
                }
                (SMART_CONTRACT_MODULE, NRU_CONSUMPTION_RECEIVED) => {
                    if let Ok(Some(evt)) = evt.as_event::<V115ContractNruConsumptionReceivedEvent>()
                    {
                        events.push(RuntimeEvents::NruConsumptionReceived(evt.0.into()));
                    } else if let Ok(Some(evt)) =
                        evt.as_event::<V123ContractNruConsumptionReceivedEvent>()
                    {
                        events.push(RuntimeEvents::NruConsumptionReceived(evt.0.into()));
                    };
                }
                (SMART_CONTRACT_MODULE, CONTRACT_CREATED) => {
                    if let Ok(Some(evt)) = evt.as_event::<V115ContractCreatedEvent>() {
                        events.push(RuntimeEvents::ContractCreated(evt.0.into()));
                    } else if let Ok(Some(evt)) = evt.as_event::<V123ContractCreatedEvent>() {
                        events.push(RuntimeEvents::ContractCreated(evt.0.into()));
                    };
                }
                (_m, _e) => (),
            }
        }
        Ok(events)
    }

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
            subxt::dynamic::storage(TIMESTAMP_MODULE, TIMESTAMP_NOW, vec![]);
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
            subxt::dynamic::storage(TFGRID_MODULE, TWINS, vec![Value::u128(id.into())]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch(&storage_address)
            .await?;

        if result.is_none() {
            return Ok(None);
        }

        let r: Vec<u8> = result.unwrap().into_encoded().into();

        let twinv115: Result<V115Twin, codec::Error> = codec::decode_from_bytes(r.clone().into());
        if let Ok(twin) = twinv115 {
            Ok(Some(twin.into()))
        } else {
            let twinv123: V123Twin = codec::decode_from_bytes(r.into())?;
            Ok(Some(twinv123.into()))
        }
    }

    /// Get the amount of twins on the grid.
    async fn twin_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage(TFGRID_MODULE, TWIN_ID, vec![]);
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
            subxt::dynamic::storage(TFGRID_MODULE, FARMS, vec![Value::u128(id.into())]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch(&storage_address)
            .await?;

        if result.is_none() {
            return Ok(None);
        }

        let r: Vec<u8> = result.unwrap().into_encoded().into();

        let farmv115: Result<V115Farm, codec::Error> = codec::decode_from_bytes(r.clone().into());
        if let Ok(farm) = farmv115 {
            Ok(Some(farm.into()))
        } else {
            let farmv123: V123Farm = codec::decode_from_bytes(r.into())?;
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
            TFGRID_MODULE,
            FARM_PAYOUT_V2_ADDRESS,
            vec![Value::u128(id.into())],
        );
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch(&storage_address)
            .await?;

        if result.is_none() {
            return Ok(None);
        }

        let r: Vec<u8> = result.unwrap().into_encoded().into();

        Ok(Some(codec::decode_from_bytes(r.into())?))
    }

    /// Get the amount of farms on the grid.
    async fn farm_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage(TFGRID_MODULE, FARM_ID, vec![]);
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
            subxt::dynamic::storage(TFGRID_MODULE, NODES, vec![Value::u128(id.into())]);
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch(&storage_address)
            .await?;

        if result.is_none() {
            return Ok(None);
        }

        let r: Vec<u8> = result.unwrap().into_encoded().into();

        let nodev115: Result<V115Node, codec::Error> = codec::decode_from_bytes(r.clone().into());
        if let Ok(node) = nodev115 {
            Ok(Some(node.into()))
        } else {
            let nodev123: V123Node = codec::decode_from_bytes(r.into())?;
            Ok(Some(nodev123.into()))
        }
    }

    /// Get the amount of nodes on the grid.
    async fn node_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage(TFGRID_MODULE, NODE_ID, vec![]);
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
            SMART_CONTRACT_MODULE,
            CONTRACTS,
            vec![Value::u128(id.into())],
        );
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch(&storage_address)
            .await?;

        if result.is_none() {
            return Ok(None);
        }

        let r: Vec<u8> = result.unwrap().into_encoded().into();

        let cv115: Result<V115Contract, codec::Error> = codec::decode_from_bytes(r.clone().into());
        if let Ok(contract) = cv115 {
            Ok(Some(contract.into()))
        } else {
            let cv123: V123Contract = codec::decode_from_bytes(r.into())?;
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
            SMART_CONTRACT_MODULE,
            NODE_CONTRACT_RESOURCES,
            vec![Value::u128(id.into())],
        );
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch(&storage_address)
            .await?;

        if result.is_none() {
            return Ok(None);
        }

        let r: Vec<u8> = result.unwrap().into_encoded().into();

        let crv115: Result<V115ContractResources, codec::Error> =
            codec::decode_from_bytes(r.clone().into());
        if let Ok(resources) = crv115 {
            Ok(Some(resources.into()))
        } else {
            let crv123: V123ContractResources = codec::decode_from_bytes(r.into())?;
            Ok(Some(crv123.into()))
        }
    }

    /// Get the amount of contracts on the grid.
    async fn contract_count(&self, block: Option<Hash>) -> Result<u64, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage(SMART_CONTRACT_MODULE, CONTRACT_ID, vec![]);
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
            TFGRID_MODULE,
            FARMING_POLICIES,
            vec![Value::u128(id.into())],
        );
        let result = self
            .api
            .storage()
            .at(block)
            .await?
            .fetch(&storage_address)
            .await?;

        if result.is_none() {
            return Ok(None);
        }

        let r: Vec<u8> = result.unwrap().into_encoded().into();

        let fpv115: Result<V115FarmingPolicy, codec::Error> =
            codec::decode_from_bytes(r.clone().into());
        if let Ok(policy) = fpv115 {
            Ok(Some(policy.into()))
        } else {
            let fpv123: V123FarmingPolicy = codec::decode_from_bytes(r.into())?;
            Ok(Some(fpv123.into()))
        }
    }

    /// Get the amount of farming policies on the grid.
    async fn farming_policy_count(
        &self,
        block: Option<Hash>,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        let storage_address: DynamicStorageAddress<Value> =
            subxt::dynamic::storage(TFGRID_MODULE, FARMING_POLICY_ID, vec![]);
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
