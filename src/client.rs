//! Low level client to interact with the chain. For upstream usage, other than constructing a
//! [Client], you likely want to look at the [window](crate::window) module.

use crate::events::TfchainEvent;
pub use crate::types::Hash;
use crate::types::{AccountData, AccountInfo, BlockNumber, Contract, Farm, Node, Twin};
use runtime::Block;
pub use sp_core::crypto::AccountId32;
use sp_core::crypto::Pair;
use std::sync::mpsc;
use std::sync::Arc;
use substrate_api_client::sp_runtime::MultiSignature;
use substrate_api_client::{
    compose_extrinsic, Api, ApiClientError, UncheckedExtrinsicV4, XtStatus,
};

pub type ApiResult<T> = Result<T, ApiClientError>;

#[derive(Clone)]
pub struct SharedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    inner: Arc<Client<P>>,
}

impl<P> SharedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn new(client: Client<P>) -> Self {
        Self {
            inner: Arc::new(client),
        }
    }
}

impl<P> std::ops::Deref for SharedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    type Target = Client<P>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Client<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    inner: RawClient<P>,
}

impl<P> Client<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn new(url: String, signer: P) -> Client<P> {
        let api = Api::new(url).unwrap().set_signer(signer);
        Client {
            inner: RawClient { api },
        }
    }

    pub fn create_twin(&self, ip: &str) -> ApiResult<Option<Hash>> {
        let mut res = self.inner.create_twin(ip);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.create_twin(ip);
        }

        res
    }

    pub fn get_twin_by_id(&self, id: u32) -> ApiResult<Twin> {
        let mut res = self.inner.get_twin_by_id(id);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_twin_by_id(id);
        }

        res
    }

    pub fn create_farm(&self, name: &str) -> ApiResult<Option<Hash>> {
        let mut res = self.inner.create_farm(name);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.create_farm(name);
        }

        res
    }

    pub fn get_farm_by_id(&self, id: u32, block: Option<Hash>) -> ApiResult<Option<Farm>> {
        let mut res = self.inner.get_farm_by_id(id, block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_farm_by_id(id, block);
        }

        res
    }

    pub fn get_farm_id_by_name(&self, name: &str) -> ApiResult<u32> {
        let mut res = self.inner.get_farm_id_by_name(name);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_farm_id_by_name(name);
        }

        res
    }

    pub fn farm_count(&self, block: Option<Hash>) -> ApiResult<u32> {
        let mut res = self.inner.farm_count(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.farm_count(block);
        }

        res
    }

    pub fn get_account_free_balance(&self, account: &AccountId32) -> ApiResult<AccountData> {
        let mut res = self.inner.get_account_free_balance(account);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_account_free_balance(account);
        }

        res
    }

    pub fn get_node_by_id(&self, node_id: u32) -> ApiResult<Node> {
        let mut res = self.inner.get_node_by_id(node_id);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_node_by_id(node_id);
        }

        res
    }

    pub fn get_contract_by_id(&self, contract_id: u64) -> ApiResult<Contract> {
        let mut res = self.inner.get_contract_by_id(contract_id);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_contract_by_id(contract_id);
        }

        res
    }

    pub fn get_block_by_hash(&self, block_hash: &str) -> ApiResult<Option<Block>> {
        let mut res = self.inner.get_block_by_hash(block_hash);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_block_by_hash(block_hash);
        }

        res
    }

    pub fn get_block_events(&self, block: Option<Hash>) -> ApiResult<Vec<TfchainEvent>> {
        let mut res = self.inner.get_block_events(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_block_events(block);
        }

        res
    }

    pub fn block_timestamp(&self, block: Option<Hash>) -> ApiResult<u64> {
        let mut res = self.inner.block_timestamp(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.block_timestamp(block);
        }

        res
    }

    pub fn get_hash_at_height(&self, height: BlockNumber) -> ApiResult<Option<Hash>> {
        let mut res = self.inner.get_hash_at_height(height);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_hash_at_height(height);
        }

        res
    }

    pub fn finalized_block_headers(&self) -> ApiResult<FinalizedHeadSubscription> {
        // TODO: what if subscription breaks
        let mut res = self.inner.finalized_block_headers();
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.finalized_block_headers();
        }

        res
    }
}

pub struct RawClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub api: Api<P>,
}

