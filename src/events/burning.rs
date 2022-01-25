use crate::types::{Balance, BlockNumber};
use sp_core::crypto::AccountId32;

#[derive(Debug)]
pub enum Event {
    TokensBurned(AccountId32, Balance, BlockNumber, Vec<u8>),
}

impl From<pallet_burning::Event<runtime::Runtime>> for Event {
    fn from(pbe: pallet_burning::Event<runtime::Runtime>) -> Self {
        match pbe {
            pallet_burning::Event::<runtime::Runtime>::BurnTransactionCreated(
                acc,
                amount,
                block,
                data,
            ) => Event::TokensBurned(acc, (amount as u64).into(), block, data),
        }
    }
}
