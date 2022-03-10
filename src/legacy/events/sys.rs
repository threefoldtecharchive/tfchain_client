use crate::events::SystemEvent;

impl From<system::Event<runtime_legacy::Runtime>> for SystemEvent {
    fn from(fse: system::Event<runtime_legacy::Runtime>) -> Self {
        match fse {
            system::Event::ExtrinsicSuccess(_) => SystemEvent::ExtrinsicSuccess,
            system::Event::ExtrinsicFailed(_, _) => {
                SystemEvent::ExtrinsicFailed
                // Event::ExtrinsicSuccess(DispatchError, DispatchInfo)
            }
            system::Event::CodeUpdated => SystemEvent::CodeUpdated,
            system::Event::NewAccount(acc) => SystemEvent::NewAccount(acc),
            system::Event::KilledAccount(acc) => SystemEvent::KilledAccount(acc),
            _ => SystemEvent::Unknown,
        }
    }
}
