use crate::types::{
    AccountId32, CertificationCodes, Entity, Farm, FarmCertification, FarmingPolicy,
    FarmingPolicyLimit, Node, NodeCertification, PricingPolicy, PublicConfig, Twin,
};

#[derive(Debug)]
pub enum Event {
    FarmStored(Farm),
    FarmUpdated(Farm),
    FarmDeleted(u32),

    NodeStored(Node),
    NodeUpdated(Node),
    NodeDeleted(u32),
    NodeUptimeReported(u32, u64, u64),
    NodePublicConfigStored(u32, PublicConfig),

    EntityStored(Entity),
    EntityUpdated(Entity),
    EntityDeleted(u32),

    TwinStored(Twin),
    TwinUpdated(Twin),

    TwinEntityStored(u32, u32, Vec<u8>),
    TwinEntityRemoved(u32, u32),
    TwinDeleted(u32),

    PricingPolicyStored(PricingPolicy),
    CertificationCodeStored(CertificationCodes),
    FarmingPolicyStored(FarmingPolicy<u32>),
    FarmPayoutV2AddressRegistered(u32, Vec<u8>),

    /// Farm has been marked as dedicated \[farm id\]
    FarmMarkedAsDedicated(u32),

    ConnectionPriceSet(u32),
    NodeCertificationSet(u32, NodeCertification),
    NodeCertifierAdded(AccountId32),
    NodeCertifierRemoved(AccountId32),
    FarmingPolicyUpdated(FarmingPolicy<u32>),
    FarmingPolicySet(u32, Option<FarmingPolicyLimit>),
    FarmCertificationSet(u32, FarmCertification),
}

impl From<pallet_tfgrid::Event<runtime::Runtime>> for Event {
    fn from(tfge: pallet_tfgrid::Event<runtime::Runtime>) -> Self {
        match tfge {
            pallet_tfgrid::Event::<runtime::Runtime>::FarmStored(farm) => {
                Event::FarmStored(farm.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::FarmUpdated(farm) => {
                Event::FarmUpdated(farm.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::FarmDeleted(id) => Event::FarmDeleted(id),
            pallet_tfgrid::Event::<runtime::Runtime>::NodeStored(node) => {
                Event::NodeStored(node.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::NodeUpdated(node) => {
                Event::NodeUpdated(node.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::NodeDeleted(id) => Event::NodeDeleted(id),
            pallet_tfgrid::Event::<runtime::Runtime>::NodeUptimeReported(id, time, uptime) => {
                Event::NodeUptimeReported(id, time, uptime)
            }
            pallet_tfgrid::Event::<runtime::Runtime>::NodePublicConfigStored(id, config) => {
                Event::NodePublicConfigStored(id, config.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::EntityStored(entity) => {
                Event::EntityStored(entity.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::EntityUpdated(entity) => {
                Event::EntityUpdated(entity.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::EntityDeleted(id) => Event::EntityDeleted(id),
            pallet_tfgrid::Event::<runtime::Runtime>::TwinStored(twin) => {
                Event::TwinStored(twin.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::TwinUpdated(twin) => {
                Event::TwinUpdated(twin.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::TwinEntityStored(twinid, entityid, sig) => {
                Event::TwinEntityStored(twinid, entityid, sig)
            }
            pallet_tfgrid::Event::<runtime::Runtime>::TwinEntityRemoved(twinid, entityid) => {
                Event::TwinEntityRemoved(twinid, entityid)
            }
            pallet_tfgrid::Event::<runtime::Runtime>::TwinDeleted(id) => Event::TwinDeleted(id),
            pallet_tfgrid::Event::<runtime::Runtime>::PricingPolicyStored(policy) => {
                Event::PricingPolicyStored(policy.into())
            }
            // pallet_tfgrid::Event::<runtime::Runtime>::CertificationCodeStored(cc) => {
            //     Event::CertificationCodeStored(cc.into())
            // }
            pallet_tfgrid::Event::<runtime::Runtime>::FarmingPolicyStored(fp) => {
                Event::FarmingPolicyStored(fp.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::FarmPayoutV2AddressRegistered(
                id,
                address,
            ) => Event::FarmPayoutV2AddressRegistered(id, address),
            pallet_tfgrid::Event::<runtime::Runtime>::FarmMarkedAsDedicated(farm_id) => {
                Event::FarmMarkedAsDedicated(farm_id)
            }
            pallet_tfgrid::Event::<runtime::Runtime>::ConnectionPriceSet(node_id) => {
                Event::ConnectionPriceSet(node_id)
            }
            pallet_tfgrid::Event::<runtime::Runtime>::NodeCertificationSet(node_id, nc) => {
                Event::NodeCertificationSet(node_id, nc.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::NodeCertifierAdded(who) => {
                Event::NodeCertifierAdded(who)
            }
            pallet_tfgrid::Event::<runtime::Runtime>::NodeCertifierRemoved(who) => {
                Event::NodeCertifierRemoved(who)
            }
            pallet_tfgrid::Event::<runtime::Runtime>::FarmingPolicyUpdated(policy) => {
                Event::FarmingPolicyUpdated(policy.into())
            }
            pallet_tfgrid::Event::<runtime::Runtime>::FarmingPolicySet(policy_id, limit) => {
                Event::FarmingPolicySet(policy_id, limit.map(FarmingPolicyLimit::from))
            }
            pallet_tfgrid::Event::<runtime::Runtime>::FarmCertificationSet(farm_id, cert) => {
                Event::FarmCertificationSet(farm_id, cert.into())
            }
        }
    }
}
