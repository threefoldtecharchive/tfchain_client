//! Abstractions for working with chain storage at fixed times (i.e. blocks)

use crate::client::SharedClient;
use crate::events;
use crate::types::{
    BlockNumber, Contract, ContractState, Farm, FarmingPolicy, Hash, Node, Resources,
};
use chrono::prelude::*;
use sp_core::crypto::Pair;
use std::fmt;
use substrate_api_client::sp_runtime::MultiSignature;
use substrate_api_client::ApiClientError;

/// The [Result](std::result::Result) type used by [Window] operations.
pub type WindowResult<T> = Result<T, WindowError>;

/// A `Window` gives a view into the blockchain storage at a certain point in time. If a window is
/// pointed at a historic block, the values returned are guaranteed to not change. The only
/// exception to this rule is in case of very recent blocks, which have not been finalized yet.
pub struct Window<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    client: EventTypedClient<P>,
    target: Option<(BlockNumber, Hash)>,
    network: Network,
}

impl<P> Window<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    /// Create a new [Window] at the given height. If the used block height does not exist yet on
    /// the chain, Ok(None) is returned.
    pub fn at_height<C>(
        client: C,
        height: BlockNumber,
        network: Network,
    ) -> WindowResult<Option<Window<P>>>
    where
        C: Into<EventTypedClient<P>>,
    {
        let client = match network {
            Network::Main => {
                if height < 3_037_661 {
                    client.into().as_legacy()
                } else {
                    client.into().as_current()
                }
            }
            Network::Test => {
                if height < 3_615_198 {
                    client.into().as_legacy()
                } else {
                    client.into().as_current()
                }
            }
            Network::Dev => {
                // TODO
                if height < 1_500_000 {
                    client.into().as_legacy()
                } else {
                    client.into().as_current()
                }
            }
        };

        Ok(client.get_hash_at_height(height)?.map(|hash| Window {
            client,
            target: Some((height, hash)),
            network,
        }))
    }

    /// Indicates if the [Window] points to data in the past, or the current head. If this is
    /// false, all storage values returned are considered volatile.
    pub fn is_historic(&self) -> bool {
        self.target.is_some()
    }

    /// Get the next [window], i.e. the [Window] for the next block. Repeatedly calling `next` can
    /// be used to iterate over all blocks in the chain.
    pub fn advance(&self) -> WindowResult<Option<Window<P>>> {
        let client = self.client.clone();
        if let Some((h, _)) = self.target {
            Self::at_height(client, h + 1, self.network)
        } else {
            Err(WindowError::NonHistoricWindow)
        }
    }

    /// Get the [Window] pointing to the block `amount` blocks past the one pointed to by the
    /// current [Window].
    pub fn advance_by(&self, amount: BlockNumber) -> WindowResult<Option<Window<P>>> {
        let client = self.client.clone();
        if let Some((h, _)) = self.target {
            Self::at_height(client, h + amount, self.network)
        } else {
            Err(WindowError::NonHistoricWindow)
        }
    }

    /// Get the previous [Window], i.e. the [Window] for the previous block. Repeatedly calling
    /// `previous` can be used to iterate over all blocks in the chain in reverse order.
    pub fn previous(&self) -> WindowResult<Option<Window<P>>> {
        let client = self.client.clone();
        if let Some((h, _)) = self.target {
            Self::at_height(client, h - 1, self.network)
        } else {
            Err(WindowError::NonHistoricWindow)
        }
    }

    /// Get the previous [Window], pointing to the block `amount` blocks before the one pointed to
    /// by the current [Window].
    pub fn previous_by(&self, amount: BlockNumber) -> WindowResult<Option<Window<P>>> {
        let client = self.client.clone();
        if let Some((h, _)) = self.target {
            Self::at_height(client, h - amount, self.network)
        } else {
            Err(WindowError::NonHistoricWindow)
        }
    }

    /// Get the [events](events::TfchainEvent) for the block pointed at by the window.
    pub fn events(&self) -> WindowResult<Vec<events::TfchainEvent>> {
        Ok(self.client.get_block_events(self.hash())?)
    }

    /// Gets the date at which the block pointed to by this [Window] was made.
    pub fn date(&self) -> WindowResult<DateTime<Utc>> {
        let ts = self.client.block_timestamp(self.hash())?;
        Ok(Utc.timestamp(ts as i64 / 1000, ts as u32 % 1000))
    }

    /// Get the height of the block pointed at by the current [Window].
    pub fn height(&self) -> WindowResult<BlockNumber> {
        if let Some((height, _)) = self.target {
            Ok(height)
        } else {
            todo!();
        }
    }

    /// Get an iterator returning all farms in the current [Window]. If the [Window] is not
    /// historic, slow consumption can lead to innacurate results.
    pub fn farms(&self) -> WindowResult<FarmIterator<P>> {
        let amount = self.client.farm_count(self.hash())?;
        Ok(FarmIterator {
            client: self.client.clone(),
            block: self.hash(),
            amount,
            current: 0,
        })
    }

    /// Get an iterator returning all nodes in the current [Window]. If the [Window] is not
    /// historic, slow consumption can lead to innacurate results.
    pub fn nodes(&self) -> WindowResult<NodeIterator<P>> {
        let amount = self.client.node_count(self.hash())?;
        Ok(NodeIterator {
            client: self.client.clone(),
            block: self.hash(),
            amount,
            current: 0,
        })
    }

    /// Get an iterator returning all contracts in the current [Window]. If the [Window] is not
    /// historic, slow consumption can lead to inaccurate results. If deployed is true, only
    /// contracts currently deployed will be returned.
    pub fn contracts(&self, live: bool) -> WindowResult<ContractIterator<P>> {
        let amount = self.client.contract_count(self.hash())?;
        Ok(ContractIterator {
            client: self.client.clone(),
            block: self.hash(),
            amount,
            current: 0,
            live,
        })
    }

    pub fn farm_policies(&self) -> WindowResult<FarmPolicyIterator<P>> {
        let amount = self.client.farm_policy_count(self.hash())?;
        Ok(FarmPolicyIterator {
            client: self.client.clone(),
            block: self.hash(),
            amount,
            current: 0,
        })
    }

    /// Get the farm stellar address in the block pointed at by the current [Window].
    ///
    /// Setting this is optional and the responsibility of the farmer.
    pub fn farm_payout_address(&self, farm_id: u32) -> WindowResult<Option<String>> {
        let maybe_address = self.client.get_farm_payout_address(farm_id, self.hash())?;
        Ok(maybe_address)
    }

    /// Helper function to get the active hash, for invoking client commands.
    fn hash(&self) -> Option<Hash> {
        self.target.map(|(_, h)| h)
    }
}

