use crate::events::BurningEvent;

impl From<pallet_burning_legacy::Event<runtime_legacy::Runtime>> for BurningEvent {
    fn from(pbe: pallet_burning_legacy::Event<runtime_legacy::Runtime>) -> Self {
        match pbe {
            pallet_burning_legacy::Event::<runtime_legacy::Runtime>::BurnTransactionCreated(
                acc,
                amount,
                block,
                data,
            ) => BurningEvent::TokensBurned(acc, (amount as u64).into(), block, data),
        }
    }
}
