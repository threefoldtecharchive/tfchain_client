use crate::events::ValidatorSetEvent;

impl From<substrate_validator_set_legacy::Event<runtime_legacy::Runtime>> for ValidatorSetEvent {
    fn from(svse: substrate_validator_set_legacy::Event<runtime_legacy::Runtime>) -> Self {
        match svse {
            substrate_validator_set_legacy::Event::ValidatorAdded(acc) => {
                ValidatorSetEvent::ValidatorAdded(acc)
            }
            substrate_validator_set_legacy::Event::ValidatorRemoved(acc) => {
                ValidatorSetEvent::ValidatorRemoved(acc)
            }
            _ => ValidatorSetEvent::Unknown,
        }
    }
}
