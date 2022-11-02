use super::runtime::api::runtime_types::pallet_tfgrid::{
    twin::TwinIp as RuntimeTwinIP,
    types::{EntityProof as RuntimeEntityProof, Twin as RuntimeTwin},
};
use crate::types::{EntityProof, Twin};
use subxt::ext::sp_runtime::AccountId32;

impl From<RuntimeTwin<RuntimeTwinIP, AccountId32>> for Twin {
    fn from(rt: RuntimeTwin<RuntimeTwinIP, AccountId32>) -> Self {
        let RuntimeTwin {
            version,
            id,
            account_id,
            ip,
            entities,
        } = rt;
        Twin {
            version,
            id,
            account_id: account_id.into(),
            // SAFETY: all on chain IP's are verified to be properly formatted as strings.
            ip: String::from_utf8(ip.0 .0).unwrap().parse().unwrap(),
            entities: entities.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<RuntimeEntityProof> for EntityProof {
    fn from(rtep: RuntimeEntityProof) -> Self {
        let RuntimeEntityProof {
            entity_id,
            signature,
        } = rtep;
        EntityProof {
            entity_id,
            // SAFETY: signatures are always 64 bytes in the current allowed signature schemes.
            signature: signature.try_into().unwrap(),
        }
    }
}
