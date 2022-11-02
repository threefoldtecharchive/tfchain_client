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

pub struct Farm {}

pub struct Node {}

pub struct Contract {}

pub struct ContractResources {}

pub struct FarmPolicy {}

pub struct EntityProof {
    pub entity_id: u32,
    pub signature: Vec<u8>,
}
