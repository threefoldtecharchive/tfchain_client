use runtime::Block;
pub use sp_core::crypto::AccountId32;
use sp_core::crypto::Pair;
pub use sp_core::H256 as Hash;
use substrate_api_client::sp_runtime::MultiSignature;
use substrate_api_client::{
    compose_extrinsic, Api, ApiClientError, UncheckedExtrinsicV4, XtStatus,
};

type ApiResult<T> = Result<T, ApiClientError>;

pub mod events;
mod types;
use types::{AccountData, AccountInfo, Contract, Farm, Node, Twin};

pub struct Client<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub api: Api<P>,
}

impl<P> Client<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn new(url: String, signer: P) -> Client<P> {
        let api = Api::new(url).unwrap().set_signer(signer);
        Client { api }
    }

    pub fn create_twin(&self, ip: String) -> ApiResult<Option<Hash>> {
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

    pub fn create_farm(&self, name: String) -> ApiResult<Option<Hash>> {
        let xt: UncheckedExtrinsicV4<_> =
            compose_extrinsic!(self.api.clone(), "TfgridModule", "create_farm", name);
        self.api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
    }

    pub fn get_farm_by_id(&self, id: u32) -> ApiResult<Farm> {
        let farm: Farm = self
            .api
            .get_storage_map("TfgridModule", "Farms", id, None)
            .unwrap()
            .or_else(|| Some(Farm::default()))
            .unwrap();

        Ok(farm)
    }

    pub fn get_farm_id_by_name(&self, name: String) -> ApiResult<u32> {
        let farm_id: u32 = self
            .api
            .get_storage_map("TfgridModule", "FarmIdByName", name, None)
            .unwrap()
            .or_else(|| Some(0))
            .unwrap();

        Ok(farm_id)
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
        Ok(self.api.get_block(Some(hash))?)
    }

    pub fn get_block_events(
        &self,
        block_hash: &str,
    ) -> ApiResult<Vec<system::EventRecord<runtime::Event, Hash>>> {
        let mut raw_hash = [0; 32];
        hex::decode_to_slice(&block_hash[2..], &mut raw_hash).unwrap();
        let hash = Hash::from(raw_hash);
        let events: Vec<system::EventRecord<runtime::Event, Hash>> = self
            .api
            .get_storage_value("System", "Events", Some(hash))?
            .unwrap();

        Ok(events)
    }
}
