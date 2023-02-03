use std::collections::{BTreeMap, HashMap};

use anchor_lang::prelude::*;
use ibc::{
    core::{
        ics02_client::{
            client_state::ClientState, client_type::ClientType, consensus_state::ConsensusState,
        },
        ics03_connection::connection::ConnectionEnd,
        ics04_channel::{
            channel::ChannelEnd,
            commitment::{AcknowledgementCommitment, PacketCommitment},
            packet::{Receipt, Sequence},
        },
        ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
        ics26_routing::context::ModuleId,
    },
    timestamp::Timestamp,
    Height,
};

mod channel;
mod client;
mod connection;
mod port;
mod router;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ibc_solana {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deliver(
        ctx: Context<Deliver>,
        messages: Vec<ibc_proto::google::protobuf::Any>,
    ) -> Result<()> {
        let ctx = &mut ctx.accounts.ibc_context.context;

        let (events, logs, errors) = messages.into_iter().fold(
            (vec![], vec![], vec![]),
            |(mut events, mut logs, mut errors), msg| {
                match ibc::core::ics26_routing::handler::deliver(ctx, msg) {
                    Ok(ibc::core::ics26_routing::handler::MsgReceipt {
                        events: temp_events,
                        log: temp_logs,
                    }) => {
                        events.extend(temp_events);
                        logs.extend(temp_logs);
                    }
                    Err(e) => errors.push(e),
                }
                (events, logs, errors)
            },
        );
        msg!("[deliver]: events: {:?}", events);
        msg!("[deliver]: logs: {:?}", logs);
        msg!("[deliver]: errors: {:?}", errors);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]

pub struct Deliver<'info> {
    ibc_context: Account<'info, CallIbcContext>,
}

#[account]
pub struct CallIbcContext {
    context: IbcStore,
}

/// A mock of an IBC client record as it is stored in a mock context.
/// For testing ICS02 handlers mostly, cf. `MockClientContext`.
#[derive(Clone, Debug)]
pub struct ClientRecord {
    /// The type of this client.
    pub client_type: ClientType,

    /// The client state (representing only the latest height at the moment).
    pub client_state: Option<Box<dyn ClientState>>,

    /// Mapping of heights to consensus states for this client.
    pub consensus_states: HashMap<Height, Box<dyn ConsensusState>>,
}

type PortChannelIdMap<V> = BTreeMap<PortId, BTreeMap<ChannelId, V>>;

type AnyClientState = Vec<u8>;
type AnyConsensusState = Vec<u8>;

/// An object that stores all IBC related data.
#[account]
pub struct IbcStore {
    /// The set of all clients, indexed by their id.
    pub client_types: BTreeMap<ClientId, ClientType>,

    pub client_state: BTreeMap<ClientId, AnyClientState>,

    pub consensus_states: BTreeMap<ClientId, BTreeMap<Height, AnyConsensusState>>,

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

    /// All the channels in the store. TODO Make new key PortId X ChanneId
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