// TODO: these 3 iterators could technically be made generic, by taking a Fn with output type as
// generic to the output of the iterator
pub struct NodeIterator<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    client: EventTypedClient<P>,
    block: Option<Hash>,
    amount: u32,
    current: u32,
}

impl<P> Iterator for NodeIterator<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    type Item = WindowResult<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Nodes start at index 1
            self.current += 1;
            if self.current > self.amount {
                return None;
            }

            return match self
                .client
                .get_node_by_id(self.current, self.block)
                .map_err(WindowError::from)
            {
                Ok(maybe_node) => match maybe_node {
                    Some(node) => Some(Ok(node)),
                    None => continue,
                },
                Err(err) => Some(Err(err)),
            };
        }
    }
}

pub struct FarmIterator<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    client: EventTypedClient<P>,
    block: Option<Hash>,
    amount: u32,
    current: u32,
}

impl<P> Iterator for FarmIterator<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    type Item = WindowResult<Farm>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Farms start at index 1
            self.current += 1;
            if self.current > self.amount {
                return None;
            }

            return match self
                .client
                .get_farm_by_id(self.current, self.block)
                .map_err(WindowError::from)
            {
                Ok(maybe_farm) => match maybe_farm {
                    Some(farm) => Some(Ok(farm)),
                    None => continue,
                },
                Err(err) => Some(Err(err)),
            };
        }
    }
}

pub struct ContractIterator<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    client: EventTypedClient<P>,
    block: Option<Hash>,
    amount: u64,
    current: u64,
    live: bool,
}

