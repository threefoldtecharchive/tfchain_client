//! Handling and parsing of events from tfchain.
//!
//! The event module contains all events that can possibly emitted in a block. Technically this is
//! redundant, as all events are already part of the [Runtime](runtime::Runtime). Working with
//! those raw types directly has some disadvantages though. Mostly, they are generic over the
//! [Runtime](runtime::Runtime), to support dependencies on other pallets, and allow easy configuration.
//! This comes with the downside that we need to specify this generic parameter everywhere. As such,
//! retyping them here allows us to effectively remove the parameter, as it is known here anyway.

mod balance;
mod burning;
mod collective;
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

pub use balance::Event as BalanceEvent;
pub use burning::Event as BurningEvent;
pub use collective::Event as CollectiveEvent;
pub use grandpa::Event as GrandpaEvent;
pub use kvstore::Event as KVEvent;
pub use runtime_upgrade::Event as RuntimeUpgradeEvent;
pub use scheduler::Event as SchedulerEvent;
pub use session::Event as SessionEvent;
pub use smart_contract::Event as SmartContractEvent;
pub use sudo::Event as SudoEvent;
pub use sys::Event as SystemEvent;
pub use tfgrid::Event as TFGridEvent;
pub use tft_bridge::Event as TftBridgeEvent;
pub use tft_price::Event as TftPriceEvent;
pub use validator_set::Event as ValidatorSetEvent;

#[derive(Debug)]
pub enum TfchainEvent {
    System(SystemEvent),
    Burning(BurningEvent),
    KVStore(KVEvent),
    RuntimeUpgrade(RuntimeUpgradeEvent),
    SmartContract(SmartContractEvent),
    TFGrid(TFGridEvent),
    // VestingValidator(VestingValidatorEvent),
    ValidatorSet(ValidatorSetEvent),
    Balance(BalanceEvent),
    Grandpa(GrandpaEvent),
    Sudo(SudoEvent),
    TftPriceEvent(TftPriceEvent),
    TftBridgeEvent(TftBridgeEvent),
    Scheduler(SchedulerEvent),
    Collective(CollectiveEvent),
    Session(SessionEvent),
}

impl From<runtime::Event> for TfchainEvent {
    fn from(e: runtime::Event) -> Self {
        match e {
            runtime::Event::frame_system(se) => TfchainEvent::System(se.into()),
            runtime::Event::validatorset(vse) => TfchainEvent::ValidatorSet(vse.into()),
            runtime::Event::pallet_tfgrid(tfge) => TfchainEvent::TFGrid(tfge.into()),
            runtime::Event::pallet_burning(be) => TfchainEvent::Burning(be.into()),
            runtime::Event::pallet_kvstore(kve) => TfchainEvent::KVStore(kve.into()),
            runtime::Event::pallet_smart_contract(sce) => TfchainEvent::SmartContract(sce.into()),
            runtime::Event::pallet_runtime_upgrade(rtue) => {
                TfchainEvent::RuntimeUpgrade(rtue.into())
            }
            runtime::Event::pallet_balances(be) => TfchainEvent::Balance(be.into()),
            runtime::Event::pallet_grandpa(ge) => TfchainEvent::Grandpa(ge.into()),
            runtime::Event::pallet_sudo(se) => TfchainEvent::Sudo(se.into()),
            runtime::Event::pallet_tft_price(tpe) => TfchainEvent::TftPriceEvent(tpe.into()),
            runtime::Event::pallet_tft_bridge(be) => TfchainEvent::TftBridgeEvent(be.into()),
            runtime::Event::pallet_scheduler(se) => TfchainEvent::Scheduler(se.into()),
            runtime::Event::pallet_collective_Instance1(ce) => TfchainEvent::Collective(ce.into()),
            runtime::Event::pallet_session(se) => TfchainEvent::Session(se.into()),
        }
    }
}
