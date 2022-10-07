use subxt::{
    rpc::{BlockNumber, NumberOrHex},
    OnlineClient,
};

pub use subxt::events::Events;
pub use subxt::rpc::PolkadotConfig;

pub mod testnet;

const BLOCK_TIME_SECONDS: i64 = 6;

pub enum Runtime {
    Devnet,
    Testnet,
    Mainnet,
}

pub struct Client {
    api: OnlineClient<PolkadotConfig>,
    runtime: Runtime,
}

impl Client {
    pub async fn new(url: &str, runtime: Runtime) -> Result<Self, Box<dyn std::error::Error>> {
        let api = OnlineClient::from_url(url).await?;
        Ok(Client { api, runtime })
    }

    pub async fn get_events(
        &self,
        block: Option<u32>,
    ) -> Result<Events<PolkadotConfig>, Box<dyn std::error::Error>> {
        Ok(self
            .api
            .events()
            .at(self.hash_at_height(block).await?)
            .await?)
    }

    pub async fn hash_at_height(
        &self,
        block: Option<u32>,
    ) -> Result<Option<subxt::ext::sp_core::H256>, Box<dyn std::error::Error>> {
        Ok(self
            .api
            .rpc()
            .block_hash(block.map(|block| BlockNumber::from(NumberOrHex::from(block))))
            .await?)
    }

    pub async fn height_at_timestamp(&self, ts: i64) -> Result<u32, Box<dyn std::error::Error>> {
        let latest_ts = self.block_timestamp(None).await? / 1000;
        if latest_ts < ts {
            panic!(
                "can't fetch block for future timestamp {} vs latest {}",
                ts, latest_ts
            );
        }
        let mut height = 1;
        let mut last_height = 1;
        loop {
            let hash = match self.hash_at_height(Some(height)).await? {
                Some(hash) => hash,
                None => {
                    height = (height + last_height) / 2;
                    continue;
                }
            };
            // TODO: fetch based on hash
            let block_time = self.block_timestamp(Some(height)).await? / 1000;
            let time_delta = ts - block_time;
            let block_delta = time_delta / BLOCK_TIME_SECONDS;
            if block_delta == 0 {
                if time_delta >= 0 {
                    return Ok(height + 1);
                } else {
                    return Ok(height);
                }
            }
            if (height as i64 + block_delta) < 0 {
                panic!(
                    "negative height search (height {} delta {})",
                    height, block_delta
                );
            }

            last_height = height;

            height = (height as i64 + block_delta) as u32;
        }
    }

    pub async fn block_timestamp(
        &self,
        block: Option<u32>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(self
            .api
            .storage()
            .fetch(
                &testnet::api::storage().timestamp().now(),
                self.hash_at_height(block).await?,
            )
            .await?
            .map(|u| u as i64)
            .unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn event_decode() {
        let client = Client::new("wss://tfchain.test.grid.tf:443", Runtime::Testnet)
            .await
            .unwrap();

        let evts = client.get_events(Some(4240025)).await.unwrap();
        for evt in evts.iter() {
            println!(
                "{} - {}",
                evt.as_ref().unwrap().pallet_name(),
                evt.as_ref().unwrap().variant_name()
            );
        }
        let evts = client.get_events(Some(4240026)).await.unwrap();
        for evt in evts.iter() {
            let evt = evt.as_ref().unwrap();
            println!("{} - {}", evt.pallet_name(), evt.variant_name());
            if evt.variant_name() == "NruConsumptionReportReceived"
                && evt.pallet_name() == "SmartContractModule"
            {
                let nru_report = evt.as_event::<testnet::api::smart_contract_module::events::NruConsumptionReportReceived>().unwrap().unwrap();
                println!(
                    "Got nru consumption report for contract {}, consumed {} bytes",
                    nru_report.0.contract_id, nru_report.0.nru
                );
            }
        }
    }
    #[tokio::test]
    async fn get_nodes() {
        let client = Client::new("wss://tfchain.test.grid.tf:443", Runtime::Testnet)
            .await
            .unwrap();
        //let nodes = client.nodes(4600000).await.unwrap();
    }
}
