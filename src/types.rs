use std::net::IpAddr;
/// The hash type used on the TfChain.
pub type Hash = subxt::utils::H256;
use subxt::utils::AccountId32;
/// Public Key type, this is a placeholder.
pub type PublicKey = [u8; 32];
/// Signature type, this is a placeholder.
pub type Signature = [u8; 64];
pub type BlockNumber = subxt::rpc::types::BlockNumber;

pub const TFGRID_MODULE: &str = "TfgridModule";
pub const NODE_STORED: &str = "NodeStored";
pub const NODE_UPDATED: &str = "NodeUpdated";
pub const NODE_UPTIME_REPORTED: &str = "NodeUptimeReported";
pub const FARMING_POLICIES: &str = "FarmingPoliciesMap";
pub const FARMING_POLICY_ID: &str = "FarmingPolicyID";
pub const NODE_ID: &str = "NodeID";
pub const NODES: &str = "Nodes";
pub const FARM_ID: &str = "FarmID";
pub const FARM_PAYOUT_V2_ADDRESS: &str = "FarmPayoutV2AddressByFarmID";
pub const TWINS: &str = "Twins";
pub const TWIN_ID: &str = "TwinID";
pub const FARMS: &str = "Farms";

pub const SMART_CONTRACT_MODULE: &str = "SmartContractModule";
pub const UPDATE_USED_RESOURCES: &str = "UpdatedUsedResources";
pub const NRU_CONSUMPTION_RECEIVED: &str = "NruConsumptionReportReceived";
pub const CONTRACT_CREATED: &str = "ContractCreated";
pub const NODE_CONTRACT_CANCELLED: &str = "NodeContractCanceled";
pub const NODE_CONTRACT_RESOURCES: &str = "NodeContractResources";
pub const CONTRACT_ID: &str = "ContractID";
pub const CONTRACTS: &str = "Contracts";

pub const TIMESTAMP_MODULE: &str = "Timestamp";
pub const TIMESTAMP_NOW: &str = "Now";

#[derive(Debug, Clone)]
pub struct Twin {
    pub version: u32,
    pub id: u32,
    // TODO: proper typing
    pub account_id: AccountId32,
    pub ip: IpAddr,
    pub entities: Vec<EntityProof>,
}

#[derive(Debug, Clone)]
pub struct Farm {
    pub version: u32,
    pub id: u32,
    pub name: String,
    pub twin_id: u32,
    pub pricing_policy_id: u32,
    pub certification: FarmCertification,
    pub public_ips: Vec<PublicIP>,
    pub dedicated_farm: bool,
    pub farming_policy_limits: Option<FarmingPolicyLimit>,
}

#[derive(Debug, Clone)]
pub struct PublicIP {
    // Not ideal but there is no type in std to represent IP+subnet currently, could make this a
    // tuple of (IP, mask).
    pub ip: String,
    // String to keep in line with the above.
    pub gateway: String,
    pub contract_id: u64,
}

#[derive(Debug, Clone)]
pub struct FarmingPolicyLimit {
    pub farming_policy_id: u32,
    pub cu: Option<u64>,
    pub su: Option<u64>,
    pub end: Option<u64>,
    pub node_count: Option<u32>,
    pub node_certification: bool,
}

#[derive(Debug, Clone)]
pub enum FarmCertification {
    Gold,
    NotCertified,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub version: u32,
    pub id: u32,
    pub farm_id: u32,
    pub twin_id: u32,
    pub resources: Resources,
    pub location: Location,
    pub country: String,
    pub city: String,
    pub public_config: Option<PublicConfig>,
    pub created: u64,
    pub farming_policy_id: u32,
    pub interfaces: Vec<Interface>,
    pub certification: NodeCertification,
    pub secure_boot: bool,
    pub virtualized: bool,
    pub serial_number: String,
    pub connection_price: u32,
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    // This really should be a [u8;6], but the chain saves this as a string currently;
    pub mac: String,
    pub ips: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Resources {
    pub hru: u64,
    pub sru: u64,
    pub cru: u64,
    pub mru: u64,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub longitude: String,
    pub latitude: String,
}

#[derive(Debug, Clone)]
pub struct Contract {
    pub version: u32,
    pub state: ContractState,
    pub contract_id: u64,
    pub twin_id: u32,
    pub contract_type: ContractData,
    pub solution_provider_id: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum ContractState {
    Created,
    Deleted(Cause),
    GracePeriod(u64),
}

#[derive(Debug, Clone)]
pub enum Cause {
    CanceledByUser,
    OutOfFunds,
}

#[derive(Debug, Clone)]
pub enum ContractData {
    NodeContract(NodeContract),
    NameContract(NameContract),
    RentContract(RentContract),
}

#[derive(Debug, Clone)]
pub struct NodeContract {
    pub node_id: u32,
    pub deployment_hash: Hash,
    pub deployment_data: Vec<u8>,
    pub public_ips: u32,
    pub public_ips_list: Vec<PublicIP>,
}

#[derive(Debug, Clone)]
pub struct NameContract {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RentContract {
    pub node_id: u32,
}

#[derive(Debug, Clone)]
pub struct PublicConfig {
    pub ip4: PubIPConfig,
    pub ip6: Option<PubIPConfig>,
    pub domain: Option<Domain>,
}

#[derive(Debug, Clone)]
pub struct PubIPConfig {
    pub ip: String,
    pub gw: String,
}

#[derive(Debug, Clone)]
pub struct Domain(pub String);

#[derive(Debug, Clone)]
pub struct ContractResources {
    pub contract_id: u64,
    pub used: Resources,
}

#[derive(Debug, Clone)]
pub struct EntityProof {
    pub entity_id: u32,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct FarmPolicy {
    pub version: u32,
    pub id: u32,
    pub name: String,
    pub cu: u32,
    pub su: u32,
    pub nu: u32,
    pub ipv4: u32,
    pub minimal_uptime: u16,
    pub policy_created: u32,
    pub policy_end: u32,
    pub immutable: bool,
    pub default: bool,
    pub node_certification: NodeCertification,
    pub farm_certification: FarmCertification,
}

#[derive(Debug, Clone)]
pub enum NodeCertification {
    Certified,
    Diy,
}

#[derive(Debug, Clone)]
pub struct NruConsumption {
    pub contract_id: u64,
    pub timestamp: u64,
    pub window: u64,
    pub nru: u64,
}

#[derive(Debug, Clone)]
pub enum RuntimeEvents {
    NodeStoredEvent(Node),
    NodeUpdatedEvent(Node),
    NodeUptimeReported(u32, u64, u64),
    ContractCreated(Contract),
    ContractUsedResourcesUpdated(ContractResources),
    NruConsumptionReceived(NruConsumption),
}
