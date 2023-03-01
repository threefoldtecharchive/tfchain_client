pub use super::runtime::api::runtime_types::{
    frame_support::storage::bounded_vec::BoundedVec,
    pallet_smart_contract::types::{
        Cause as RuntimeCause, Contract as RuntimeContract, ContractData as RuntimeContractData,
        ContractResources as RuntimeContractResources, ContractState as RuntimeContractState,
        NameContract as RuntimeNameContract, NodeContract as RuntimeNodeContract,
        NruConsumption as RuntimeNruResources, RentContract as RuntimeRentContract,
    },
    pallet_tfgrid::{
        farm::FarmName as RuntimeFarmName,
        interface::{
            InterfaceIp as RuntimeInterfaceIp, InterfaceMac as RuntimeInterfaceMac,
            InterfaceName as RuntimeInterfaceName,
        },
        pub_config::{
            Domain as RuntimeDomain, GW4 as RuntimeGW4, GW6 as RuntimeGW6, IP4 as RuntimeIP4,
            IP6 as RuntimeIP6,
        },
        pub_ip::{GatewayIP as RuntimeGatewayIP, PublicIP as RuntimePublicIP},
        twin::TwinIp as RuntimeTwinIP,
        types::{
            EntityProof as RuntimeEntityProof, FarmingPolicy as RuntimeFarmingPolicy,
            Twin as RuntimeTwin,
        },
    },
    tfchain_support::types::{
        Farm as RuntimeFarm, FarmCertification as RuntimeFarmCertification,
        FarmingPolicyLimit as RuntimeFarmingPolicyLimit, Interface as RuntimeInterface,
        Location as RuntimeLocation, Node as RuntimeNode,
        NodeCertification as RuntimeNodeCertification, PublicConfig as RuntimePublicConfig,
        PublicIP as RuntimePublicIPGroup, Resources as RuntimeResources, IP as RuntimeIP,
    },
};
use crate::types::{
    Cause, Contract, ContractData, ContractResources, ContractState, Domain, EntityProof, Farm,
    FarmCertification, FarmPolicy, FarmingPolicyLimit, Interface, Location, NameContract, Node,
    NodeCertification, NodeContract, NruConsumption, PubIPConfig, PublicConfig, PublicIP,
    RentContract, Resources, Twin,
};
use subxt::utils::AccountId32;

pub type V115Twin = RuntimeTwin<RuntimeTwinIP, AccountId32>;
pub type V115Farm =
    RuntimeFarm<RuntimeFarmName, RuntimePublicIPGroup<RuntimePublicIP, RuntimeGatewayIP>>;
pub type V115Node = RuntimeNode<
    RuntimePublicConfig<
        RuntimeIP<RuntimeIP4, RuntimeGW4>,
        Option<RuntimeIP<RuntimeIP6, RuntimeGW6>>,
        Option<RuntimeDomain>,
    >,
    RuntimeInterface<RuntimeInterfaceName, RuntimeInterfaceMac, BoundedVec<RuntimeInterfaceIp>>,
>;
pub type V115Contract = RuntimeContract;
pub type V115ContractResources = RuntimeContractResources;
pub type V115FarmingPolicy = RuntimeFarmingPolicy<u32>;

pub type V115NodeStoredEvent = super::runtime::api::tfgrid_module::events::NodeStored;
pub type V115NodeUpdatedEvent = super::runtime::api::tfgrid_module::events::NodeUpdated;
pub type V115NodeUptimeReportedEvent =
    super::runtime::api::tfgrid_module::events::NodeUptimeReported;
pub type V115ContractCreatedEvent =
    super::runtime::api::smart_contract_module::events::ContractCreated;
pub type V115ContractUpdatedResourcesEvent =
    super::runtime::api::smart_contract_module::events::UpdatedUsedResources;
pub type V115ContractNruConsumptionReceivedEvent =
    super::runtime::api::smart_contract_module::events::NruConsumptionReportReceived;

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
            account_id,
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

impl From<RuntimeFarmingPolicy<u32>> for FarmPolicy {
    fn from(rfp: RuntimeFarmingPolicy<u32>) -> Self {
        let RuntimeFarmingPolicy {
            version,
            id,
            name,
            cu,
            su,
            nu,
            ipv4,
            minimal_uptime,
            policy_created,
            policy_end,
            immutable,
            default,
            node_certification,
            farm_certification,
        } = rfp;
        FarmPolicy {
            version,
            id,
            // SAFETY: Chain ensures this can only be valid ASCII.
            name: unsafe { String::from_utf8_unchecked(name) },
            cu,
            su,
            nu,
            ipv4,
            minimal_uptime,
            policy_created,
            policy_end,
            immutable,
            default,
            node_certification: node_certification.into(),
            farm_certification: farm_certification.into(),
        }
    }
}

