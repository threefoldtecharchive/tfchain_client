use crate::types::{AccountId32, Hash};

#[derive(Debug)]
pub enum Event {
    Voted(AccountId32, Hash, bool, u32, u32),
    Proposed(AccountId32, u32, Hash, u32),
    Approved(Hash),
    Disapproved(Hash),
    Executed(Hash),
    Closed(Hash, u32, u64, u32, u64),
    ClosedByCouncil(Hash, Vec<AccountId32>),
    CouncilMemberVeto(Hash, AccountId32),
}

impl From<pallet_dao::Event<runtime::Runtime>> for Event {
    fn from(pde: pallet_dao::Event<runtime::Runtime>) -> Self {
        match pde {
            pallet_dao::Event::<runtime::Runtime>::Voted {
                account,
                proposal_hash,
                voted,
                yes,
                no,
            } => Event::Voted(account, proposal_hash, voted, yes, no),
            pallet_dao::Event::<runtime::Runtime>::Proposed {
                account,
                proposal_index,
                proposal_hash,
                threshold,
            } => Event::Proposed(account, proposal_index, proposal_hash, threshold),
            pallet_dao::Event::<runtime::Runtime>::Approved { proposal_hash } => {
                Event::Approved(proposal_hash)
            }
            pallet_dao::Event::<runtime::Runtime>::Disapproved { proposal_hash } => {
                Event::Disapproved(proposal_hash)
            }
            pallet_dao::Event::<runtime::Runtime>::Executed {
                proposal_hash,
                result: _,
            } => Event::Executed(proposal_hash),
            pallet_dao::Event::<runtime::Runtime>::Closed {
                proposal_hash,
                yes,
                yes_weight,
                no,
                no_weight,
            } => Event::Closed(proposal_hash, yes, yes_weight, no, no_weight),
            pallet_dao::Event::<runtime::Runtime>::ClosedByCouncil {
                proposal_hash,
                vetos,
            } => Event::ClosedByCouncil(proposal_hash, vetos),
            pallet_dao::Event::<runtime::Runtime>::CouncilMemberVeto { proposal_hash, who } => {
                Event::CouncilMemberVeto(proposal_hash, who)
            }
            _ => panic!("impossible event"),
        }
    }
}
