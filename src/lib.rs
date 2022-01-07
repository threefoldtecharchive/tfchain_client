use sp_core::crypto::{Pair};
use substrate_api_client::sp_runtime::{MultiSignature};
use substrate_api_client::{compose_extrinsic, Api, UncheckedExtrinsicV4, XtStatus, ApiClientError};
pub use sp_core::H256 as Hash;
use sp_core::crypto::AccountId32;

type ApiResult<T> = Result<T, ApiClientError>;

mod types;
use types::{Twin, Farm};

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
        println!("[+] Composed Extrinsic:\n {:?}\n", xt);
        self.api
            .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
    }

    pub fn get_twin_by_id(&self, id: u32) -> ApiResult<Twin<AccountId32>> {
        let twin: Twin<AccountId32> = self.api
            .get_storage_map("TfgridModule", "Twins", id, None)
                .unwrap()
                .or_else(|| Some(Twin::default()))
                .unwrap();

        Ok(twin)
    }

    pub fn get_farm_by_id(&self, id: u32) -> ApiResult<Farm> {
        let farm: Farm = self.api
            .get_storage_map("TfgridModule", "Farms", id, None)
                .unwrap()
                .or_else(|| Some(Farm::default()))
                .unwrap();

        Ok(farm)
    }

    pub fn get_farm_id_by_name(&self, name: String) -> ApiResult<u32> {
        let farm_id: u32 = self.api
            .get_storage_map("TfgridModule", "FarmIdByName", name, None)
                .unwrap()
                .or_else(|| Some(0))
                .unwrap();

        Ok(farm_id)
    }
}
