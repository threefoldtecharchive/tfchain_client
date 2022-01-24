use crate::types::{
    AccountId32, Balance, BurnTransaction, MintTransaction, RefundTransaction, StellarSignature,
};

#[derive(Debug)]
pub enum Event {
    // Minting events
    MintTransactionProposed(Vec<u8>, AccountId32, Balance),
    MintTransactionVoted(Vec<u8>),
    MintCompleted(MintTransaction),
    // not emitted?
    MintTransactionExpired(Vec<u8>, Balance, AccountId32),
    // Burn events
    BurnTransactionCreated(u64, Vec<u8>, Balance),
    BurnTransactionProposed(u64, Vec<u8>, Balance),
    BurnTransactionSignatureAdded(u64, StellarSignature),
    BurnTransactionReady(u64),
    BurnTransactionProcessed(BurnTransaction),
    BurnTransactionExpired(u64, Vec<u8>, Balance),
    // Refund events
    RefundTransactionCreated(Vec<u8>, Vec<u8>, Balance),
    RefundTransactionsignatureAdded(Vec<u8>, StellarSignature),
    RefundTransactionReady(Vec<u8>),
    RefundTransactionProcessed(RefundTransaction),
    RefundTransactionExpired(Vec<u8>, Vec<u8>, Balance),
}

impl From<pallet_tft_bridge::Event<runtime::Runtime>> for Event {
    fn from(tbe: pallet_tft_bridge::Event<runtime::Runtime>) -> Self {
        match tbe {
            pallet_tft_bridge::Event::<runtime::Runtime>::MintTransactionProposed(
                tx_id,
                target,
                amount,
            ) => Event::MintTransactionProposed(tx_id, target, amount.into()),
            pallet_tft_bridge::Event::<runtime::Runtime>::MintTransactionVoted(tx_id) => {
                Event::MintTransactionVoted(tx_id)
            }
            pallet_tft_bridge::Event::<runtime::Runtime>::MintCompleted(tx) => {
                Event::MintCompleted(tx.into())
            }
            pallet_tft_bridge::Event::<runtime::Runtime>::MintTransactionExpired(
                tx_id,
                amount,
                target,
            ) => Event::MintTransactionExpired(tx_id, amount.into(), target),
            pallet_tft_bridge::Event::<runtime::Runtime>::BurnTransactionCreated(
                burn_id,
                stellar_address,
                amount,
            ) => Event::BurnTransactionCreated(burn_id, stellar_address, amount.into()),
            pallet_tft_bridge::Event::<runtime::Runtime>::BurnTransactionProposed(
                tx_id,
                target,
                amount,
            ) => Event::BurnTransactionProposed(tx_id, target, amount.into()),
            pallet_tft_bridge::Event::<runtime::Runtime>::BurnTransactionSignatureAdded(
                tx_id,
                sig,
            ) => Event::BurnTransactionSignatureAdded(tx_id, sig.into()),
            pallet_tft_bridge::Event::<runtime::Runtime>::BurnTransactionReady(tx_id) => {
                Event::BurnTransactionReady(tx_id)
            }
            pallet_tft_bridge::Event::<runtime::Runtime>::BurnTransactionProcessed(tx) => {
                Event::BurnTransactionProcessed(tx.into())
            }
            pallet_tft_bridge::Event::<runtime::Runtime>::BurnTransactionExpired(
                tx_id,
                target,
                amount,
            ) => Event::BurnTransactionExpired(tx_id, target, amount.into()),
            pallet_tft_bridge::Event::<runtime::Runtime>::RefundTransactionCreated(
                tx_hash,
                target,
                amount,
            ) => Event::RefundTransactionCreated(tx_hash, target, amount.into()),
            pallet_tft_bridge::Event::<runtime::Runtime>::RefundTransactionsignatureAdded(
                tx_hash,
                sig,
            ) => Event::RefundTransactionsignatureAdded(tx_hash, sig.into()),
            pallet_tft_bridge::Event::<runtime::Runtime>::RefundTransactionReady(tx_hash) => {
                Event::RefundTransactionReady(tx_hash)
            }
            pallet_tft_bridge::Event::<runtime::Runtime>::RefundTransactionProcessed(tx) => {
                Event::RefundTransactionProcessed(tx.into())
            }
            pallet_tft_bridge::Event::<runtime::Runtime>::RefundTransactionExpired(
                tx_id,
                target,
                amount,
            ) => Event::RefundTransactionExpired(tx_id, target, amount.into()),
        }
    }
}
