use ibc::{
    core::{
        ics02_client::{
            client_state::ClientState, consensus_state::ConsensusState, context::ClientReader,
        },
        ics03_connection::{
            connection::ConnectionEnd,
            context::{ConnectionKeeper, ConnectionReader},
            error::ConnectionError,
        },
        ics23_commitment::commitment::CommitmentPrefix,
        ics24_host::identifier::{ClientId, ConnectionId},
    },
    Height,
};
use ibc_proto::google::protobuf::Any;

use crate::IbcStore;

impl ConnectionReader for IbcStore {
    fn connection_end(&self, conn_id: &ConnectionId) -> Result<ConnectionEnd, ConnectionError> {
        match self.connections.get(conn_id) {
            Some(connection_end) => Ok(connection_end.clone()),
            None => Err(ConnectionError::ConnectionNotFound {
                connection_id: conn_id.clone(),
            }),
        }
    }

    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, ConnectionError> {
        // Forward method call to the Ics2 Client-specific method.
        ClientReader::client_state(self, client_id).map_err(ConnectionError::Client)
    }

    fn decode_client_state(
        &self,
        client_state: Any,
    ) -> Result<Box<dyn ClientState>, ConnectionError> {
        ClientReader::decode_client_state(self, client_state).map_err(ConnectionError::Client)
    }

    // todo
    fn host_current_height(&self) -> Result<Height, ConnectionError> {
        todo!()
    }

    // todo
    fn host_oldest_height(&self) -> Result<Height, ConnectionError> {
        todo!()
    }

    fn commitment_prefix(&self) -> CommitmentPrefix {
        CommitmentPrefix::try_from(b"Ibc".to_vec()).unwrap_or_default()
    }

    fn client_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Box<dyn ConsensusState>, ConnectionError> {
        // Forward method call to the Ics2Client-specific method.
        self.consensus_state(client_id, height)
            .map_err(|e| ConnectionError::Client(e))
    }

    fn host_consensus_state(
        &self,
        height: &Height,
    ) -> Result<Box<dyn ConsensusState>, ConnectionError> {
        ClientReader::host_consensus_state(self, height).map_err(ConnectionError::Client)
    }

    fn connection_counter(&self) -> Result<u64, ConnectionError> {
        Ok(self.connection_ids_counter)
    }

    fn validate_self_client(&self, _counterparty_client_state: Any) -> Result<(), ConnectionError> {
        Ok(())
    }
}

impl ConnectionKeeper for IbcStore {
    fn store_connection(
        &mut self,
        connection_id: ConnectionId,
        connection_end: ConnectionEnd,
    ) -> Result<(), ConnectionError> {
        self.connections.insert(connection_id, connection_end);

        Ok(())
    }

    fn store_connection_to_client(
        &mut self,
        connection_id: ConnectionId,
        client_id: ClientId,
    ) -> Result<(), ConnectionError> {
        self.client_connections.insert(client_id, connection_id);

        Ok(())
    }

    fn increase_connection_counter(&mut self) {
        self.connection_ids_counter = self.connection_ids_counter.saturating_add(1);
    }
}
