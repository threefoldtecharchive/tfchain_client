use sp_core::crypto::AccountId32;

#[derive(Debug)]
pub enum Event {
    /// An extrinsic completed successfully. \[info\]
    ExtrinsicSuccess,
    //TODO: ExtrinsicSuccess(DispatchInfo),
    /// An extrinsic failed. \[error, info\]
    ExtrinsicFailed,
    //TODO: ExtrinsicFailed(DispatchError, DispatchInfo),
    /// `:code` was updated.
    CodeUpdated,
    /// A new \[account\] was created.
    NewAccount(AccountId32),
    /// An \[account\] was reaped.
    KilledAccount(AccountId32),
    /// Unknown event
    Unknown,
}

impl From<system::Event<runtime::Runtime>> for Event {
    fn from(fse: system::Event<runtime::Runtime>) -> Self {
        match fse {
            system::Event::ExtrinsicSuccess(_) => Event::ExtrinsicSuccess,
            system::Event::ExtrinsicFailed(_, _) => {
                Event::ExtrinsicFailed
                // Event::ExtrinsicSuccess(DispatchError, DispatchInfo)
            }
            system::Event::CodeUpdated => Event::CodeUpdated,
            system::Event::NewAccount(acc) => Event::NewAccount(acc),
            system::Event::KilledAccount(acc) => Event::KilledAccount(acc),
            _ => Event::Unknown,
        }
    }
}
