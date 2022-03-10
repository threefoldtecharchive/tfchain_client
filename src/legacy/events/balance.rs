use crate::events::BalanceEvent;

impl From<pallet_balance::Event<runtime_legacy::Runtime>> for BalanceEvent {
    fn from(be: pallet_balance::Event<runtime_legacy::Runtime>) -> Self {
        match be {
            pallet_balance::Event::Endowed(acc, balance) => {
                BalanceEvent::Endowed(acc, (balance as u64).into())
            }
            pallet_balance::Event::DustLost(acc, balance) => {
                BalanceEvent::DustLost(acc, (balance as u64).into())
            }
            pallet_balance::Event::Transfer(from, to, balance) => {
                BalanceEvent::Transfer(from, to, (balance as u64).into())
            }
            pallet_balance::Event::BalanceSet(acc, from, to) => {
                BalanceEvent::BalanceSet(acc, (from as u64).into(), (to as u64).into())
            }
            pallet_balance::Event::Deposit(acc, amount) => {
                BalanceEvent::Deposit(acc, (amount as u64).into())
            }
            pallet_balance::Event::Reserved(acc, amount) => {
                BalanceEvent::Reserved(acc, (amount as u64).into())
            }
            pallet_balance::Event::Unreserved(acc, amount) => {
                BalanceEvent::Unreserved(acc, (amount as u64).into())
            }
            pallet_balance::Event::ReserveRepatriated(from, to, amount, dest_status) => {
                BalanceEvent::ReserveRepatriated(from, to, (amount as u64).into(), dest_status)
            }
            _ => BalanceEvent::Unknown,
        }
    }
}