impl<P> Iterator for ContractIterator<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    type Item = WindowResult<(Contract, Resources)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Contracts start at index 1
            self.current += 1;
            if self.current > self.amount {
                return None;
            }

            return match self
                .client
                .get_contract_by_id(self.current, self.block)
                .map_err(WindowError::from)
            {
                Ok(maybe_contract) => match maybe_contract {
                    Some((contract, cr)) => {
                        if self.live {
                            if matches!(contract.state, ContractState::Created) {
                                Some(Ok((contract, cr)))
                            } else {
                                continue;
                            }
                        } else {
                            Some(Ok((contract, cr)))
                        }
                    }
                    None => continue,
                },
                Err(err) => Some(Err(err)),
            };
        }
    }
}

pub struct FarmPolicyIterator<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    client: EventTypedClient<P>,
    block: Option<Hash>,
    amount: u32,
    current: u32,
}

impl<P> Iterator for FarmPolicyIterator<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    type Item = WindowResult<FarmingPolicy<u32>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Farms start at index 1
            self.current += 1;
            if self.current > self.amount {
                return None;
            }

            return match self
                .client
                .get_farm_policy_by_id(self.current, self.block)
                .map_err(WindowError::from)
            {
                Ok(maybe_policy) => match maybe_policy {
                    Some(policy) => Some(Ok(policy)),
                    None => continue,
                },
                Err(err) => Some(Err(err)),
            };
        }
    }
}

/// A `WindowError` contains details about errors when working with [Window]s
#[derive(Debug)]
pub enum WindowError {
    /// An error while executing a call to the chain
    Api(ApiClientError),
    /// Result of trying to advance or go back from a [Window] pointing to the head of the current
    /// chain.
    NonHistoricWindow,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WindowError::Api(ref apie) => apie as &dyn fmt::Display,
                WindowError::NonHistoricWindow =>
                    &"method call expected historic window, found window pointing to head"
                        as &dyn fmt::Display,
            }
        )
    }
}

impl std::error::Error for WindowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WindowError::Api(ref apie) => Some(apie),
            _ => None,
        }
    }
}

impl From<ApiClientError> for WindowError {
    fn from(ace: ApiClientError) -> Self {
        WindowError::Api(ace)
    }
}

/// Grid networks, mainly used to identify
#[derive(Debug, Clone, Copy)]
pub enum Network {
    Main,
    Test,
    Dev,
}

/// The client with actual event types.
#[derive(Clone)]
pub enum EventTypedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    Current(SharedClient<P, runtime::Event>),
    Legacy(SharedClient<P, runtime_legacy::Event>),
}

/// State transition functions
impl<P> EventTypedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    /// Convert a client to use the latest runtime, if it was not already using that.
    fn as_current(self) -> Self {
        match self {
            EventTypedClient::Current(_) => self,
            EventTypedClient::Legacy(sc) => EventTypedClient::Current(sc.with_events()),
        }
    }

    /// Convert a client to use the legacy runtime, if it was not already using that.
    fn as_legacy(self) -> Self {
        match self {
            EventTypedClient::Current(sc) => EventTypedClient::Legacy(sc.with_events()),
            EventTypedClient::Legacy(_) => self,
        }
    }
}

