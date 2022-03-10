use crate::events::TftPriceEvent;

impl From<pallet_tft_price_legacy::Event<runtime_legacy::Runtime>> for TftPriceEvent {
    fn from(tpe: pallet_tft_price_legacy::Event<runtime_legacy::Runtime>) -> Self {
        match tpe {
            pallet_tft_price_legacy::Event::<runtime_legacy::Runtime>::PriceStored(price) => {
                TftPriceEvent::PriceStored(price.to_string().parse::<f64>().unwrap())
            }
            pallet_tft_price_legacy::Event::<runtime_legacy::Runtime>::OffchainWorkerExecuted(
                id,
            ) => TftPriceEvent::OffchainWorkerExecuted(id),
        }
    }
}
