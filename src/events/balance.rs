use crate::types::{Balance, BalanceStatus};
use sp_core::crypto::AccountId32;

#[derive(Debug)]
pub enum Event {
    /// An account was created with some free balance. \[account, free_balance\]
    Endowed(AccountId32, Balance),
    /// An account was removed whose balance was non-zero but below ExistentialDeposit,
    /// resulting in an outright loss. \[account, balance\]
    DustLost(AccountId32, Balance),
    /// Transfer succeeded. \[from, to, value\]
    Transfer(AccountId32, AccountId32, Balance),
    /// A balance was set by root. \[who, free, reserved\]
    BalanceSet(AccountId32, Balance, Balance),
    /// Some amount was deposited (e.g. for transaction fees). \[who, deposit\]
    Deposit(AccountId32, Balance),
    /// Some balance was reserved (moved from free to reserved). \[who, value\]
    Reserved(AccountId32, Balance),
    /// Some balance was unreserved (moved from reserved to free). \[who, value\]
    Unreserved(AccountId32, Balance),
    /// Some balance was moved from the reserve of the first account to the second account.
    /// Final argument indicates the destination balance type.
    /// \[from, to, balance, destination_status\]
    ReserveRepatriated(AccountId32, AccountId32, Balance, BalanceStatus),
    /// Unknown event
    Unknown,
}

impl From<pallet_balance::Event<runtime::Runtime>> for Event {
    fn from(be: pallet_balance::Event<runtime::Runtime>) -> Self {
        match be {
            pallet_balance::Event::Endowed(acc, balance) => {
                Event::Endowed(acc, (balance as u64).into())
            }
            pallet_balance::Event::DustLost(acc, balance) => {
                Event::DustLost(acc, (balance as u64).into())
            }
            pallet_balance::Event::Transfer(from, to, balance) => {
                Event::Transfer(from, to, (balance as u64).into())
            }
            pallet_balance::Event::BalanceSet(acc, from, to) => {
                Event::BalanceSet(acc, (from as u64).into(), (to as u64).into())
            }
            pallet_balance::Event::Deposit(acc, amount) => {
                Event::Deposit(acc, (amount as u64).into())
            }
            pallet_balance::Event::Reserved(acc, amount) => {
                Event::Reserved(acc, (amount as u64).into())
            }
            pallet_balance::Event::Unreserved(acc, amount) => {
                Event::Unreserved(acc, (amount as u64).into())
            }
            pallet_balance::Event::ReserveRepatriated(from, to, amount, dest_status) => {
                Event::ReserveRepatriated(from, to, (amount as u64).into(), dest_status)
            }
            _ => Event::Unknown,
        }
    }
}
