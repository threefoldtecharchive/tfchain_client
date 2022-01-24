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
mod grandpa;
mod kvstore;
mod runtime_upgrade;
mod smart_contract;
mod sys;
mod tfgrid;
mod validator_set;
// mod vesting_validator;

pub use balance::Event as BalanceEvent;
pub use burning::Event as BurningEvent;
pub use grandpa::Event as GrandpaEvent;
pub use kvstore::Event as KVEvent;
pub use runtime_upgrade::Event as RuntimeUpgradeEvent;
pub use smart_contract::Event as SmartContractEvent;
pub use sys::Event as SystemEvent;
pub use tfgrid::Event as TFGridEvent;
pub use validator_set::Event as ValidatorSetEvent;
// pub use vesting_validator::Event as VestingValidatorEvent;

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
            runtime::Event::pallet_sudo(_)
            | runtime::Event::pallet_tft_price(_)
            | runtime::Event::pallet_tft_bridge(_)
            | runtime::Event::pallet_scheduler(_)
            | runtime::Event::pallet_collective_Instance1(_)
            | runtime::Event::pallet_session(_) => {
                todo!()
            }
        }
    }
}
