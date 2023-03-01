// use super::runtime;
// use crate::{
//     client::RuntimeClient,
//     types::{Contract, ContractResources, Farm, FarmPolicy, Hash, Node, Twin},
// };
// use subxt::{
//     events::Events,
//     rpc::types::{BlockNumber, NumberOrHex},
//     OnlineClient, PolkadotConfig,
// };

// pub struct Client {
//     api: OnlineClient<PolkadotConfig>,
// }

// impl Client {
//     pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
//         let api = OnlineClient::from_url(url).await?;
//         Ok(Client { api })
//     }
// }

// #[async_trait::async_trait]
// impl RuntimeClient for Client {
//     /// Get all events in a block.
//     async fn events(
//         &self,
//         block: Option<Hash>,
//     ) -> Result<Events<PolkadotConfig>, Box<dyn std::error::Error>> {
//         Ok(self.api.events().at(block).await?)
//     }

//     /// Get the hash of a block at the given height. Note that in this case, block is actually the
//     /// height rather than the hash to query at.
//     async fn hash_at_height(
//         &self,
//         block: Option<u32>,
//     ) -> Result<Option<Hash>, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .rpc()
//             .block_hash(block.map(|block| BlockNumber::from(NumberOrHex::from(block))))
//             .await?)
//     }

//     /// Get the on chain timestamp of the block, in seconds since the UNIX epoch.
//     async fn timestamp(&self, block: Option<Hash>) -> Result<i64, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(&runtime::api::storage().timestamp().now(), block)
//             .await?
//             .map(|u| u as i64)
//             .unwrap_or_default())
//     }

//     /// Get the twin referenced by this ID.
//     async fn twin(
//         &self,
//         id: u32,
//         block: Option<Hash>,
//     ) -> Result<Option<Twin>, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(&runtime::api::storage().tfgrid_module().twins(id), block)
//             .await?
//             .map(|t| t.into()))
//     }

//     /// Get the amount of twins on the grid.
//     async fn twin_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(&runtime::api::storage().tfgrid_module().twin_id(), block)
//             .await?
//             // SAFETY: This value is initialized in genesis and always set.
//             .unwrap())
//     }

//     /// Get the farm referenced by this ID.
//     async fn farm(
//         &self,
//         id: u32,
//         block: Option<Hash>,
//     ) -> Result<Option<Farm>, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(&runtime::api::storage().tfgrid_module().farms(id), block)
//             .await?
//             .map(|f| f.into()))
//     }

//     /// Get the payout address of the farm referenced by this ID.
//     async fn farm_payout_address(
//         &self,
//         id: u32,
//         block: Option<Hash>,
//     ) -> Result<Option<String>, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(
//                 &runtime::api::storage()
//                     .tfgrid_module()
//                     .farm_payout_v2_address_by_farm_id(id),
//                 block,
//             )
//             .await?
//             // SAFETY: This should be checked on chain, though not sure. Panic if it is not
//             .map(|f| String::from_utf8(f).unwrap()))
//     }

//     /// Get the amount of farms on the grid.
//     async fn farm_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(&runtime::api::storage().tfgrid_module().farm_id(), block)
//             .await?
//             // SAFETY: This value is initialized in genesis and always set.
//             .unwrap())
//     }

//     /// Get the node referenced by this ID.
//     async fn node(
//         &self,
//         id: u32,
//         block: Option<Hash>,
//     ) -> Result<Option<Node>, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(&runtime::api::storage().tfgrid_module().nodes(id), block)
//             .await?
//             .map(|n| n.into()))
//     }

//     /// Get the amount of nodes on the grid.
//     async fn node_count(&self, block: Option<Hash>) -> Result<u32, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(&runtime::api::storage().tfgrid_module().node_id(), block)
//             .await?
//             // SAFETY: This value is initialized in genesis and always set.
//             .unwrap())
//     }

//     /// Get the contract referenced by this ID.
//     async fn contract(
//         &self,
//         id: u64,
//         block: Option<Hash>,
//     ) -> Result<Option<Contract>, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(
//                 &runtime::api::storage()
//                     .smart_contract_module()
//                     .contracts(id),
//                 block,
//             )
//             .await?
//             .map(|c| c.into()))
//     }

//     /// Get the resources of the contract referenced by this ID.
//     async fn contract_resources(
//         &self,
//         id: u64,
//         block: Option<Hash>,
//     ) -> Result<Option<ContractResources>, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(
//                 &runtime::api::storage()
//                     .smart_contract_module()
//                     .node_contract_resources(id),
//                 block,
//             )
//             .await?
//             .map(|cr| cr.into()))
//     }

//     /// Get the amount of contracts on the grid.
//     async fn contract_count(&self, block: Option<Hash>) -> Result<u64, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(
//                 &runtime::api::storage()
//                     .smart_contract_module()
//                     .contract_id(),
//                 block,
//             )
//             .await?
//             // SAFETY: This value is initialized in genesis and always set.
//             .unwrap())
//     }

//     /// Get the farming policy referenced by this ID.
//     async fn farming_policy(
//         &self,
//         id: u32,
//         block: Option<Hash>,
//     ) -> Result<Option<FarmPolicy>, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(
//                 &runtime::api::storage()
//                     .tfgrid_module()
//                     .farming_policies_map(id),
//                 block,
//             )
//             .await?
//             .map(|fp| fp.into()))
//     }

//     /// Get the amount of farming policies on the grid.
//     async fn farming_policy_count(
//         &self,
//         block: Option<Hash>,
//     ) -> Result<u32, Box<dyn std::error::Error>> {
//         Ok(self
//             .api
//             .storage()
//             .fetch(
//                 &runtime::api::storage().tfgrid_module().farming_policy_id(),
//                 block,
//             )
//             .await?
//             // SAFETY: This value is initialized in genesis and always set.
//             .unwrap())
//     }
// }
