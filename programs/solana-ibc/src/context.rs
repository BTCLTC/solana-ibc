// use crate::{prelude::*, Config, PacketCommitment as PacketCommitStore, TENDERMINT_CLIENT_TYPE, *};
use crate::SolanaIbcContext;
use core::time::Duration;
use ibc::core::router::ModuleId;
use ibc::{
    applications::transfer::{
        MODULE_ID_STR as TRANSFER_MODULE_ID, PORT_ID_STR as TRANSFER_PORT_ID,
    },
    clients::ics07_tendermint::{
        client_state::ClientState as Ics07ClientState,
        consensus_state::ConsensusState as Ics07ConsensusState,
    },
    core::{
        events::IbcEvent,
        ics02_client::{
            client_state::ClientState, client_type::ClientType, consensus_state::ConsensusState,
            error::ClientError,
        },
        ics03_connection::{connection::ConnectionEnd, error::ConnectionError},
        ics04_channel::{
            channel::ChannelEnd,
            commitment::{AcknowledgementCommitment, PacketCommitment},
            error::{ChannelError, PacketError},
            packet::{Receipt, Sequence},
        },
        ics23_commitment::commitment::CommitmentPrefix,
        ics24_host::{
            identifier::{ClientId, ConnectionId, PortId},
            path::{
                AckPath, ChannelEndPath, ClientConnectionPath, ClientConsensusStatePath,
                ClientStatePath, CommitmentPath, ConnectionPath, ReceiptPath, SeqAckPath,
                SeqRecvPath, SeqSendPath,
            },
        },
        router::Module,
        timestamp::Timestamp,
        ContextError, ExecutionContext, ValidationContext,
    },
    Height, Signer,
};
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use ics06_solomachine::cosmos::crypto::PublicKey;
use std::collections::BTreeMap;
use std::sync::Arc;
// use pallet_ibc_utils::module::{AddModule, Router};

// #[derive(Clone, Debug)]
// pub struct Context<T: Config> {
//     pub _pd: PhantomData<T>,
//     pub router: Router,
// }

// impl<T: Config> Context<T> {
//     pub fn new() -> Self {
//         let router = Router::new();
//         let r = T::IbcModule::add_module(router);
//         Self {
//             _pd: PhantomData::default(),
//             router: r,
//         }
//     }

//     pub fn add_route(
//         &mut self,
//         module_id: ModuleId,
//         module: impl Module + 'static,
//     ) -> Result<(), String> {
//         match self.router.router.insert(module_id, Arc::new(module)) {
//             None => Ok(()),
//             Some(_) => Err("Duplicate module_id".to_owned()),
//         }
//     }

//     fn client_type(&self, client_id: &ClientId) -> Result<ClientType, ClientError> {
//         let data = <ClientTypeById<T>>::get(client_id.clone()).ok_or(ClientError::Other {
//             description: format!("Client({}) not found!", client_id.clone()),
//         })?;
//         match data.as_str() {
//             TENDERMINT_CLIENT_TYPE => {
//                 ClientType::new(TENDERMINT_CLIENT_TYPE.into()).map_err(|e| ClientError::Other {
//                     description: format!("{}", e),
//                 })
//             }
//             unimplemented => {
//                 return Err(ClientError::UnknownClientStateType {
//                     client_state_type: unimplemented.to_string(),
//                 })
//             }
//         }
//     }
// }

// impl<T: Config> Default for Context<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl ibc::core::router::Router for SolanaIbcContext {
    /// Returns a reference to a `Module` registered against the specified `ModuleId`
    fn get_route(&self, module_id: &ModuleId) -> Option<&dyn Module> {
        self.router.get(module_id).map(Arc::as_ref)
    }

    /// Returns a mutable reference to a `Module` registered against the specified `ModuleId`
    fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn Module> {
        // NOTE: The following:

        // self.router.get_mut(module_id).and_then(Arc::get_mut)

        // doesn't work due to a compiler bug. So we expand it out manually.

        match self.router.get_mut(module_id) {
            Some(arc_mod) => match Arc::get_mut(arc_mod) {
                Some(m) => Some(m),
                None => None,
            },
            None => None,
        }
    }

    /// Returns true if the `Router` has a `Module` registered against the specified `ModuleId`
    fn has_route(&self, module_id: &ModuleId) -> bool {
        self.router.get(module_id).is_some()
    }

    /// Return the module_id associated with a given port_id
    fn lookup_module_by_port(&self, port_id: &PortId) -> Option<ModuleId> {
        // self.ibc_store.lock().port_to_module.get(port_id).cloned()
        todo!()
    }
}

