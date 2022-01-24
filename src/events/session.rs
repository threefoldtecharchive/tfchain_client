pub type SessionIndex = u32;

#[derive(Debug)]
pub enum Event {
    /// New session has happened. Note that the argument is the \[session_index\], not the block
    /// number as the type might suggest.
    NewSession(SessionIndex),
}

impl From<pallet_session::Event> for Event {
    fn from(se: pallet_session::Event) -> Self {
        match se {
            pallet_session::Event::NewSession(si) => Event::NewSession(si),
        }
    }
}
