use std::net::IpAddr;

/// The hash type used on the TfChain.
pub type Hash = subxt::ext::sp_core::H256;

/// Public Key type, this is a placeholder.
pub type PublicKey = [u8; 32];
/// Signature type, this is a placeholder.
pub type Signature = [u8; 64];

pub struct Twin {
    pub version: u32,
    pub id: u32,
    // TODO: proper typing
    pub account_id: PublicKey,
    pub ip: IpAddr,
    pub entities: Vec<EntityProof>,
}

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

pub struct PublicIP {
    // Not ideal but there is no type in std to represent IP+subnet currently, could make this a
    // tuple of (IP, mask).
    pub ip: String,
    // String to keep in line with the above.
    pub gateway: String,
    pub contract_id: u64,
}

pub struct FarmingPolicyLimit {
    pub farming_policy_id: u32,
    pub cu: Option<u64>,
    pub su: Option<u64>,
    pub end: Option<u64>,
    pub node_count: Option<u32>,
    pub node_certification: bool,
}

pub enum FarmCertification {
    Gold,
    NotCertified,
}

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

pub struct Interface {
    pub name: String,
    // This really should be a [u8;6], but the chain saves this as a string currently;
    pub mac: String,
    pub ips: Vec<String>,
}

pub struct Resources {
    pub hru: u64,
    pub sru: u64,
    pub cru: u64,
    pub mru: u64,
}

pub struct Location {
    pub longitude: String,
    pub latitude: String,
}

pub struct Contract {
    pub version: u32,
    pub state: ContractState,
    pub contract_id: u64,
    pub twin_id: u32,
    pub contract_type: ContractData,
    pub solution_provider_id: Option<u64>,
}

pub enum ContractState {
    Created,
    Deleted(Cause),
    GracePeriod(u64),
}

pub enum Cause {
    CanceledByUser,
    OutOfFunds,
}

pub enum ContractData {
    NodeContract(NodeContract),
    NameContract(NameContract),
    RentContract(RentContract),
}

pub struct NodeContract {
    pub node_id: u32,
    pub deployment_hash: Hash,
    pub deployment_data: Vec<u8>,
    pub public_ips: u32,
    pub public_ips_list: Vec<PublicIP>,
}

pub struct NameContract {
    pub name: String,
}

pub struct RentContract {
    pub node_id: u32,
}

pub struct PublicConfig {
    pub ip4: PubIPConfig,
    pub ip6: Option<PubIPConfig>,
    pub domain: Option<Domain>,
}

pub struct PubIPConfig {
    pub ip: String,
    pub gw: String,
}

pub struct Domain(pub String);

pub struct ContractResources {
    pub contract_id: u64,
    pub used: Resources,
}

pub struct EntityProof {
    pub entity_id: u32,
    pub signature: Vec<u8>,
}

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

pub enum NodeCertification {
    Certified,
    Diy,
}