impl ValidationContext for SolanaIbcContext {
    /// Returns the ClientState for the given identifier `client_id`.
    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, ContextError> {
        match self.ibc_store.client_states.get(client_id) {
            Some(client_record) => {
                // todo (davirain) need decode contruct type by clienttype
                todo!()
            }
            None => Err(ClientError::ClientStateNotFound {
                client_id: client_id.clone(),
            }),
        }
        .map_err(ContextError::ClientError)
    }

    /// Tries to decode the given `client_state` into a concrete light client state.
    fn decode_client_state(&self, client_state: Any) -> Result<Box<dyn ClientState>, ContextError> {
        if let Ok(client_state) = Ics07ClientState::try_from(client_state.clone()) {
            Ok(client_state.into_box())
        } else {
            Err(ClientError::UnknownClientStateType {
                client_state_type: client_state.type_url,
            }
            .into())
        }
    }

    /// Retrieve the consensus state for the given client ID at the specified
    /// height.
    ///
    /// Returns an error if no such state exists.
    fn consensus_state(
        &self,
        client_cons_state_path: &ClientConsensusStatePath,
    ) -> Result<Box<dyn ConsensusState>, ContextError> {
        let client_id = &client_cons_state_path.client_id;
        let height = Height::new(client_cons_state_path.epoch, client_cons_state_path.height)?;
        match self.ibc_store.consensus_states.get(client_id) {
            Some(client_record) => match client_record.get(&height) {
                Some(consensus_state) => {
                    // Ok(consensus_state.clone())
                    // todo davirian
                    todo!()
                }
                None => Err(ClientError::ConsensusStateNotFound {
                    client_id: client_id.clone(),
                    height,
                }),
            },
            None => Err(ClientError::ConsensusStateNotFound {
                client_id: client_id.clone(),
                height,
            }),
        }
        .map_err(ContextError::ClientError)
    }

    /// Search for the lowest consensus state higher than `height`.
    fn next_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, ContextError> {
        let ibc_store = self.ibc_store.clone();
        let client_record = ibc_store.consensus_states.get(client_id).ok_or_else(|| {
            ClientError::ClientStateNotFound {
                client_id: client_id.clone(),
            }
        })?;

        // Get the consensus state heights and sort them in ascending order.
        let mut heights: Vec<Height> = client_record.keys().cloned().collect();
        heights.sort();

        // Search for next state.
        for h in heights {
            if h > *height {
                // unwrap should never happen, as the consensus state for h must exist
                // return Ok(Some(client_record.get(&h).unwrap().clone()));
                todo!()
            }
        }
        Ok(None)
    }

    /// Search for the highest consensus state lower than `height`.
    fn prev_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, ContextError> {
        let ibc_store = self.ibc_store.clone();
        let client_record = ibc_store.consensus_states.get(client_id).ok_or_else(|| {
            ClientError::ClientStateNotFound {
                client_id: client_id.clone(),
            }
        })?;

        // Get the consensus state heights and sort them in descending order.
        let mut heights: Vec<Height> = client_record.keys().cloned().collect();
        heights.sort_by(|a, b| b.cmp(a));

        // Search for previous state.
        for h in heights {
            if h < *height {
                // unwrap should never happen, as the consensus state for h must exist
                // decode by client type and returen
                // return Ok(Some(client_record.get(&h).unwrap().clone()));
                todo!()
            }
        }
        Ok(None)
    }

    /// Returns the current height of the local chain.
    fn host_height(&self) -> Result<Height, ContextError> {
        // ref: https://github.com/cosmos/ibc-rs/blob/349d7f5b259e2a34f1f9cbe730c1487e7ecffa38/crates/ibc/src/mock/context.rs#L827
        todo!()
    }

    /// Returns the current timestamp of the local chain.
    fn host_timestamp(&self) -> Result<Timestamp, ContextError> {
        // ref: https://github.com/cosmos/ibc-rs/blob/349d7f5b259e2a34f1f9cbe730c1487e7ecffa38/crates/ibc/src/mock/context.rs#L831
        todo!()
    }

