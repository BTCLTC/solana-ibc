use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use ibc::core::ics02_client::client_state::{ClientState, UpdateKind, UpdatedState};
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::consensus_state::ConsensusState;
use ibc::core::router::Module;
use ibc::core::router::ModuleId;
use ibc::core::{
    events::IbcEvent,
    ics02_client::height::Height,
    ics03_connection::connection::ConnectionEnd,
    ics04_channel::{
        channel::ChannelEnd,
        commitment::{AcknowledgementCommitment, PacketCommitment},
        packet::{Receipt, Sequence},
    },
    ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
    timestamp::Timestamp,
    MsgEnvelope, RouterError,
};
use std::sync::Arc;
use std::time::Duration;

mod context;
mod error;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ibc_solana {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]

pub struct Deliver<'info> {
    ibc_context: Account<'info, SolanaIbcStore>,
}

#[derive(Debug)]
struct HostBlock {}

/// A context implementing the dependencies necessary for testing any IBC module.
#[derive(Debug)]
pub struct SolanaIbcContext {
    // / The type of host chain underlying this mock context.
    // host_chain_type: HostType,
    /// Host chain identifier.
    host_chain_id: ChainId,

    /// Maximum size for the history of the host chain. Any block older than this is pruned.
    max_history_size: usize,

    /// The chain of blocks underlying this context. A vector of size up to `max_history_size`
    /// blocks, ascending order by their height (latest block is on the last position).
    history: Vec<HostBlock>,

    /// Average time duration between blocks
    block_time: Duration,

    /// An object that stores all IBC related data.
    pub ibc_store: SolanaIbcStore,

    /// To implement ValidationContext Router
    router: BTreeMap<ModuleId, Arc<dyn Module>>,

    pub events: Vec<IbcEvent>,

    pub logs: Vec<String>,
}

type PortChannelIdMap<V> = BTreeMap<PortId, BTreeMap<ChannelId, V>>;

/// An object that stores all IBC related data.
#[account]
#[derive(Debug, Default)]
pub struct SolanaIbcStore {
    /// The set of all client states, indexed by their id.
    pub client_states: BTreeMap<ClientId, Vec<u8>>,

    /// The set of all consensus state, indexed by their id.
    pub consensus_states: BTreeMap<ClientId, BTreeMap<Height, Vec<u8>>>,

    /// The set of all client types, indexed by their id.
    pub client_types: BTreeMap<ClientId, ClientType>,

    /// Tracks the processed time for clients header updates
    pub client_processed_times: BTreeMap<(ClientId, Height), Timestamp>,

    /// Tracks the processed height for the clients
    pub client_processed_heights: BTreeMap<(ClientId, Height), Height>,

    /// Counter for the client identifiers, necessary for `increase_client_counter` and the
    /// `client_counter` methods.
    pub client_ids_counter: u64,

    /// Association between client ids and connection ids.
    pub client_connections: BTreeMap<ClientId, ConnectionId>,

    /// All the connections in the store.
    pub connections: BTreeMap<ConnectionId, ConnectionEnd>,

    /// Counter for connection identifiers (see `increase_connection_counter`).
    pub connection_ids_counter: u64,

    /// Association between connection ids and channel ids.
    pub connection_channels: BTreeMap<ConnectionId, Vec<(PortId, ChannelId)>>,

    /// Counter for channel identifiers (see `increase_channel_counter`).
    pub channel_ids_counter: u64,

    /// All the channels in the store. TODO Make new key PortId X ChannelId
    pub channels: PortChannelIdMap<ChannelEnd>,

    /// Tracks the sequence number for the next packet to be sent.
    pub next_sequence_send: PortChannelIdMap<Sequence>,

    /// Tracks the sequence number for the next packet to be received.
    pub next_sequence_recv: PortChannelIdMap<Sequence>,

    /// Tracks the sequence number for the next packet to be acknowledged.
    pub next_sequence_ack: PortChannelIdMap<Sequence>,

    pub packet_acknowledgement: PortChannelIdMap<BTreeMap<Sequence, AcknowledgementCommitment>>,

    /// Maps ports to the the module that owns it
    pub port_to_module: BTreeMap<PortId, ModuleId>,

    /// Constant-size commitments to packets data fields
    pub packet_commitment: PortChannelIdMap<BTreeMap<Sequence, PacketCommitment>>,

    // Used by unordered channel
    pub packet_receipt: PortChannelIdMap<BTreeMap<Sequence, Receipt>>,
}