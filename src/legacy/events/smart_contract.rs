use crate::{events::SmartContractEvent, types::PublicIP};

impl From<pallet_smart_contract_legacy::Event<runtime_legacy::Runtime>> for SmartContractEvent {
    fn from(sce: pallet_smart_contract_legacy::Event<runtime_legacy::Runtime>) -> Self {
        match sce {
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::ContractCreated(c) => {
                SmartContractEvent::ContractCreated(c.into())
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::ContractUpdated(c) => {
                SmartContractEvent::ContractUpdated(c.into())
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::NodeContractCanceled(
                contract_id,
                node_id,
                twin_id,
            ) => SmartContractEvent::NodeContractCanceled(contract_id, node_id, twin_id),
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::NameContractCanceled(contract_id) => {
                SmartContractEvent::NameContractCanceled(contract_id)
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::IPsReserved(contract_id, ips) => {
                SmartContractEvent::IPsReserved(contract_id, ips.into_iter().map(PublicIP::from).collect())
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::IPsFreed(contract_id, ips) => {
                SmartContractEvent::IPsFreed(contract_id, ips)
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::ContractDeployed(
                contract_id,
                twin,
            ) => SmartContractEvent::ContractDeployed(contract_id, twin),
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::ConsumptionReportReceived(cs) => {
                SmartContractEvent::ConsumptionReportReceived(cs.into())
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::ContractBilled(ctb) => {
                SmartContractEvent::ContractBilled(ctb.into())
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::TokensBurned(contract_id, amount) => {
                SmartContractEvent::TokensBurned(contract_id, (amount as u64).into())
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::UpdatedUsedResources(resources) => {
                SmartContractEvent::UpdatedUsedResources(resources.contract_id, resources.used.into())
            }
            pallet_smart_contract_legacy::Event::<runtime_legacy::Runtime>::NruConsumptionReportReceived(n) => {
                SmartContractEvent::NruConsumption(n.contract_id, n.timestamp, n.window, n.nru)
            }
        }
    }
}