    /// Returns the `ConsensusState` of the host (local) chain at a specific height.
    // ref: https://github.com/cosmos/ibc-rs/blob/349d7f5b259e2a34f1f9cbe730c1487e7ecffa38/crates/ibc/src/mock/context.rs#L841
    fn host_consensus_state(
        &self,
        _height: &Height,
    ) -> Result<Box<dyn ConsensusState>, ContextError> {
        //ref: https://github.com/octopus-network/hermes/commit/7d7891ff29e79f8dd13d6826f75bce8544d54826
        use ics06_solomachine::consensus_state::ConsensusState as SolConsensusState;
        // todo(davirain) need fix
        let fix_public_key = "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"
		A5W0C7iEAuonX56sR81PiwaKTE0GvZlCYuGwHTMpWJo+\"}";
        let fix_public_key =
            fix_public_key
                .parse::<PublicKey>()
                .map_err(|e| ClientError::Other {
                    description: format!(" parse Publickey failed ({})", e),
                })?;
        let host_timestamp = self.host_timestamp()?;
        let consensus_state =
            SolConsensusState::new(fix_public_key, "substrate".to_string(), host_timestamp);
        Ok(Box::new(consensus_state))
        // Err(ClientError::Other { description: "no impl".to_string() }.into())
    }

    /// Returns a natural number, counting how many clients have been created
    /// thus far. The value of this counter should increase only via method
    /// `ExecutionContext::increase_client_counter`.
    fn client_counter(&self) -> Result<u64, ContextError> {
        Ok(self.ibc_store.client_ids_counter)
    }

    /// Returns the ConnectionEnd for the given identifier `conn_id`.
    fn connection_end(&self, conn_id: &ConnectionId) -> Result<ConnectionEnd, ContextError> {
        match self.ibc_store.connections.get(conn_id) {
            Some(connection_end) => Ok(connection_end.clone()),
            None => Err(ConnectionError::ConnectionNotFound {
                connection_id: conn_id.clone(),
            }),
        }
        .map_err(ContextError::ConnectionError)
    }

    /// Validates the `ClientState` of the client (a client referring to host) stored on the
    /// counterparty chain against the host's internal state.
    ///
    /// For more information on the specific requirements for validating the
    /// client state of a host chain, please refer to the [ICS24 host
    /// requirements](https://github.com/cosmos/ibc/tree/main/spec/core/ics-024-host-requirements#client-state-validation)
    ///
    /// Additionally, implementations specific to individual chains can be found
    /// in the [hosts](crate::hosts) module.
    fn validate_self_client(
        &self,
        _client_state_of_host_on_counterparty: Any,
    ) -> Result<(), ContextError> {
        // todo(davirain) need Add
        // ref: https://github.com/cosmos/ibc-rs/blob/349d7f5b259e2a34f1f9cbe730c1487e7ecffa38/crates/ibc/src/mock/context.rs#L867
        Ok(())
    }

    /// Returns the prefix that the local chain uses in the KV store.
    fn commitment_prefix(&self) -> CommitmentPrefix {
        CommitmentPrefix::try_from(b"solana".to_vec()).expect("Never failed")
    }

    /// Returns a counter on how many connections have been created thus far.
    fn connection_counter(&self) -> Result<u64, ContextError> {
        Ok(self.ibc_store.connection_ids_counter)
    }

    /// Returns the `ChannelEnd` for the given `port_id` and `chan_id`.
    fn channel_end(&self, channel_end_path: &ChannelEndPath) -> Result<ChannelEnd, ContextError> {
        let port_id = &channel_end_path.0;
        let channel_id = &channel_end_path.1;

        match self
            .ibc_store
            .channels
            .get(port_id)
            .and_then(|map| map.get(channel_id))
        {
            Some(channel_end) => Ok(channel_end.clone()),
            None => Err(ChannelError::ChannelNotFound {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
            }),
        }
        .map_err(ContextError::ChannelError)
    }

    /// Returns the sequence number for the next packet to be sent for the given store path
    fn get_next_sequence_send(
        &self,
        seq_send_path: &SeqSendPath,
    ) -> Result<Sequence, ContextError> {
        let port_id = &seq_send_path.0;
        let channel_id = &seq_send_path.1;

        match self
            .ibc_store
            .next_sequence_send
            .get(port_id)
            .and_then(|map| map.get(channel_id))
        {
            Some(sequence) => Ok(*sequence),
            None => Err(PacketError::MissingNextSendSeq {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
            }),
        }
        .map_err(ContextError::PacketError)
    }

