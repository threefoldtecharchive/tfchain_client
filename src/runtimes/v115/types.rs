use super::runtime::api::runtime_types::{
    pallet_tfgrid::{
        farm::FarmName as RuntimeFarmName,
        pub_ip::{GatewayIP as RuntimeGatewayIP, PublicIP as RuntimePublicIP},
        twin::TwinIp as RuntimeTwinIP,
        types::{EntityProof as RuntimeEntityProof, Twin as RuntimeTwin},
    },
    tfchain_support::types::{
        Farm as RuntimeFarm, FarmCertification as RuntimeFarmCertification,
        FarmingPolicyLimit as RuntimeFarmingPolicyLimit, PublicIP as RuntimePublicIPGroup,
    },
};
use crate::types::{EntityProof, Farm, FarmCertification, FarmingPolicyLimit, PublicIP, Twin};
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
            ip: unsafe { String::from_utf8_unchecked(ip.0 .0) }
                .parse()
                .unwrap(),
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

impl From<RuntimeFarm<RuntimeFarmName, RuntimePublicIPGroup<RuntimePublicIP, RuntimeGatewayIP>>>
    for Farm
{
    fn from(
        rf: RuntimeFarm<RuntimeFarmName, RuntimePublicIPGroup<RuntimePublicIP, RuntimeGatewayIP>>,
    ) -> Self {
        let RuntimeFarm {
            version,
            id,
            name,
            twin_id,
            pricing_policy_id,
            certification,
            public_ips,
            dedicated_farm,
            farming_policy_limits,
        } = rf;
        Farm {
            version,
            id,
            // SAFETY: Chain ensures this is valid utf8
            name: unsafe { String::from_utf8_unchecked(name.0 .0) },
            twin_id,
            pricing_policy_id,
            certification: certification.into(),
            public_ips: public_ips.0.into_iter().map(|rpi| rpi.into()).collect(),
            dedicated_farm,
            farming_policy_limits: farming_policy_limits.map(|fpl| fpl.into()),
        }
    }
}

impl From<RuntimeFarmCertification> for FarmCertification {
    fn from(rfc: RuntimeFarmCertification) -> Self {
        match rfc {
            RuntimeFarmCertification::Gold => FarmCertification::Gold,
            RuntimeFarmCertification::NotCertified => FarmCertification::NotCertified,
        }
    }
}

impl From<RuntimeFarmingPolicyLimit> for FarmingPolicyLimit {
    fn from(rfpl: RuntimeFarmingPolicyLimit) -> Self {
        let RuntimeFarmingPolicyLimit {
            farming_policy_id,
            cu,
            su,
            end,
            node_count,
            node_certification,
        } = rfpl;
        FarmingPolicyLimit {
            farming_policy_id,
            cu,
            su,
            end,
            node_count,
            node_certification,
        }
    }
}

impl From<RuntimePublicIPGroup<RuntimePublicIP, RuntimeGatewayIP>> for PublicIP {
    fn from(rpip: RuntimePublicIPGroup<RuntimePublicIP, RuntimeGatewayIP>) -> Self {
        let RuntimePublicIPGroup {
            ip,
            gateway,
            contract_id,
        } = rpip;
        PublicIP {
            // SAFETY: Chain ensures all IP's are validly formatted ASCII strings.
            ip: unsafe { String::from_utf8_unchecked(ip.0 .0) },
            // SAFETY: Chain ensures all IP's are validly formatted ASCII strings.
            gateway: unsafe { String::from_utf8_unchecked(gateway.0 .0) },
            contract_id,
        }
    }
}
