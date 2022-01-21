#[derive(Debug)]
pub enum Event {
    /// An empty variant which should never be constructible.
    Empty,
}

impl From<pallet_runtime_upgrade::Event> for Event {
    fn from(_: pallet_runtime_upgrade::Event) -> Self {
        Event::Empty
    }
}