    /// Returns the sequence number for the next packet to be received for the given store path
    fn get_next_sequence_recv(
        &self,
        seq_recv_path: &SeqRecvPath,
    ) -> Result<Sequence, ContextError> {
        let port_id = &seq_recv_path.0;
        let channel_id = &seq_recv_path.1;

        match self
            .ibc_store
            .next_sequence_recv
            .get(port_id)
            .and_then(|map| map.get(channel_id))
        {
            Some(sequence) => Ok(*sequence),
            None => Err(PacketError::MissingNextRecvSeq {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
            }),
        }
        .map_err(ContextError::PacketError)
    }

    /// Returns the sequence number for the next packet to be acknowledged for the given store path
    fn get_next_sequence_ack(&self, seq_ack_path: &SeqAckPath) -> Result<Sequence, ContextError> {
        let port_id = &seq_ack_path.0;
        let channel_id = &seq_ack_path.1;

        match self
            .ibc_store
            .next_sequence_ack
            .get(port_id)
            .and_then(|map| map.get(channel_id))
        {
            Some(sequence) => Ok(*sequence),
            None => Err(PacketError::MissingNextAckSeq {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
            }),
        }
        .map_err(ContextError::PacketError)
    }

    /// Returns the packet commitment for the given store path
    fn get_packet_commitment(
        &self,
        commitment_path: &CommitmentPath,
    ) -> Result<PacketCommitment, ContextError> {
        let port_id = &commitment_path.port_id;
        let channel_id = &commitment_path.channel_id;
        let seq = &commitment_path.sequence;

        match self
            .ibc_store
            .packet_commitment
            .get(port_id)
            .and_then(|map| map.get(channel_id))
            .and_then(|map| map.get(seq))
        {
            Some(commitment) => Ok(commitment.clone()),
            None => Err(PacketError::PacketCommitmentNotFound { sequence: *seq }),
        }
        .map_err(ContextError::PacketError)
    }

    /// Returns the packet receipt for the given store path
    fn get_packet_receipt(&self, receipt_path: &ReceiptPath) -> Result<Receipt, ContextError> {
        let port_id = &receipt_path.port_id;
        let channel_id = &receipt_path.channel_id;
        let seq = &receipt_path.sequence;

        match self
            .ibc_store
            .packet_receipt
            .get(port_id)
            .and_then(|map| map.get(channel_id))
            .and_then(|map| map.get(seq))
        {
            Some(receipt) => Ok(receipt.clone()),
            None => Err(PacketError::PacketReceiptNotFound { sequence: *seq }),
        }
        .map_err(ContextError::PacketError)
    }

    /// Returns the packet acknowledgement for the given store path
    fn get_packet_acknowledgement(
        &self,
        ack_path: &AckPath,
    ) -> Result<AcknowledgementCommitment, ContextError> {
        let port_id = &ack_path.port_id;
        let channel_id = &ack_path.channel_id;
        let seq = &ack_path.sequence;

        match self
            .ibc_store
            .packet_acknowledgement
            .get(port_id)
            .and_then(|map| map.get(channel_id))
            .and_then(|map| map.get(seq))
        {
            Some(ack) => Ok(ack.clone()),
            None => Err(PacketError::PacketAcknowledgementNotFound { sequence: *seq }),
        }
        .map_err(ContextError::PacketError)
    }

