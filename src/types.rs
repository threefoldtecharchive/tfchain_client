use std::net::IpAddr;

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

pub struct Node {}

pub struct Contract {}

pub struct ContractResources {}

pub struct FarmPolicy {}

pub struct EntityProof {
    pub entity_id: u32,
    pub signature: Vec<u8>,
}