impl<P> RawClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn new(url: String, signer: P) -> RawClient<P> {
        let api = Api::new(url).unwrap().set_signer(signer);
        RawClient { api }
    }

    pub fn create_twin(&self, ip: &str) -> ApiResult<Option<Hash>> {
        let xt: UncheckedExtrinsicV4<_> =
            compose_extrinsic!(self.api.clone(), "TfgridModule", "create_twin", ip);
        self.api.send_extrinsic(xt.hex_encode(), XtStatus::Ready)
    }

    pub fn get_twin_by_id(&self, id: u32) -> ApiResult<Twin> {
        let twin: Twin = self
            .api
            .get_storage_map("TfgridModule", "Twins", id, None)
            .unwrap()
            .or_else(|| Some(Twin::default()))
            .unwrap();

        Ok(twin)
    }

    pub fn create_farm(&self, name: &str) -> ApiResult<Option<Hash>> {
        let xt: UncheckedExtrinsicV4<_> =
            compose_extrinsic!(self.api.clone(), "TfgridModule", "create_farm", name);
        self.api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
    }

    pub fn get_farm_by_id(&self, id: u32, block: Option<Hash>) -> ApiResult<Option<Farm>> {
        self.api.get_storage_map("TfgridModule", "Farms", id, block)
    }

    pub fn get_farm_id_by_name(&self, name: &str) -> ApiResult<u32> {
        let farm_id: u32 = self
            .api
            .get_storage_map("TfgridModule", "FarmIdByName", name, None)
            .unwrap()
            .or_else(|| Some(0))
            .unwrap();

        Ok(farm_id)
    }

    pub fn farm_count(&self, block: Option<Hash>) -> ApiResult<u32> {
        // Safety: farmID is initialized in genesis so this value is always set.
        self.api
            .get_storage_value("TfgridModule", "FarmID", block)
            .map(|i| i.unwrap())
    }

    pub fn get_account_free_balance(&self, account: &AccountId32) -> ApiResult<AccountData> {
        let info: AccountInfo = self
            .api
            .get_storage_map("System", "Account", account, None)?
            .or_else(|| Some(AccountInfo::default()))
            .unwrap();

        Ok(info.data)
    }

    pub fn get_node_by_id(&self, node_id: u32) -> ApiResult<Node> {
        let node = self
            .api
            .get_storage_map("TfgridModule", "Nodes", node_id, None)?
            .or_else(|| Some(Node::default()))
            .unwrap();

        Ok(node)
    }

    pub fn get_contract_by_id(&self, contract_id: u64) -> ApiResult<Contract> {
        let contract = self
            .api
            .get_storage_map("SmartContractModule", "Contracts", contract_id, None)?
            .or_else(|| Some(Contract::default()))
            .unwrap();

        Ok(contract)
    }

    pub fn get_block_by_hash(&self, block_hash: &str) -> ApiResult<Option<Block>> {
        // TODO: Very happy path
        let mut raw_hash = [0; 32];
        hex::decode_to_slice(&block_hash[2..], &mut raw_hash).unwrap();
        let hash = Hash::from(raw_hash);
        self.api.get_block(Some(hash))
    }

    pub fn get_block_events(&self, block: Option<Hash>) -> ApiResult<Vec<TfchainEvent>> {
        let events: Vec<system::EventRecord<runtime::Event, Hash>> = self
            .api
            .get_storage_value("System", "Events", block)?
            .unwrap();

        Ok(events
            .into_iter()
            .map(|e| TfchainEvent::from(e.event))
            .collect())
    }

    pub fn block_timestamp(&self, block: Option<Hash>) -> ApiResult<u64> {
        Ok(self
            .api
            .get_storage_value("Timestamp", "Now", block)?
            .unwrap())
    }

    pub fn get_hash_at_height(&self, height: BlockNumber) -> ApiResult<Option<Hash>> {
        let req = substrate_api_client::rpc::json_req::chain_get_block_hash(Some(height));
        let resp = self.api.get_request(req.to_string())?;
        match resp {
            None => Ok(None),
            Some(hash_str) => {
                let mut raw_hash = [0; 32];
                // TODO: this could be improved
                hex::decode_to_slice(&hash_str[3..67], &mut raw_hash).unwrap();
                Ok(Some(Hash::from(raw_hash)))
            }
        }
    }

    pub fn finalized_block_headers(&self) -> ApiResult<FinalizedHeadSubscription> {
        let (heads_in, heads_out) = mpsc::channel();
        self.api.subscribe_finalized_heads(heads_in)?;

        Ok(FinalizedHeadSubscription { stream: heads_out })
    }
}

pub struct FinalizedHeadSubscription {
    stream: mpsc::Receiver<String>,
}

impl Iterator for FinalizedHeadSubscription {
    type Item = runtime::Header;

    fn next(&mut self) -> Option<Self::Item> {
        let header_str = self.stream.recv().unwrap();
        Some(serde_json::from_str(&header_str).unwrap())
    }
}
