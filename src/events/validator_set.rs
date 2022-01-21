use sp_core::crypto::AccountId32;

#[derive(Debug)]
pub enum Event {
    /// New validator added.
    ValidatorAdded(AccountId32),
    /// Validator removed.
    ValidatorRemoved(AccountId32),
    /// Unknown validator event
    Unknown,
}

impl From<substrate_validator_set::Event<runtime::Runtime>> for Event {
    fn from(svse: substrate_validator_set::Event<runtime::Runtime>) -> Self {
        match svse {
            substrate_validator_set::Event::ValidatorAdded(acc) => Event::ValidatorAdded(acc),
            substrate_validator_set::Event::ValidatorRemoved(acc) => Event::ValidatorRemoved(acc),
            _ => Event::Unknown,
        }
    }
}
