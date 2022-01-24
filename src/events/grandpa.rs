use crate::types::{AuthorityId, AuthorityList};

#[derive(Debug)]
pub enum Event {
    /// New authority set has been applied. \[authority_set\]
    NewAuthorities(AuthorityList),
    /// Current authority set has been paused.
    Paused,
    /// Current authority set has been resumed.
    Resumed,
}

impl From<pallet_grandpa::Event> for Event {
    fn from(ge: pallet_grandpa::Event) -> Self {
        match ge {
            pallet_grandpa::Event::NewAuthorities(al) => Event::NewAuthorities(
                al.into_iter()
                    .map(|(id, w)| (AuthorityId::from(id), w))
                    .collect(),
            ),
            pallet_grandpa::Event::Paused => Event::Paused,
            pallet_grandpa::Event::Resumed => Event::Resumed,
        }
    }
}
