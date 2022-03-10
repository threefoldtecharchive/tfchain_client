use crate::events::RuntimeUpgradeEvent;

impl From<pallet_runtime_upgrade_legacy::Event> for RuntimeUpgradeEvent {
    fn from(_: pallet_runtime_upgrade_legacy::Event) -> Self {
        RuntimeUpgradeEvent::Empty
    }
}
