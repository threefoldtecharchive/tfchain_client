use crate::types::BlockNumber;

#[derive(Debug)]
pub enum Event {
    /// Scheduled some task. \[when, index\]
    Scheduled(BlockNumber, u32),
    /// Canceled some task. \[when, index\]
    Canceled(BlockNumber, u32),
    /// Dispatched some task. \[task, id, result\]
    //TODO: Dispatched(TaskAddress<BlockNumber>, Option<Vec<u8>>, DispatchResult),
    Dispatched,
}

impl From<pallet_scheduler::Event<runtime_legacy::Runtime>> for Event {
    fn from(se: pallet_scheduler::Event<runtime_legacy::Runtime>) -> Self {
        match se {
            pallet_scheduler::Event::<runtime_legacy::Runtime>::Scheduled(block, idx) => {
                Event::Scheduled(block, idx)
            }
            pallet_scheduler::Event::<runtime_legacy::Runtime>::Canceled(block, idx) => {
                Event::Canceled(block, idx)
            }
            pallet_scheduler::Event::<runtime_legacy::Runtime>::Dispatched(_, _, _) => Event::Dispatched,
        }
    }
}
