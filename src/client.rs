//! Low level client to interact with the chain. For upstream usage, other than constructing a
//! [Client], you likely want to look at the [window](crate::window) module.

use crate::events::TfchainEvent;
pub use crate::types::Hash;
use crate::types::{AccountData, AccountInfo, BlockNumber, Contract, Farm, Node, Twin};
use runtime::Block;
pub use sp_core::crypto::AccountId32;
use std::sync::mpsc;
use std::sync::Arc;
use substrate_api_client::{
    compose_extrinsic, Api, ApiClientError, UncheckedExtrinsicV4, XtStatus,
};

pub use sp_core::crypto::Pair;
pub use substrate_api_client::sp_runtime::MultiSignature;

const BLOCK_TIME_SECONDS: i64 = 6;

pub type ApiResult<T> = Result<T, ApiClientError>;

pub struct SharedClient<P, E>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    inner: Arc<Client<P, E>>,
}

impl<P, E> SharedClient<P, E>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn new(client: Client<P, E>) -> Self {
        Self {
            inner: Arc::new(client),
        }
    }

    pub fn with_events<U>(self) -> SharedClient<P, U>
    where
        U: support::sp_runtime::traits::Member + support::Parameter,
        TfchainEvent: From<U>,
    {
        // TODO: Improve this
        SharedClient {
            inner: Arc::new(Client {
                inner: RawClient {
                    api: self.inner.inner.api.clone(),
                    _marker: std::marker::PhantomData,
                },
            }),
        }
    }
}

impl<P, E> Clone for SharedClient<P, E>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<P, E> std::ops::Deref for SharedClient<P, E>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    type Target = Client<P, E>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Client<P, E>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    inner: RawClient<P, E>,
}

