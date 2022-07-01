use crate::types::{Balance, Consumption, Contract, ContractBill, PublicIP, Resources};
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
    /// Update the resources used by a contract \[Contract id, resources\]
    UpdatedUsedResources(u64, Resources),
    /// Nru consumption reported by a node for contract \[Contract id, timestamp, window duration,
    /// NRU\]
    NruConsumption(u64, u64, u64, u64),
    /// A rent contract was cancelled \[contract id\]
    RentContractCancelled(u64),
    /// A contract was moved to grace period \[contract id, node id, twin id, block number\]
    ContractGracePeriodStarted(u64, u32, u32, u64),
    /// A contract grace period ended \[contract id, node id, twin id\]
    ContractGracePeriodEnded(u64, u32, u32),
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
            pallet_smart_contract::Event::<runtime::Runtime>::UpdatedUsedResources(resources) => {
                Event::UpdatedUsedResources(resources.contract_id, resources.used.into())
            }
            pallet_smart_contract::Event::<runtime::Runtime>::NruConsumptionReportReceived(n) => {
                Event::NruConsumption(n.contract_id, n.timestamp, n.window, n.nru)
            }
            pallet_smart_contract::Event::<runtime::Runtime>::RentContractCanceled(contract_id) => {
                Event::RentContractCancelled(contract_id)
            }
            pallet_smart_contract::Event::<runtime::Runtime>::ContractGracePeriodStarted(
                contract_id,
                node_id,
                twin_id,
                block,
            ) => Event::ContractGracePeriodStarted(contract_id, node_id, twin_id, block),
            pallet_smart_contract::Event::<runtime::Runtime>::ContractGracePeriodEnded(
                contract_id,
                node_id,
                twin_id,
            ) => Event::ContractGracePeriodEnded(contract_id, node_id, twin_id),
        }
    }
}