impl<P> EventTypedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    fn block_timestamp(&self, hash: Option<Hash>) -> crate::client::ApiResult<i64> {
        match self {
            EventTypedClient::Current(ref sc) => sc.block_timestamp(hash),
            EventTypedClient::Legacy(ref sc) => sc.block_timestamp(hash),
        }
    }

    fn get_farm_by_id(
        &self,
        farm_id: u32,
        block: Option<Hash>,
    ) -> crate::client::ApiResult<Option<Farm>> {
        match self {
            EventTypedClient::Current(ref sc) => sc.get_farm_by_id(farm_id, block),
            EventTypedClient::Legacy(ref sc) => sc.get_farm_by_id(farm_id, block),
        }
    }

    fn get_node_by_id(
        &self,
        node_id: u32,
        block: Option<Hash>,
    ) -> crate::client::ApiResult<Option<Node>> {
        match self {
            EventTypedClient::Current(ref sc) => sc.get_node_by_id(node_id, block),
            EventTypedClient::Legacy(ref sc) => sc.get_node_by_id(node_id, block),
        }
    }

    fn get_contract_by_id(
        &self,
        contract_id: u64,
        block: Option<Hash>,
    ) -> crate::client::ApiResult<Option<(Contract, Resources)>> {
        match self {
            EventTypedClient::Current(ref sc) => {
                match sc.get_contract_by_id(contract_id, block)? {
                    None => Ok(None),
                    Some(c) => Ok(Some((
                        c,
                        sc.get_contract_resources(contract_id, block)?
                            .unwrap_or_default()
                            .used,
                    ))),
                }
            }
            EventTypedClient::Legacy(ref sc) => match sc.get_contract_by_id(contract_id, block)? {
                None => Ok(None),
                Some(c) => Ok(Some((
                    c,
                    sc.get_contract_resources(contract_id, block)?
                        .unwrap_or_default()
                        .used,
                ))),
            },
        }
    }

    fn get_farm_policy_by_id(
        &self,
        farm_policy_id: u32,
        block: Option<Hash>,
    ) -> crate::client::ApiResult<Option<FarmingPolicy<BlockNumber>>> {
        match self {
            EventTypedClient::Current(ref sc) => sc.get_farming_policy(farm_policy_id, block),
            EventTypedClient::Legacy(ref sc) => sc.get_farming_policy(farm_policy_id, block),
        }
    }

    fn get_farm_payout_address(
        &self,
        farm_id: u32,
        block: Option<Hash>,
    ) -> crate::client::ApiResult<Option<String>> {
        match self {
            EventTypedClient::Current(ref sc) => sc.get_farm_payout_address(farm_id, block),
            EventTypedClient::Legacy(ref sc) => sc.get_farm_payout_address(farm_id, block),
        }
    }

    fn contract_count(&self, block: Option<Hash>) -> crate::client::ApiResult<u64> {
        match self {
            EventTypedClient::Current(ref sc) => sc.contract_count(block),
            EventTypedClient::Legacy(ref sc) => sc.contract_count(block),
        }
    }

    fn node_count(&self, block: Option<Hash>) -> crate::client::ApiResult<u32> {
        match self {
            EventTypedClient::Current(ref sc) => sc.node_count(block),
            EventTypedClient::Legacy(ref sc) => sc.node_count(block),
        }
    }

    fn farm_count(&self, block: Option<Hash>) -> crate::client::ApiResult<u32> {
        match self {
            EventTypedClient::Current(ref sc) => sc.farm_count(block),
            EventTypedClient::Legacy(ref sc) => sc.farm_count(block),
        }
    }

    fn farm_policy_count(&self, block: Option<Hash>) -> crate::client::ApiResult<u32> {
        match self {
            EventTypedClient::Current(ref sc) => sc.farm_policy_count(block),
            EventTypedClient::Legacy(ref sc) => sc.farm_policy_count(block),
        }
    }

    fn get_block_events(
        &self,
        block: Option<Hash>,
    ) -> crate::client::ApiResult<Vec<events::TfchainEvent>> {
        match self {
            EventTypedClient::Current(ref sc) => sc.get_block_events(block),
            EventTypedClient::Legacy(ref sc) => sc.get_block_events(block),
        }
    }

    fn get_hash_at_height(&self, height: u32) -> crate::client::ApiResult<Option<Hash>> {
        match self {
            EventTypedClient::Current(ref sc) => sc.get_hash_at_height(height),
            EventTypedClient::Legacy(ref sc) => sc.get_hash_at_height(height),
        }
    }
}

impl<P> From<SharedClient<P, runtime::Event>> for EventTypedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    fn from(sc: SharedClient<P, runtime::Event>) -> Self {
        EventTypedClient::Current(sc)
    }
}

impl<P> From<SharedClient<P, runtime_legacy::Event>> for EventTypedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    fn from(sc: SharedClient<P, runtime_legacy::Event>) -> Self {
        EventTypedClient::Legacy(sc)
    }
}