impl<P, E> Client<P, E>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    E: support::sp_runtime::traits::Member + support::Parameter,
    TfchainEvent: From<E>,
{
    pub fn new(url: String, signer: Option<P>) -> Client<P, E> {
        let mut api = Api::new(url).unwrap();
        if let Some(signer) = signer {
            api = api.set_signer(signer);
        }
        Client {
            inner: RawClient {
                api,
                _marker: std::marker::PhantomData,
            },
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

    pub fn get_node_by_id(&self, node_id: u32, block: Option<Hash>) -> ApiResult<Option<Node>> {
        let mut res = self.inner.get_node_by_id(node_id, block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_node_by_id(node_id, block);
        }

        res
    }

    pub fn node_count(&self, block: Option<Hash>) -> ApiResult<u32> {
        let mut res = self.inner.node_count(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.node_count(block);
        }

        res
    }

    pub fn get_contract_by_id(
        &self,
        contract_id: u64,
        block: Option<Hash>,
    ) -> ApiResult<Option<Contract>> {
        let mut res = self.inner.get_contract_by_id(contract_id, block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_contract_by_id(contract_id, block);
        }

        res
    }

    pub fn contract_count(&self, block: Option<Hash>) -> ApiResult<u64> {
        let mut res = self.inner.contract_count(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.contract_count(block);
        }

        res
    }

    pub fn get_farm_payout_address(
        &self,
        farm_id: u32,
        block: Option<Hash>,
    ) -> ApiResult<Option<String>> {
        let mut res = self.inner.get_farm_payout_address(farm_id, block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_farm_payout_address(farm_id, block);
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

    pub fn block_timestamp(&self, block: Option<Hash>) -> ApiResult<i64> {
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

    // Get the height just past the timestamp. i.e. `block_x_time | ts | block_x+1_time` returns
    // block x+1
    pub fn height_at_timestamp(&self, ts: i64) -> ApiResult<BlockNumber> {
        // TODO: clean these unwraps, this assumes block 1 always exists (which is the case for
        // now).
        // SAFETY: sanity check that ts is smaller than the last height.
        let latest_ts = self.block_timestamp(None)? / 1000;
        if latest_ts < ts {
            panic!(
                "can't fetch block for future timestamp {} vs latest {}",
                ts, latest_ts
            );
        }
        let mut height = 1;
        let mut last_height = 1;
        loop {
            let hash = match self.get_hash_at_height(height)? {
                Some(hash) => hash,
                // In case the network stalled we might be on a future block, try to fix that
                None => {
                    // Don't override last_height. That way we will incrementally approach
                    // last_height as we go
                    height = (height + last_height) / 2;
                    continue;
                }
            };
            // timestmap is in milliseconds
            let block_time = self.block_timestamp(Some(hash))? / 1000;
            let time_delta = ts - block_time;
            let block_delta = time_delta / BLOCK_TIME_SECONDS;
            if block_delta == 0 {
                if time_delta >= 0 {
                    // the timestamp is slightly before this block, so return the this block;
                    return Ok((height + 1) as u32);
                } else {
                    // the timestamp is slightly past this block, so return the next block;
                    return Ok(height as u32);
                }
            }
            // check that the delta is in range
            if (height as i64 + block_delta) < 0 {
                panic!(
                    "negative height search (height {} delta {})",
                    height, block_delta
                );
            }

            // adjust height
            last_height = height;
            // we can't just cast block_delta to u32 here, as that would misbehave in case delta is
            // negative
            height = (height as i64 + block_delta) as u32;
        }
    }
}

pub struct RawClient<P, E>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub api: Api<P>,
    _marker: std::marker::PhantomData<E>,
}

impl<P, E> RawClient<P, E>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    E: support::Parameter + sp_runtime::traits::Member,
    TfchainEvent: From<E>,
{
    pub fn new(url: String, signer: P) -> RawClient<P, E> {
        let api = Api::new(url).unwrap().set_signer(signer);
        RawClient {
            api,
            _marker: std::marker::PhantomData,
        }
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

    pub fn get_node_by_id(&self, node_id: u32, block: Option<Hash>) -> ApiResult<Option<Node>> {
        // Try to decode all known node types here.
        let res = self.api.get_storage_map::<_, pallet_tfgrid::types::Node>(
            "TfgridModule",
            "Nodes",
            node_id,
            block,
        );
        if res.is_ok() {
            return Ok(res.unwrap().map(Node::from));
        }
        self.api
            .get_storage_map::<_, pallet_tfgrid_legacy::types::Node>(
                "TfgridModule",
                "Nodes",
                node_id,
                block,
            )
            .map(|pr| pr.map(Node::from))
    }

    pub fn node_count(&self, block: Option<Hash>) -> ApiResult<u32> {
        // Safety: nodeID is initialized in genesis so this value is always set.
        self.api
            .get_storage_value("TfgridModule", "NodeID", block)
            .map(|i| i.unwrap())
    }

    pub fn get_contract_by_id(
        &self,
        contract_id: u64,
        block: Option<Hash>,
    ) -> ApiResult<Option<Contract>> {
        self.api
            .get_storage_map("SmartContractModule", "Contracts", contract_id, block)
    }

    pub fn contract_count(&self, block: Option<Hash>) -> ApiResult<u64> {
        // Safety: contractID is initialized in genesis so this value is always set.
        self.api
            .get_storage_value("SmartContractModule", "ContractID", block)
            .map(|i| i.unwrap_or(0))
    }

    pub fn get_farm_payout_address(
        &self,
        farm_id: u32,
        block: Option<Hash>,
    ) -> ApiResult<Option<String>> {
        self.api.get_storage_map(
            "TfgridModule",
            "FarmPayoutV2AddressByFarmID",
            farm_id,
            block,
        )
    }

    pub fn get_block_by_hash(&self, block_hash: &str) -> ApiResult<Option<Block>> {
        // TODO: Very happy path
        let mut raw_hash = [0; 32];
        hex::decode_to_slice(&block_hash[2..], &mut raw_hash).unwrap();
        let hash = Hash::from(raw_hash);
        self.api.get_block(Some(hash))
    }

    pub fn get_block_events(&self, block: Option<Hash>) -> ApiResult<Vec<TfchainEvent>> {
        let events: Vec<system::EventRecord<E, Hash>> = self
            .api
            .get_storage_value("System", "Events", block)?
            .unwrap();

        Ok(events
            .into_iter()
            .map(|e| TfchainEvent::from(e.event))
            .collect())
    }

    pub fn block_timestamp(&self, block: Option<Hash>) -> ApiResult<i64> {
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

/// A subscription on finalized heads. This iterator will never finish naturally. If it does it
/// indicates a receiving error, and the client should create a new subscription.
pub struct FinalizedHeadSubscription {
    stream: mpsc::Receiver<String>,
}

impl Iterator for FinalizedHeadSubscription {
    type Item = runtime::Header;

    fn next(&mut self) -> Option<Self::Item> {
        let header_str = self.stream.recv().ok()?;
        Some(serde_json::from_str(&header_str).unwrap())
    }
}