    // todo(davirian) Don't Know this correct
    /// Returns the time when the client state for the given [`ClientId`] was updated with a header
    /// for the given [`Height`]
    fn client_update_time(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Timestamp, ContextError> {
        match self
            .ibc_store
            .client_processed_times
            .get(&(client_id.clone(), *height))
        {
            Some(time) => Ok(*time),
            None => Err(ChannelError::ProcessedTimeNotFound {
                client_id: client_id.clone(),
                height: *height,
            }),
        }
        .map_err(ContextError::ChannelError)
    }

    // todo(davirian) Don't Know this correct
    /// Returns the height when the client state for the given [`ClientId`] was updated with a
    /// header for the given [`Height`]
    fn client_update_height(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Height, ContextError> {
        match self
            .ibc_store
            .client_processed_heights
            .get(&(client_id.clone(), *height))
        {
            Some(height) => Ok(*height),
            None => Err(ChannelError::ProcessedHeightNotFound {
                client_id: client_id.clone(),
                height: *height,
            }),
        }
        .map_err(ContextError::ChannelError)
    }

    /// Returns a counter on the number of channel ids have been created thus far.
    /// The value of this counter should increase only via method
    /// `ExecutionContext::increase_channel_counter`.
    fn channel_counter(&self) -> Result<u64, ContextError> {
        Ok(self.ibc_store.channel_ids_counter)
    }

    /// Returns the maximum expected time per block
    fn max_expected_time_per_block(&self) -> Duration {
        self.block_time
    }

    /// Validates the `signer` field of IBC messages, which represents the address
    /// of the user/relayer that signed the given message.
    fn validate_message_signer(&self, _signer: &Signer) -> Result<(), ContextError> {
        Ok(())
    }
}

impl ExecutionContext for SolanaIbcContext {
    /// Called upon successful client creation and update
    fn store_client_state(
        &mut self,
        client_state_path: ClientStatePath,
        client_state: Box<dyn ClientState>,
    ) -> Result<(), ContextError> {
        let client_type = client_state.client_type();
        let data = client_state.encode_vec();

        self.ibc_store
            .client_states
            .insert(client_state_path.0.clone(), data);

        self.ibc_store
            .client_types
            .insert(client_state_path.0, client_type);

        Ok(())
    }

    /// Called upon successful client creation and update
    fn store_consensus_state(
        &mut self,
        consensus_state_path: ClientConsensusStatePath,
        consensus_state: Box<dyn ConsensusState>,
    ) -> Result<(), ContextError> {
        let client_record = self
            .ibc_store
            .consensus_states
            .entry(consensus_state_path.client_id)
            .or_insert(BTreeMap::default());
        let height =
            Height::new(consensus_state_path.epoch, consensus_state_path.height).map_err(|e| {
                ClientError::Other {
                    description: format!("Construct Height failed({})", e),
                }
            })?;
        let consensus_state = consensus_state.encode_vec();
        client_record.insert(height, consensus_state);
        Ok(())
    }

    /// Called upon client creation.
    /// Increases the counter which keeps track of how many clients have been created.
    /// Should never fail.
    fn increase_client_counter(&mut self) {
        self.ibc_store.client_ids_counter += 1
    }

    /// Called upon successful client update.
    /// Implementations are expected to use this to record the specified time as the time at which
    /// this update (or header) was processed.
    fn store_update_time(
        &mut self,
        client_id: ClientId,
        height: Height,
        timestamp: Timestamp,
    ) -> Result<(), ContextError> {
        let _ = self
            .ibc_store
            .client_processed_times
            .insert((client_id, height), timestamp);
        Ok(())
    }

    /// Called upon successful client update.
    /// Implementations are expected to use this to record the specified height as the height at
    /// at which this update (or header) was processed.
    fn store_update_height(
        &mut self,
        client_id: ClientId,
        height: Height,
        host_height: Height,
    ) -> Result<(), ContextError> {
        let _ = self
            .ibc_store
            .client_processed_heights
            .insert((client_id, height), host_height);
        Ok(())
    }

    /// Stores the given connection_end at path
    fn store_connection(
        &mut self,
        connection_path: &ConnectionPath,
        connection_end: ConnectionEnd,
    ) -> Result<(), ContextError> {
        let connection_id = connection_path.0.clone();
        self.ibc_store
            .connections
            .insert(connection_id, connection_end);
        Ok(())
    }

    /// Stores the given connection_id at a path associated with the client_id.
    fn store_connection_to_client(
        &mut self,
        client_connection_path: &ClientConnectionPath,
        conn_id: ConnectionId,
    ) -> Result<(), ContextError> {
        let client_id = client_connection_path.0.clone();
        self.ibc_store.client_connections.insert(client_id, conn_id);
        Ok(())
    }

    /// Called upon connection identifier creation (Init or Try process).
    /// Increases the counter which keeps track of how many connections have been created.
    /// Should never fail.
    fn increase_connection_counter(&mut self) {
        self.ibc_store.connection_ids_counter += 1;
    }

    /// Stores the given packet commitment at the given store path
    fn store_packet_commitment(
        &mut self,
        commitment_path: &CommitmentPath,
        commitment: PacketCommitment,
    ) -> Result<(), ContextError> {
        self.ibc_store
            .packet_commitment
            .entry(commitment_path.port_id.clone())
            .or_default()
            .entry(commitment_path.channel_id.clone())
            .or_default()
            .insert(commitment_path.sequence, commitment);
        Ok(())
    }

    /// Deletes the packet commitment at the given store path
    fn delete_packet_commitment(
        &mut self,
        commitment_path: &CommitmentPath,
    ) -> Result<(), ContextError> {
        self.ibc_store
            .packet_commitment
            .get_mut(&commitment_path.port_id)
            .and_then(|map| map.get_mut(&commitment_path.channel_id))
            .and_then(|map| map.remove(&commitment_path.sequence));
        Ok(())
    }

    /// Stores the given packet receipt at the given store path
    fn store_packet_receipt(
        &mut self,
        receipt_path: &ReceiptPath,
        receipt: Receipt,
    ) -> Result<(), ContextError> {
        self.ibc_store
            .packet_receipt
            .entry(receipt_path.port_id.clone())
            .or_default()
            .entry(receipt_path.channel_id.clone())
            .or_default()
            .insert(receipt_path.sequence, receipt);
        Ok(())
    }

    /// Stores the given packet acknowledgement at the given store path
    fn store_packet_acknowledgement(
        &mut self,
        ack_path: &AckPath,
        ack_commitment: AcknowledgementCommitment,
    ) -> Result<(), ContextError> {
        let port_id = ack_path.port_id.clone();
        let channel_id = ack_path.channel_id.clone();
        let seq = ack_path.sequence;

        self.ibc_store
            .packet_acknowledgement
            .entry(port_id)
            .or_default()
            .entry(channel_id)
            .or_default()
            .insert(seq, ack_commitment);
        Ok(())
    }

    /// Deletes the packet acknowledgement at the given store path
    fn delete_packet_acknowledgement(&mut self, ack_path: &AckPath) -> Result<(), ContextError> {
        let port_id = ack_path.port_id.clone();
        let channel_id = ack_path.channel_id.clone();
        let sequence = ack_path.sequence;

        self.ibc_store
            .packet_acknowledgement
            .get_mut(&port_id)
            .and_then(|map| map.get_mut(&channel_id))
            .and_then(|map| map.remove(&sequence));
        Ok(())
    }

    /// Stores the given channel_end at a path associated with the port_id and channel_id.
    fn store_channel(
        &mut self,
        channel_end_path: &ChannelEndPath,
        channel_end: ChannelEnd,
    ) -> Result<(), ContextError> {
        let port_id = channel_end_path.0.clone();
        let channel_id = channel_end_path.1.clone();

        self.ibc_store
            .channels
            .entry(port_id)
            .or_default()
            .insert(channel_id, channel_end);
        Ok(())
    }

    /// Stores the given `nextSequenceSend` number at the given store path
    fn store_next_sequence_send(
        &mut self,
        seq_send_path: &SeqSendPath,
        seq: Sequence,
    ) -> Result<(), ContextError> {
        let port_id = seq_send_path.0.clone();
        let channel_id = seq_send_path.1.clone();

        self.ibc_store
            .next_sequence_send
            .entry(port_id)
            .or_default()
            .insert(channel_id, seq);
        Ok(())
    }

    /// Stores the given `nextSequenceRecv` number at the given store path
    fn store_next_sequence_recv(
        &mut self,
        seq_recv_path: &SeqRecvPath,
        seq: Sequence,
    ) -> Result<(), ContextError> {
        let port_id = seq_recv_path.0.clone();
        let channel_id = seq_recv_path.1.clone();

        self.ibc_store
            .next_sequence_recv
            .entry(port_id)
            .or_default()
            .insert(channel_id, seq);
        Ok(())
    }

    /// Stores the given `nextSequenceAck` number at the given store path
    fn store_next_sequence_ack(
        &mut self,
        seq_ack_path: &SeqAckPath,
        seq: Sequence,
    ) -> Result<(), ContextError> {
        let port_id = seq_ack_path.0.clone();
        let channel_id = seq_ack_path.1.clone();

        self.ibc_store
            .next_sequence_ack
            .entry(port_id)
            .or_default()
            .insert(channel_id, seq);
        Ok(())
    }

    /// Called upon channel identifier creation (Init or Try message processing).
    /// Increases the counter which keeps track of how many channels have been created.
    /// Should never fail.
    fn increase_channel_counter(&mut self) {
        self.ibc_store.channel_ids_counter += 1;
    }

    /// Emit the given IBC event
    fn emit_ibc_event(&mut self, event: IbcEvent) {
        self.events.push(event);
    }

    /// Log the given message.
    fn log_message(&mut self, message: String) {
        self.logs.push(message);
    }
}
