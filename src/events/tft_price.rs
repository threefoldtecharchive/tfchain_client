use crate::types::AccountId32;

#[derive(Debug)]
pub enum Event {
    /// Price of tft has been stored \[price\].
    PriceStored(f64),
    /// Off chain worker has executed \[offchain worker account\].
    OffchainWorkerExecuted(AccountId32),
}

impl From<pallet_tft_price::Event<runtime::Runtime>> for Event {
    fn from(tpe: pallet_tft_price::Event<runtime::Runtime>) -> Self {
        match tpe {
            pallet_tft_price::Event::<runtime::Runtime>::PriceStored(price) => {
                Event::PriceStored(price.to_string().parse::<f64>().unwrap())
            }
            pallet_tft_price::Event::<runtime::Runtime>::OffchainWorkerExecuted(id) => {
                Event::OffchainWorkerExecuted(id)
            }
        }
    }
}
