use ibc::{
    core::{
        ics02_client::{
            client_state::ClientState, consensus_state::ConsensusState, context::ClientReader,
        },
        ics03_connection::{
            connection::ConnectionEnd, context::ConnectionReader, error::ConnectionError,
        },
        ics04_channel::{
            channel::ChannelEnd,
            commitment::{
                AcknowledgementCommitment as IbcAcknowledgementCommitment,
                PacketCommitment as IbcPacketCommitment,
            },
            context::{ChannelKeeper, ChannelReader},
            error::{ChannelError, PacketError},
            packet::{Receipt, Sequence},
        },
        ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
    },
    timestamp::Timestamp,
    Height,
};
use std::time::Duration;

use crate::IbcStore;

impl ChannelReader for IbcStore {
    fn channel_end(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<ChannelEnd, ChannelError> {
        match self
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
    }

    fn connection_end(&self, connection_id: &ConnectionId) -> Result<ConnectionEnd, ChannelError> {
        ConnectionReader::connection_end(self, connection_id).map_err(ChannelError::Connection)
    }

    /// Returns the `ChannelsConnection` for the given identifier `conn_id`.
    fn connection_channels(
        &self,
        conn_id: &ConnectionId,
    ) -> Result<Vec<(PortId, ChannelId)>, ChannelError> {
        match self.connection_channels.get(conn_id) {
            Some(pcid) => Ok(pcid.clone()),
            None => Err(ChannelError::MissingChannel),
        }
    }

    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, ChannelError> {
        ClientReader::client_state(self, client_id)
            .map_err(|e| ChannelError::Connection(ConnectionError::Client(e)))
    }

    fn client_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Box<dyn ConsensusState>, ChannelError> {
        ClientReader::consensus_state(self, client_id, height)
            .map_err(|e| ChannelError::Connection(ConnectionError::Client(e)))
    }

    fn get_next_sequence_send(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<Sequence, PacketError> {
        match self
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
    }

    fn get_next_sequence_recv(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<Sequence, PacketError> {
        match self
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
    }

    fn get_next_sequence_ack(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<Sequence, PacketError> {
        match self
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
    }

    /// Returns the `PacketCommitment` for the given identifier `(PortId, ChannelId, Sequence)`.
    fn get_packet_commitment(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<IbcPacketCommitment, PacketError> {
        match self
            .packet_commitment
            .get(port_id)
            .and_then(|map| map.get(channel_id))
            .and_then(|map| map.get(seq))
        {
            Some(commitment) => Ok(commitment.clone()),
            None => Err(PacketError::PacketCommitmentNotFound { sequence: *seq }),
        }
    }

    fn get_packet_receipt(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<Receipt, PacketError> {
        match self
            .packet_receipt
            .get(port_id)
            .and_then(|map| map.get(channel_id))
            .and_then(|map| map.get(seq))
        {
            Some(receipt) => Ok(receipt.clone()),
            None => Err(PacketError::PacketReceiptNotFound { sequence: *seq }),
        }
    }

    /// Returns the `Acknowledgements` for the given identifier `(PortId, ChannelId, Sequence)`.
    fn get_packet_acknowledgement(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<IbcAcknowledgementCommitment, PacketError> {
        match self
            .packet_acknowledgement
            .get(port_id)
            .and_then(|map| map.get(channel_id))
            .and_then(|map| map.get(seq))
        {
            Some(ack) => Ok(ack.clone()),
            None => Err(PacketError::PacketAcknowledgementNotFound { sequence: *seq }),
        }
    }

    /// A hashing function for packet commitments
    fn hash(&self, value: &[u8]) -> Vec<u8> {
        use sha2::Digest;
        sha2::Sha256::digest(value).to_vec()
    }

    /// Returns the current height of the local chain.
    fn host_height(&self) -> Result<Height, ChannelError> {
        todo!()
    }

    /// Returns the `AnyConsensusState` for the given identifier `height`.
    fn host_consensus_state(
        &self,
        height: &Height,
    ) -> Result<Box<dyn ConsensusState>, ChannelError> {
        ConnectionReader::host_consensus_state(self, height).map_err(ChannelError::Connection)
    }

    fn pending_host_consensus_state(&self) -> Result<Box<dyn ConsensusState>, ChannelError> {
        ClientReader::pending_host_consensus_state(self)
            .map_err(|e| ChannelError::Connection(ConnectionError::Client(e)))
    }

    /// Returns the `ClientProcessedTimes` for the given identifier `client_id` & `height`.
    fn client_update_time(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Timestamp, ChannelError> {
        match self
            .client_processed_times
            .get(&(client_id.clone(), *height))
        {
            Some(time) => Ok(*time),
            None => Err(ChannelError::ProcessedTimeNotFound {
                client_id: client_id.clone(),
                height: *height,
            }),
        }
    }

    fn client_update_height(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Height, ChannelError> {
        match self
            .client_processed_heights
            .get(&(client_id.clone(), *height))
        {
            Some(height) => Ok(*height),
            None => Err(ChannelError::ProcessedHeightNotFound {
                client_id: client_id.clone(),
                height: *height,
            }),
        }
    }

    /// Returns a counter on the number of channel ids have been created thus far.
    /// The value of this counter should increase only via method
    /// `ChannelKeeper::increase_channel_counter`.
    fn channel_counter(&self) -> Result<u64, ChannelError> {
        Ok(self.channel_ids_counter)
    }

    fn max_expected_time_per_block(&self) -> Duration {
        todo!()
    }
}
impl ChannelKeeper for IbcStore {
    fn store_packet_commitment(
        &mut self,
        port_id: PortId,
        channel_id: ChannelId,
        seq: Sequence,
        commitment: IbcPacketCommitment,
    ) -> Result<(), PacketError> {
        self.packet_commitment
            .entry(port_id)
            .or_default()
            .entry(channel_id)
            .or_default()
            .insert(seq, commitment);
        Ok(())
    }

    fn store_packet_acknowledgement(
        &mut self,
        port_id: PortId,
        channel_id: ChannelId,
        seq: Sequence,
        ack_commitment: IbcAcknowledgementCommitment,
    ) -> Result<(), PacketError> {
        self.packet_acknowledgement
            .entry(port_id)
            .or_default()
            .entry(channel_id)
            .or_default()
            .insert(seq, ack_commitment);
        Ok(())
    }

    fn delete_packet_acknowledgement(
        &mut self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<(), PacketError> {
        self.packet_acknowledgement
            .get_mut(port_id)
            .and_then(|map| map.get_mut(channel_id))
            .and_then(|map| map.remove(seq));
        Ok(())
    }

    fn store_connection_channels(
        &mut self,
        cid: ConnectionId,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<(), ChannelError> {
        self.connection_channels
            .entry(cid)
            .or_insert_with(Vec::new)
            .push((port_id, channel_id));
        Ok(())
    }

    fn store_channel(
        &mut self,
        port_id: PortId,
        channel_id: ChannelId,
        channel_end: ChannelEnd,
    ) -> Result<(), ChannelError> {
        self.channels
            .entry(port_id)
            .or_default()
            .insert(channel_id, channel_end);
        Ok(())
    }

    fn store_next_sequence_send(
        &mut self,
        port_id: PortId,
        channel_id: ChannelId,
        seq: Sequence,
    ) -> Result<(), PacketError> {
        self.next_sequence_send
            .entry(port_id)
            .or_default()
            .insert(channel_id, seq);
        Ok(())
    }

    fn store_next_sequence_recv(
        &mut self,
        port_id: PortId,
        channel_id: ChannelId,
        seq: Sequence,
    ) -> Result<(), PacketError> {
        self.next_sequence_recv
            .entry(port_id)
            .or_default()
            .insert(channel_id, seq);
        Ok(())
    }

    fn store_next_sequence_ack(
        &mut self,
        port_id: PortId,
        channel_id: ChannelId,
        seq: Sequence,
    ) -> Result<(), PacketError> {
        self.next_sequence_ack
            .entry(port_id)
            .or_default()
            .insert(channel_id, seq);
        Ok(())
    }

    fn increase_channel_counter(&mut self) {
        self.channel_ids_counter = self.channel_ids_counter.saturating_add(1);
    }

    fn delete_packet_commitment(
        &mut self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<(), PacketError> {
        self.packet_commitment
            .get_mut(port_id)
            .and_then(|map| map.get_mut(channel_id))
            .and_then(|map| map.remove(seq));
        Ok(())
    }

    fn store_packet_receipt(
        &mut self,
        port_id: PortId,
        channel_id: ChannelId,
        seq: Sequence,
        receipt: Receipt,
    ) -> Result<(), PacketError> {
        self.packet_receipt
            .entry(port_id)
            .or_default()
            .entry(channel_id)
            .or_default()
            .insert(seq, receipt);
        Ok(())
    }
}