impl From<RuntimeNodeCertification> for NodeCertification {
    fn from(rnc: RuntimeNodeCertification) -> Self {
        match rnc {
            RuntimeNodeCertification::Certified => NodeCertification::Certified,
            RuntimeNodeCertification::Diy => NodeCertification::Diy,
        }
    }
}

impl
    From<
        RuntimeNode<
            RuntimePublicConfig<
                RuntimeIP<RuntimeIP4, RuntimeGW4>,
                Option<RuntimeIP<RuntimeIP6, RuntimeGW6>>,
                Option<RuntimeDomain>,
            >,
            RuntimeInterface<
                RuntimeInterfaceName,
                RuntimeInterfaceMac,
                BoundedVec<RuntimeInterfaceIp>,
            >,
        >,
    > for Node
{
    fn from(
        rn: RuntimeNode<
            RuntimePublicConfig<
                RuntimeIP<RuntimeIP4, RuntimeGW4>,
                Option<RuntimeIP<RuntimeIP6, RuntimeGW6>>,
                Option<RuntimeDomain>,
            >,
            RuntimeInterface<
                RuntimeInterfaceName,
                RuntimeInterfaceMac,
                BoundedVec<RuntimeInterfaceIp>,
            >,
        >,
    ) -> Self {
        let RuntimeNode {
            version,
            id,
            farm_id,
            twin_id,
            resources,
            location,
            country,
            city,
            public_config,
            created,
            farming_policy_id,
            interfaces,
            certification,
            secure_boot,
            virtualized,
            serial_number,
            connection_price,
        } = rn;
        Node {
            version,
            id,
            farm_id,
            twin_id,
            resources: resources.into(),
            location: location.into(),
            // SAFETY: Chain ensures this is a valid ASCII string
            country: unsafe { String::from_utf8_unchecked(country) },
            // SAFETY: Chain ensures this is a valid ASCII string
            city: unsafe { String::from_utf8_unchecked(city) },
            public_config: public_config.map(|pc| pc.into()),
            created,
            farming_policy_id,
            interfaces: interfaces.into_iter().map(|i| i.into()).collect(),
            certification: certification.into(),
            secure_boot,
            virtualized,
            // SAFETY: Chain esures this is a valid ASCII string
            serial_number: unsafe { String::from_utf8_unchecked(serial_number) },
            connection_price,
        }
    }
}

impl From<RuntimeLocation> for Location {
    fn from(rl: RuntimeLocation) -> Self {
        let RuntimeLocation {
            longitude,
            latitude,
        } = rl;
        Location {
            // SAFETY: Chain ensures this is a valid ASCII string.
            longitude: unsafe { String::from_utf8_unchecked(longitude) },
            // SAFETY: Chain ensures this is a valid ASCII string.
            latitude: unsafe { String::from_utf8_unchecked(latitude) },
        }
    }
}

impl From<RuntimeResources> for Resources {
    fn from(rl: RuntimeResources) -> Self {
        let RuntimeResources { hru, sru, cru, mru } = rl;
        Resources { hru, sru, cru, mru }
    }
}

impl
    From<
        RuntimePublicConfig<
            RuntimeIP<RuntimeIP4, RuntimeGW4>,
            Option<RuntimeIP<RuntimeIP6, RuntimeGW6>>,
            Option<RuntimeDomain>,
        >,
    > for PublicConfig
{
    fn from(
        rpc: RuntimePublicConfig<
            RuntimeIP<RuntimeIP4, RuntimeGW4>,
            Option<RuntimeIP<RuntimeIP6, RuntimeGW6>>,
            Option<RuntimeDomain>,
        >,
    ) -> Self {
        let RuntimePublicConfig { ip4, ip6, domain } = rpc;
        PublicConfig {
            ip4: ip4.into(),
            ip6: ip6.map(|i| i.into()),
            domain: domain.map(|d| d.into()),
        }
    }
}

impl From<RuntimeIP<RuntimeIP4, RuntimeGW4>> for PubIPConfig {
    fn from(rip: RuntimeIP<RuntimeIP4, RuntimeGW4>) -> Self {
        let RuntimeIP { ip, gw } = rip;
        PubIPConfig {
            // SAFETY: Chain ensures this is a valid ASCII string.
            ip: unsafe { String::from_utf8_unchecked(ip.0 .0) },
            // SAFETY: Chain ensures this is a valid ASCII string.
            gw: unsafe { String::from_utf8_unchecked(gw.0 .0) },
        }
    }
}

impl From<RuntimeIP<RuntimeIP6, RuntimeGW6>> for PubIPConfig {
    fn from(rip: RuntimeIP<RuntimeIP6, RuntimeGW6>) -> Self {
        let RuntimeIP { ip, gw } = rip;
        PubIPConfig {
            // SAFETY: Chain ensures this is a valid ASCII string.
            ip: unsafe { String::from_utf8_unchecked(ip.0 .0) },
            // SAFETY: Chain ensures this is a valid ASCII string.
            gw: unsafe { String::from_utf8_unchecked(gw.0 .0) },
        }
    }
}

