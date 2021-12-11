use sp_core::crypto::{Pair};
use substrate_api_client::sp_runtime::{MultiSignature};
use substrate_api_client::{compose_extrinsic, Api, UncheckedExtrinsicV4, XtStatus, ApiClientError};
pub use sp_core::H256 as Hash;

type ApiResult<T> = Result<T, ApiClientError>;

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
}
