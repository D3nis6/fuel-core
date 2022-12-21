//! Contains types related to P2P data

use crate::{
    blockchain::{
        block::Block,
        consensus::Consensus,
    },
    fuel_tx::Transaction,
};
use std::fmt::Debug;

// TODO: Maybe we can remove most of types from here directly into P2P

/// Reporting levels on the status of a message received via Gossip
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GossipsubMessageAcceptance {
    /// Report whether the gossiped message is valid and safe to rebroadcast
    Accept,
    /// Ignore the received message and prevent further gossiping
    Reject,
    /// Punish the gossip sender for providing invalid
    /// (or malicious) data and prevent further gossiping
    Ignore,
}

/// A gossipped message from the network containing all relevant data.
#[derive(Debug, Clone)]
pub struct GossipData<T> {
    /// The gossipped message payload
    /// This is meant to be consumed once to avoid cloning. Subsequent attempts to fetch data from
    /// the message should return None.
    pub data: Option<T>,
    /// The ID of the network peer that sent this message
    pub peer_id: Vec<u8>,
    /// The message id that corresponds to a message payload (typically a unique hash)
    pub message_id: Vec<u8>,
}

/// Consensus header info from the network
pub type ConsensusGossipData = GossipData<Consensus>;
/// Transactions gossiped by peers for inclusion into a block
pub type TransactionGossipData = GossipData<Transaction>;
/// Newly produced block notification
pub type BlockGossipData = GossipData<Block>;

impl<T> GossipData<T> {
    /// Construct a new gossip message
    pub fn new(
        data: T,
        peer_id: impl Into<Vec<u8>>,
        message_id: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            data: Some(data),
            peer_id: peer_id.into(),
            message_id: message_id.into(),
        }
    }
}

/// A generic representation of data that's been gossipped by the network
pub trait NetworkData<T>: Debug + Send {
    /// Consume ownership of data from a gossipped message
    fn take_data(&mut self) -> Option<T>;
}

impl<T: Debug + Send + 'static> NetworkData<T> for GossipData<T> {
    fn take_data(&mut self) -> Option<T> {
        self.data.take()
    }
}
