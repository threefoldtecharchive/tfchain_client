use crate::events::TfchainEvent;

mod balance;
mod burning;
mod grandpa;
mod kvstore;
mod runtime_upgrade;
mod scheduler;
mod session;
mod smart_contract;
mod sudo;
mod sys;
mod tfgrid;
mod tft_bridge;
mod tft_price;
mod validator_set;

pub use crate::events::BalanceEvent;
pub use crate::events::BurningEvent;
pub use crate::events::CollectiveEvent;
pub use crate::events::GrandpaEvent;
pub use crate::events::KVEvent;
pub use crate::events::RuntimeUpgradeEvent;
pub use crate::events::SchedulerEvent;
pub use crate::events::SessionEvent;
pub use crate::events::SmartContractEvent;
pub use crate::events::SudoEvent;
pub use crate::events::SystemEvent;
pub use crate::events::TFGridEvent;
pub use crate::events::TftBridgeEvent;
pub use crate::events::TftPriceEvent;
pub use crate::events::ValidatorSetEvent;

impl From<runtime_legacy::Event> for TfchainEvent {
    fn from(e: runtime_legacy::Event) -> Self {
        match e {
            runtime_legacy::Event::frame_system(se) => TfchainEvent::System(se.into()),
            runtime_legacy::Event::validatorset(vse) => TfchainEvent::ValidatorSet(vse.into()),
            runtime_legacy::Event::pallet_tfgrid(tfge) => {
                TfchainEvent::TFGrid(Box::new(tfge.into()))
            }
            runtime_legacy::Event::pallet_burning(be) => TfchainEvent::Burning(be.into()),
            runtime_legacy::Event::pallet_kvstore(kve) => TfchainEvent::KVStore(kve.into()),
            runtime_legacy::Event::pallet_smart_contract(sce) => {
                TfchainEvent::SmartContract(sce.into())
            }
            runtime_legacy::Event::pallet_runtime_upgrade(rtue) => {
                TfchainEvent::RuntimeUpgrade(rtue.into())
            }
            runtime_legacy::Event::pallet_balances(be) => TfchainEvent::Balance(be.into()),
            runtime_legacy::Event::pallet_grandpa(ge) => TfchainEvent::Grandpa(ge.into()),
            runtime_legacy::Event::pallet_sudo(se) => TfchainEvent::Sudo(se.into()),
            runtime_legacy::Event::pallet_tft_price(tpe) => TfchainEvent::TftPriceEvent(tpe.into()),
            runtime_legacy::Event::pallet_tft_bridge(be) => TfchainEvent::TftBridgeEvent(be.into()),
            runtime_legacy::Event::pallet_scheduler(se) => TfchainEvent::Scheduler(se.into()),
            runtime_legacy::Event::pallet_collective_Instance1(ce) => {
                TfchainEvent::Collective(ce.into())
            }
            runtime_legacy::Event::pallet_session(se) => TfchainEvent::Session(se.into()),
            // TODO, ignored for now
            runtime_legacy::Event::pallet_membership_Instance1(_)
            | runtime_legacy::Event::pallet_validator(_) => TfchainEvent::Unknown,
        }
    }
}
