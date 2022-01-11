use chrono::prelude::*;
use codec::{Decode, Encode};
use std::fmt::{self, Display};
pub use sp_core::crypto::AccountId32;
pub use substrate_api_client::AccountInfo;
pub use tfchain_tfgrid::types::{CertificationType, Resources};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Twin<AccountId> {
    pub version: u32,
    pub id: u32,
    pub account_id: AccountId,
    pub ip: String,
    pub entities: Vec<EntityProof>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct EntityProof {
    pub entity_id: u32,
    pub signature: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Farm {
    pub version: u32,
    pub id: u32,
    pub name: String,
    pub twin_id: u32,
    pub pricing_policy_id: u32,
    pub certification_type: CertificationType,
    pub public_ips: Vec<PublicIP>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct PublicIP {
    pub ip: String,
    pub gateway: String,
    pub contract_id: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Node {
    pub version: u32,
    pub id: u32,
    pub farm_id: u32,
    pub twin_id: u32,
    pub resources: Resources,
    pub location: Location,
    pub country: String,
    pub city: String,
    // optional public config
    pub public_config: Option<PublicConfig>,
    pub created: u64,
    pub farming_policy_id: u32,
    pub interfaces: Vec<Interface>,
    pub certification_type: CertificationType,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Location {
    pub longitude: String,
    pub latitude: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Interface {
    pub name: String,
    pub mac: String,
    pub ips: Vec<IP>,
}

pub type IP = String;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct PublicConfig {
    pub ipv4: String,
    pub ipv6: String,
    pub gw4: String,
    pub gw6: String,
    pub domain: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Contract {
    pub version: u32,
    pub state: ContractState,
    pub contract_id: u64,
    pub twin_id: u32,
    pub contract_type: ContractData,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct NodeContract {
    pub node_id: u32,
    // deployment_data is the encrypted deployment body. This encrypted the deployment with the **USER** public key.
    // So only the user can read this data later on (or any other key that he keeps safe).
    // this data part is read only by the user and can actually hold any information to help him reconstruct his deployment or can be left empty.
    pub deployment_data: String,
    // Hash of the deployment, set by the user
    pub deployment_hash: String,
    pub public_ips: u32,
    pub public_ips_list: Vec<PublicIP>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct NameContract {
    pub name: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum ContractData {
    NodeContract(NodeContract),
    NameContract(NameContract),
}

impl Default for ContractData {
    fn default() -> ContractData {
        ContractData::NodeContract(NodeContract::default())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum ContractState {
    Created,
    Deleted(Cause),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum Cause {
    CanceledByUser,
    OutOfFunds
}

impl Default for ContractState {
    fn default() -> ContractState {
        ContractState::Created
    }
}

impl Display for Twin<AccountId32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Twin details for twin {}", self.id)?;
        writeln!(f, "Account ID {}", self.account_id)?;
        writeln!(f, "IP: {}", self.ip)
    }
}

impl Display for ContractData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractData::NameContract(name_contract) => {
                writeln!(f, "Name: {}", name_contract.name)
            },
            ContractData::NodeContract(node_contract) => {
                writeln!(f, "Node id: {}", node_contract.node_id)?;
                writeln!(f, "Deployment data: {}", node_contract.deployment_data)?;
                writeln!(f, "Deployment hash: {}", node_contract.deployment_hash)?;
                for ip in &node_contract.public_ips_list {
                    writeln!(f, "IP: {}", ip.ip)?;
                    writeln!(f, "Gateway: {}", ip.gateway)?;
                }
                write!(f, "Number of public ips: {}", node_contract.public_ips)
            }
        }
    }
}

impl Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Contract details for contract {}", self.contract_id)?;
        writeln!(f, "State: {}", self.state)?;
        writeln!(f, "Twin id: {}", self.twin_id)?;
        writeln!(f, "{}", self.contract_type)
    }
}

impl Display for ContractState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractState::Created => {
                write!(f, "Created")
            },
            ContractState::Deleted(Cause::CanceledByUser) => {
                write!(f, "Canceled by user")
            },
            ContractState::Deleted(Cause::OutOfFunds) => {
                write!(f, "Out of funds")
            },
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Node details for node {}", self.id)?;
        writeln!(f, "Farm ID: {}", self.farm_id)?;
        writeln!(f, "Twin ID: {}", self.twin_id)?;
        writeln!(f, "Node resources:")?;
        writeln!(f, "\tCRU: {} (logical cores)", self.resources.cru)?;
        writeln!(
            f,
            "\tMRU: {} ({:.02} GB | {:.02} GiB)",
            self.resources.mru,
            self.resources.mru as f64 / 1_000_000_000f64,
            self.resources.mru as f64 / (1 << 30) as f64
        )?;
        writeln!(
            f,
            "\tSRU: {} ({:.02} TB | {:.02} TiB)",
            self.resources.sru,
            self.resources.sru as f64 / 1_000_000_000_000f64,
            self.resources.sru as f64 / (1u64 << 40) as f64
        )?;
        writeln!(
            f,
            "\tHRU: {} ({:.02} TB | {:.02} TiB)",
            self.resources.hru,
            self.resources.hru as f64 / 1_000_000_000_000f64,
            self.resources.hru as f64 / (1u64 << 40) as f64
        )?;
        writeln!(
            f,
            "Location: {}, {} ({} lat {} long)",
            self.city, self.country, self.location.latitude, self.location.longitude
        )?;
        writeln!(
            f,
            "Created at: {}",
            Utc.timestamp(self.created as i64, 0)
                .with_timezone(&Local)
                .to_rfc2822()
        )?;
        writeln!(f, "Farming policy: {}", self.farming_policy_id)?;
        if self.interfaces.is_empty() {
            writeln!(f, "No known interfaces")?;
        } else {
            // Add some nice formatting, make sure all MAC address and IP addresses start in the
            // same column.
            // unwrap here is safe as None is only returned in case of an empty iterator, and we
            // specifically check for that case.
            let iface_name_space = self
                .interfaces
                .iter()
                .map(|iface| iface.name.len())
                .max()
                .unwrap();
            writeln!(f, "Interfaces:")?;
            for iface in &self.interfaces {
                writeln!(
                    f,
                    "\t{}: {:>width$}",
                    iface.name,
                    iface.mac,
                    width = iface_name_space - iface.name.len() + iface.mac.len()
                )?;
                for ip in &iface.ips {
                    writeln!(
                        f,
                        "\t{:>width$}",
                        ip,
                        width = ip.len() + iface_name_space + 2
                    )?;
                }
            }
        }
        if let Some(ref pub_config) = self.public_config {
            writeln!(f, "Public config:")?;
            writeln!(f, "\tIPv4: {} (gw: {})", pub_config.ipv4, pub_config.gw4)?;
            writeln!(f, "\tIPv6: {} (gw: {})", pub_config.ipv4, pub_config.gw6)?;
            writeln!(f, "\tDomain: {}", pub_config.domain)?;
        }
        writeln!(
            f,
            "Certification type {}",
            match self.certification_type {
                CertificationType::Diy => "DIY",
                CertificationType::Certified => "Certified",
            }
        )
    }
}

impl Display for Farm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Farm details for farm {} (ID: {})", self.name, self.id)?;
        writeln!(f, "Twin ID: {}", self.twin_id)?;
        writeln!(
            f,
            "Certification type {}",
            match self.certification_type {
                CertificationType::Diy => "DIY",
                CertificationType::Certified => "Certified",
            }
        )?;
        if !self.public_ips.is_empty() {
            writeln!(f, "Public IPs:")?;
            for ip in &self.public_ips {
                writeln!(f, "\tIPv4: {} (gw: {})", ip.ip, ip.gateway)?;
                writeln!(f, "\tContract id: {}", ip.contract_id)?;
            }
        }
        writeln!(f, "version: {}", self.version)
    }
}
