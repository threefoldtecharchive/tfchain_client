use crate::types::AccountId32;

#[derive(Debug)]
pub enum Event {
    /// A sudo just took place. \[result\]
    //TODO: Sudid(DispatchResult),
    Sudid,
    /// The \[sudoer\] just switched identity; the old key is supplied.
    KeyChanged(AccountId32),
    /// A sudo just took place. \[result\]
    //TODO: SudoAsDone(DispatchResult),
    SudoAsDone,
}

impl From<pallet_sudo::Event<runtime::Runtime>> for Event {
    fn from(se: pallet_sudo::Event<runtime::Runtime>) -> Self {
        match se {
            pallet_sudo::Event::<runtime::Runtime>::Sudid(_) => Event::Sudid,
            pallet_sudo::Event::<runtime::Runtime>::KeyChanged(acc) => Event::KeyChanged(acc),
            pallet_sudo::Event::<runtime::Runtime>::SudoAsDone(_) => Event::SudoAsDone,
        }
    }
}
