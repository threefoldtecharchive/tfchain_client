use crate::types::{Balance, Consumption, Contract, ContractBill, PublicIP};
use sp_core::crypto::AccountId32;

#[derive(Debug)]
pub enum Event {
    /// A new contract is created
    ContractCreated(Contract),
    /// A contract has been updated
    ContractUpdated(Contract),
    /// A contract for a node has been cancelled [contract id, node id, twin id]
    NodeContractCanceled(u64, u32, u32),
    /// A name contract has been cancelled \[contract id\]
    NameContractCanceled(u64),
    /// Public ips have been reserved \[contract id, \[public ips\]\]
    IPsReserved(u64, Vec<PublicIP>),
    /// Public ips have been released \[contract id, \[public ips\]\]
    IPsFreed(u64, Vec<Vec<u8>>),
    /// A contract has been deployed (used?)
    ContractDeployed(u64, AccountId32),
    /// Consumption report has been received
    ConsumptionReportReceived(Consumption),
    /// Contract has been billed
    ContractBilled(ContractBill),
    /// Tokens burned for contract payment \[Contract id, amount\]
    TokensBurned(u64, Balance),
}

impl From<pallet_smart_contract::Event<runtime::Runtime>> for Event {
    fn from(sce: pallet_smart_contract::Event<runtime::Runtime>) -> Self {
        match sce {
            pallet_smart_contract::Event::<runtime::Runtime>::ContractCreated(c) => {
                Event::ContractCreated(c.into())
            }
            pallet_smart_contract::Event::<runtime::Runtime>::ContractUpdated(c) => {
                Event::ContractUpdated(c.into())
            }
            pallet_smart_contract::Event::<runtime::Runtime>::NodeContractCanceled(
                contract_id,
                node_id,
                twin_id,
            ) => Event::NodeContractCanceled(contract_id, node_id, twin_id),
            pallet_smart_contract::Event::<runtime::Runtime>::NameContractCanceled(contract_id) => {
                Event::NameContractCanceled(contract_id)
            }
            pallet_smart_contract::Event::<runtime::Runtime>::IPsReserved(contract_id, ips) => {
                Event::IPsReserved(contract_id, ips.into_iter().map(PublicIP::from).collect())
            }
            pallet_smart_contract::Event::<runtime::Runtime>::IPsFreed(contract_id, ips) => {
                Event::IPsFreed(contract_id, ips)
            }
            pallet_smart_contract::Event::<runtime::Runtime>::ContractDeployed(
                contract_id,
                twin,
            ) => Event::ContractDeployed(contract_id, twin),
            pallet_smart_contract::Event::<runtime::Runtime>::ConsumptionReportReceived(cs) => {
                Event::ConsumptionReportReceived(cs.into())
            }
            pallet_smart_contract::Event::<runtime::Runtime>::ContractBilled(ctb) => {
                Event::ContractBilled(ctb.into())
            }
            pallet_smart_contract::Event::<runtime::Runtime>::TokensBurned(contract_id, amount) => {
                Event::TokensBurned(contract_id, (amount as u64).into())
            }
        }
    }
}
