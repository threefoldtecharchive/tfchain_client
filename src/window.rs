//! Abstractions for working with chain storage at fixed times (i.e. blocks)

use crate::client::SharedClient;
use crate::events;
use crate::types::{BlockNumber, Hash};
use sp_core::crypto::Pair;
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
    client: SharedClient<P>,
    target: Option<(BlockNumber, Hash)>,
}

impl<P> Window<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    /// Create a new [Window] at the given height. If the used block height does not exist yet on
    /// the chain, Ok(None) is returned.
    pub fn at_height(client: SharedClient<P>, height: BlockNumber) -> WindowResult<Option<Self>> {
        Ok(client.get_hash_at_height(height)?.map(|hash| Window {
            client,
            target: Some((height, hash)),
        }))
    }

    /// Indicates if the [Window] points to data in the past, or the current head. If this is
    /// false, all storage values returned are considered volatile.
    pub fn is_historic(&self) -> bool {
        self.target.is_some()
    }

    /// Get the next {window], i.e. the [Window] for the next block. Repeatedly calling `next` can
    /// be used to iterate over all blocks in the chain.
    pub fn next(&self) -> WindowResult<Option<Self>> {
        let client = self.client.clone();
        if let Some((h, _)) = self.target {
            Self::at_height(client, h + 1)
        } else {
            Err(WindowError::NonHistoricWindow)
        }
    }

    /// Get the previous [Window], i.e. the [Window] for the previous block. Repeatedly calling
    /// `previous` can be used to iterate over all blocks in the chain in reverse order.
    pub fn previous(&self) -> WindowResult<Option<Self>> {
        let client = self.client.clone();
        if let Some((h, _)) = self.target {
            Self::at_height(client, h - 1)
        } else {
            Err(WindowError::NonHistoricWindow)
        }
    }

    /// Get the [events](events::TfchainEvent) for the block pointed at by the window.
    pub fn events(&self) -> WindowResult<Vec<events::TfchainEvent>> {
        Ok(self.client.get_block_events(self.hash())?)
    }

    /// Helper function to get the active hash, for invoking client commands.
    fn hash(&self) -> Option<Hash> {
        self.target.map(|(_, h)| h)
    }
}

/// A `WindowError` contains details about errors when working with [Window]s
pub enum WindowError {
    /// An error while executing a call to the chain
    Api(ApiClientError),
    /// Result of trying to advance or go back from a [Window] pointing to the head of the current
    /// chain.
    NonHistoricWindow,
}

impl From<ApiClientError> for WindowError {
    fn from(ace: ApiClientError) -> Self {
        WindowError::Api(ace)
    }
}
