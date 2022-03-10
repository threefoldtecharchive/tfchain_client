use crate::events::KVEvent;

impl From<pallet_kvstore_legacy::Event<runtime_legacy::Runtime>> for KVEvent {
    fn from(kve: pallet_kvstore_legacy::Event<runtime_legacy::Runtime>) -> Self {
        match kve {
            pallet_kvstore_legacy::Event::<runtime_legacy::Runtime>::EntrySet(acc, key, value) => {
                KVEvent::EntrySet(acc, key, value)
            }
            pallet_kvstore_legacy::Event::<runtime_legacy::Runtime>::EntryGot(acc, key, value) => {
                KVEvent::EntryGot(acc, key, value)
            }
            pallet_kvstore_legacy::Event::<runtime_legacy::Runtime>::EntryTaken(
                acc,
                key,
                value,
            ) => KVEvent::EntryTaken(acc, key, value),
        }
    }
}
