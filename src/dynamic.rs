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
    Contract, ContractResources, Farm, FarmPolicy, Hash, Node, RuntimeEvents, Twin,
    CONTRACT_CREATED, NODE_STORED, NODE_UPDATED, NODE_UPTIME_REPORTED, NRU_CONSUMPTION_RECEIVED,
    SMART_CONTRACT_MODULE, TFGRID_MODULE, UPDATE_USED_RESOURCES,
};
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
    async fn events(
        &self,
        block: Option<Hash>,
    ) -> Result<Vec<RuntimeEvents>, Box<dyn std::error::Error>> {
        let block = self.api.blocks().at(block).await?;

        let mut events: Vec<RuntimeEvents> = vec![];
        for event in block.events().await?.iter() {
            let evt = event?;

            match (evt.pallet_name(), evt.variant_name()) {
                (TFGRID_MODULE, NODE_STORED) => {
                    if evt.as_event::<V115NodeStoredEvent>().is_ok() {
                        let node = evt
                            .as_event::<V115NodeStoredEvent>()
                            .unwrap()
                            .unwrap()
                            .0
                            .into();
                        events.push(RuntimeEvents::NodeStoredEvent(node))
                    } else if evt.as_event::<V123NodeStoredEvent>().is_ok() {
                        let node = evt
                            .as_event::<V123NodeStoredEvent>()
                            .unwrap()
                            .unwrap()
                            .0
                            .into();
                        events.push(RuntimeEvents::NodeStoredEvent(node))
                    }
                }
                (TFGRID_MODULE, NODE_UPDATED) => {
                    if evt.as_event::<V115NodeUpdatedEvent>().is_ok() {
                        let node = evt
                            .as_event::<V115NodeUpdatedEvent>()
                            .unwrap()
                            .unwrap()
                            .0
                            .into();
                        events.push(RuntimeEvents::NodeStoredEvent(node))
                    } else if evt.as_event::<V123NodeUpdatedEvent>().is_ok() {
                        let node = evt
                            .as_event::<V123NodeUpdatedEvent>()
                            .unwrap()
                            .unwrap()
                            .0
                            .into();
                        events.push(RuntimeEvents::NodeStoredEvent(node))
                    }
                }
                (TFGRID_MODULE, NODE_UPTIME_REPORTED) => {
                    if evt.as_event::<V115NodeUptimeReportedEvent>().is_ok() {
                        let uptime = evt
                            .as_event::<V115NodeUptimeReportedEvent>()
                            .unwrap()
                            .unwrap();
                        events.push(RuntimeEvents::NodeUptimeReported(
                            uptime.0, uptime.1, uptime.2,
                        ))
                    } else if evt.as_event::<V123NodeUptimeReportedEvent>().is_ok() {
                        let uptime = evt
                            .as_event::<V123NodeUptimeReportedEvent>()
                            .unwrap()
                            .unwrap();
                        events.push(RuntimeEvents::NodeUptimeReported(
                            uptime.0, uptime.1, uptime.2,
                        ))
                    }
                }
                (SMART_CONTRACT_MODULE, UPDATE_USED_RESOURCES) => {
                    if evt.as_event::<V115ContractUpdatedResourcesEvent>().is_ok() {
                        let contract_resources = evt
                            .as_event::<V115ContractUpdatedResourcesEvent>()
                            .unwrap()
                            .unwrap();
                        events.push(RuntimeEvents::ContractUsedResourcesUpdated(
                            contract_resources.0.into(),
                        ))
                    } else if evt.as_event::<V123ContractUpdatedResourcesEvent>().is_ok() {
                        let contract_resources = evt
                            .as_event::<V123ContractUpdatedResourcesEvent>()
                            .unwrap()
                            .unwrap();
                        events.push(RuntimeEvents::ContractUsedResourcesUpdated(
                            contract_resources.0.into(),
                        ))
                    }
                }
                (SMART_CONTRACT_MODULE, NRU_CONSUMPTION_RECEIVED) => {
                    if evt
                        .as_event::<V115ContractNruConsumptionReceivedEvent>()
                        .is_ok()
                    {
                        let nru = evt
                            .as_event::<V115ContractNruConsumptionReceivedEvent>()
                            .unwrap()
                            .unwrap();
                        events.push(RuntimeEvents::NruConsumptionReceived(nru.0.into()))
                    } else if evt
                        .as_event::<V123ContractNruConsumptionReceivedEvent>()
                        .is_ok()
                    {
                        let nru = evt
                            .as_event::<V123ContractNruConsumptionReceivedEvent>()
                            .unwrap()
                            .unwrap();
                        events.push(RuntimeEvents::NruConsumptionReceived(nru.0.into()))
                    }
                }
                (SMART_CONTRACT_MODULE, CONTRACT_CREATED) => {
                    if evt.as_event::<V115ContractCreatedEvent>().is_ok() {
                        let contract = evt.as_event::<V115ContractCreatedEvent>().unwrap().unwrap();
                        events.push(RuntimeEvents::ContractCreated(contract.0.into()))
                    } else if evt.as_event::<V123ContractCreatedEvent>().is_ok() {
                        let contract = evt.as_event::<V123ContractCreatedEvent>().unwrap().unwrap();
                        events.push(RuntimeEvents::ContractCreated(contract.0.into()))
                    }
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
            subxt::dynamic::storage("Timestamp", "Now", vec![]);
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
            subxt::dynamic::storage(TFGRID_MODULE, "Twins", vec![Value::u128(id.into())]);
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
            subxt::dynamic::storage(TFGRID_MODULE, "TwinID", vec![]);
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
            subxt::dynamic::storage(TFGRID_MODULE, "Farms", vec![Value::u128(id.into())]);
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
            "FarmPayoutV2AddressByFarmID",
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
            subxt::dynamic::storage(TFGRID_MODULE, "FarmID", vec![]);
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
            subxt::dynamic::storage(TFGRID_MODULE, "Nodes", vec![Value::u128(id.into())]);
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
            subxt::dynamic::storage(TFGRID_MODULE, "NodeID", vec![]);
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
            "Contracts",
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
            "NodeContractResources",
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
            subxt::dynamic::storage(SMART_CONTRACT_MODULE, "ContractID", vec![]);
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
            "FarmingPoliciesMap",
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
            subxt::dynamic::storage(TFGRID_MODULE, "FarmingPolicyID", vec![]);
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
