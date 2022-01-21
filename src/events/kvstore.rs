use sp_core::crypto::AccountId32;

#[derive(Debug)]
pub enum Event {
    /// A user has set their entry
    EntrySet(AccountId32, Vec<u8>, Vec<u8>),
    /// A user has read their entry, leaving it in storage
    EntryGot(AccountId32, Vec<u8>, Vec<u8>),
    /// A user has read their entry, removing it from storage
    EntryTaken(AccountId32, Vec<u8>, Vec<u8>),
}

impl From<pallet_kvstore::Event<runtime::Runtime>> for Event {
    fn from(kve: pallet_kvstore::Event<runtime::Runtime>) -> Self {
        match kve {
            pallet_kvstore::Event::<runtime::Runtime>::EntrySet(acc, key, value) => {
                Event::EntrySet(acc, key, value)
            }
            pallet_kvstore::Event::<runtime::Runtime>::EntryGot(acc, key, value) => {
                Event::EntryGot(acc, key, value)
            }
            pallet_kvstore::Event::<runtime::Runtime>::EntryTaken(acc, key, value) => {
                Event::EntryTaken(acc, key, value)
            }
        }
    }
}