impl From<RuntimeDomain> for Domain {
    fn from(rd: RuntimeDomain) -> Self {
        let RuntimeDomain(d) = rd;
        // SAFETY: Chain ensures this is a valid ASCII string.
        Domain(unsafe { String::from_utf8_unchecked(d.0) })
    }
}

impl
    From<
        RuntimeInterface<RuntimeInterfaceName, RuntimeInterfaceMac, BoundedVec<RuntimeInterfaceIp>>,
    > for Interface
{
    fn from(
        ri: RuntimeInterface<
            RuntimeInterfaceName,
            RuntimeInterfaceMac,
            BoundedVec<RuntimeInterfaceIp>,
        >,
    ) -> Self {
        let RuntimeInterface { name, mac, ips } = ri;
        Interface {
            // SAFETY: Chain ensures this is a valid ASCII string.
            name: unsafe { String::from_utf8_unchecked(name.0 .0) },
            // SAFETY: Chain ensures this is a valid ASCII string.
            mac: unsafe { String::from_utf8_unchecked(mac.0 .0) },
            ips: ips
                .0
                .into_iter()
                // SAFETY: Chain ensures this is a valid ASCII string.
                .map(|ip| unsafe { String::from_utf8_unchecked(ip.0 .0) })
                .collect(),
        }
    }
}

impl From<RuntimeContract> for Contract {
    fn from(rc: RuntimeContract) -> Self {
        let RuntimeContract {
            version,
            state,
            contract_id,
            twin_id,
            contract_type,
            solution_provider_id,
        } = rc;
        Contract {
            version,
            state: state.into(),
            contract_id,
            twin_id,
            contract_type: contract_type.into(),
            solution_provider_id,
        }
    }
}

impl From<RuntimeContractState> for ContractState {
    fn from(rcs: RuntimeContractState) -> Self {
        match rcs {
            RuntimeContractState::Created => ContractState::Created,
            RuntimeContractState::Deleted(cause) => ContractState::Deleted(cause.into()),
            RuntimeContractState::GracePeriod(gp) => ContractState::GracePeriod(gp),
        }
    }
}

impl From<RuntimeCause> for Cause {
    fn from(rc: RuntimeCause) -> Self {
        match rc {
            RuntimeCause::OutOfFunds => Cause::OutOfFunds,
            RuntimeCause::CanceledByUser => Cause::CanceledByUser,
        }
    }
}

impl From<RuntimeContractData> for ContractData {
    fn from(rcd: RuntimeContractData) -> Self {
        match rcd {
            RuntimeContractData::NodeContract(nc) => ContractData::NodeContract(nc.into()),
            RuntimeContractData::NameContract(nc) => ContractData::NameContract(nc.into()),
            RuntimeContractData::RentContract(rc) => ContractData::RentContract(rc.into()),
        }
    }
}

impl From<RuntimeNodeContract> for NodeContract {
    fn from(rnc: RuntimeNodeContract) -> Self {
        let RuntimeNodeContract {
            node_id,
            deployment_hash,
            deployment_data,
            public_ips,
            public_ips_list,
        } = rnc;
        NodeContract {
            node_id,
            deployment_hash,
            deployment_data: deployment_data.0,
            public_ips,
            public_ips_list: public_ips_list
                .0
                .into_iter()
                .map(|pip| pip.into())
                .collect(),
        }
    }
}

impl From<RuntimeNameContract> for NameContract {
    fn from(rnc: RuntimeNameContract) -> Self {
        let RuntimeNameContract { name } = rnc;
        NameContract {
            // SAFETY: Chain ensures this is a valid ASCII string.
            name: unsafe { String::from_utf8_unchecked(name.0 .0) },
        }
    }
}

impl From<RuntimeRentContract> for RentContract {
    fn from(rrc: RuntimeRentContract) -> Self {
        let RuntimeRentContract { node_id } = rrc;
        RentContract { node_id }
    }
}

impl From<RuntimeContractResources> for ContractResources {
    fn from(rcr: RuntimeContractResources) -> Self {
        let RuntimeContractResources { contract_id, used } = rcr;
        ContractResources {
            contract_id,
            used: used.into(),
        }
    }
}

impl From<RuntimeNruResources> for NruConsumption {
    fn from(rcr: RuntimeNruResources) -> Self {
        let RuntimeNruResources {
            contract_id,
            timestamp,
            window,
            nru,
        } = rcr;
        NruConsumption {
            contract_id,
            timestamp,
            window,
            nru,
        }
    }
}
