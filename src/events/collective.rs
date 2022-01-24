use crate::types::{AccountId32, Hash};
pub type MemberCount = u32;
pub type ProposalIndex = u32;

#[derive(Debug)]
pub enum Event {
    /// A motion (given hash) has been proposed (by given account) with a threshold (given
    /// `MemberCount`).
    /// \[account, proposal_index, proposal_hash, threshold\]
    Proposed(AccountId32, ProposalIndex, Hash, MemberCount),
    /// A motion (given hash) has been voted on by given account, leaving
    /// a tally (yes votes and no votes given respectively as `MemberCount`).
    /// \[account, proposal_hash, voted, yes, no\]
    Voted(AccountId32, Hash, bool, MemberCount, MemberCount),
    /// A motion was approved by the required threshold.
    /// \[proposal_hash\]
    Approved(Hash),
    /// A motion was not approved by the required threshold.
    /// \[proposal_hash\]
    Disapproved(Hash),
    /// A motion was executed; result will be `Ok` if it returned without error.
    /// \[proposal_hash, result\]
    //Executed(Hash, DispatchResult),
    Executed(Hash),
    /// A single member did some action; result will be `Ok` if it returned without error.
    /// \[proposal_hash, result\]
    //MemberExecuted(Hash, DispatchResult),
    MemberExecuted(Hash),
    /// A proposal was closed because its threshold was reached or after its duration was up.
    /// \[proposal_hash, yes, no\]
    Closed(Hash, MemberCount, MemberCount),
}

impl From<pallet_collective::Event<runtime::Runtime, pallet_collective::Instance1>> for Event {
    fn from(ce: pallet_collective::Event<runtime::Runtime, pallet_collective::Instance1>) -> Self {
        match ce {
            pallet_collective::Event::<runtime::Runtime, pallet_collective::Instance1>::Proposed(who, idx, hash, treshold) => {
                Event::Proposed(who, idx, hash, treshold)
            }
            pallet_collective::Event::<runtime::Runtime, pallet_collective::Instance1>::Voted(
                who,
                hash,
                accept,
                votes_for,
                votes_against,
            ) => Event::Voted(who, hash, accept, votes_for, votes_against),
            pallet_collective::Event::<runtime::Runtime, pallet_collective::Instance1>::Approved(hash) => Event::Approved(hash),
            pallet_collective::Event::<runtime::Runtime, pallet_collective::Instance1>::Disapproved(hash) => {
                Event::Disapproved(hash)
            }
            pallet_collective::Event::<runtime::Runtime, pallet_collective::Instance1>::Executed(hash, _) => {
                Event::Executed(hash)
            }
            pallet_collective::Event::<runtime::Runtime, pallet_collective::Instance1>::MemberExecuted(hash, _) => {
                Event::MemberExecuted(hash)
            }
            pallet_collective::Event::<runtime::Runtime, pallet_collective::Instance1>::Closed(
                hash,
                votes_for,
                votes_against,
            ) => Event::Closed(hash, votes_for, votes_against),
            _ => unreachable!(),
        }
    }
}
