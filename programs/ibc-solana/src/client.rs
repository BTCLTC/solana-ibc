use std::collections::BTreeMap;

use ibc::{
    clients::ics07_tendermint::{
        client_state::ClientState as Ics07ClientState,
        consensus_state::ConsensusState as Ics07ConsensusState,
    },
    core::{
        ics02_client::{
            client_state::ClientState,
            client_type::ClientType,
            consensus_state::ConsensusState,
            context::{ClientKeeper, ClientReader},
            error::ClientError,
        },
        ics24_host::identifier::ClientId,
    },
    timestamp::Timestamp,
    Height,
};
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};

use crate::IbcStore;

pub const TENDERMINT_CLIENT_TYPE: &'static str = "07-tendermint";

impl ClientReader for IbcStore {
    fn client_type(&self, client_id: &ClientId) -> Result<ClientType, ClientError> {
        match self.client_types.get(client_id) {
            Some(client_types) => Ok(client_types.clone()),
            None => Err(ClientError::ClientNotFound {
                client_id: client_id.clone(),
            }),
        }
    }

    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, ClientError> {
        if let Some(client_state) = self.client_state.get(&client_id) {
            return match self.client_type(client_id)?.as_str() {
                TENDERMINT_CLIENT_TYPE => {
                    let result: Ics07ClientState = Protobuf::<Any>::decode_vec(&client_state)
                        .map_err(|e| ClientError::Other {
                            description: format!("Decode Ics07ClientState failed: {:?}", e)
                                .to_string(),
                        })?;
                    Ok(Box::new(result))
                }
                unimplemented => Err(ClientError::UnknownClientStateType {
                    client_state_type: unimplemented.to_string(),
                }),
            };
        } else {
            Err(ClientError::ClientNotFound {
                client_id: client_id.clone(),
            })
        }
    }

    fn decode_client_state(&self, client_state: Any) -> Result<Box<dyn ClientState>, ClientError> {
        if let Ok(client_state) = Ics07ClientState::try_from(client_state.clone()) {
            return Ok(client_state.into_box());
        } else {
            Err(ClientError::UnknownClientStateType {
                client_state_type: client_state.type_url,
            })
        }
    }

    fn consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Box<dyn ConsensusState>, ClientError> {
        if let Some(height_with_consensus_state) = self.consensus_states.get(&client_id) {
            if let Some(consensus_state) = height_with_consensus_state.get(&height) {
                return match self.client_type(client_id)?.as_str() {
                    TENDERMINT_CLIENT_TYPE => {
                        let result: Ics07ConsensusState =
                            Protobuf::<Any>::decode_vec(&consensus_state)
                                .map_err(|_| ClientError::ImplementationSpecific)?;
                        Ok(Box::new(result))
                    }
                    unimplemented => Err(ClientError::UnknownClientStateType {
                        client_state_type: unimplemented.to_string(),
                    }),
                };
            } else {
                Err(ClientError::ConsensusStateNotFound {
                    client_id: client_id.clone(),
                    height: height.clone(),
                })
            }
        } else {
            Err(ClientError::ConsensusStateNotFound {
                client_id: client_id.clone(),
                height: height.clone(),
            })
        }
    }

    // todo
    fn next_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, ClientError> {
        let client_record =
            self.consensus_states
                .get(client_id)
                .ok_or_else(|| ClientError::ClientNotFound {
                    client_id: client_id.clone(),
                })?;

        // Get the consensus state heights and sort them in ascending order.
        let mut heights: Vec<Height> = client_record.keys().cloned().collect();
        heights.sort();

        // Search for next state.
        for h in heights {
            if h > *height {
                // unwrap should never happen, as the consensus state for h must exist
                let result = client_record.get(&h).unwrap().clone();
                return match self.client_type(client_id)?.as_str() {
                    TENDERMINT_CLIENT_TYPE => {
                        let result: Ics07ConsensusState = Protobuf::<Any>::decode_vec(&result)
                            .map_err(|e| ClientError::Other {
                                description: format!("Decode Ics07ConsensusState failed: {:?}", e)
                                    .to_string(),
                            })?;
                        Ok(Some(Box::new(result)))
                    }
                    unimplemented => Err(ClientError::UnknownClientStateType {
                        client_state_type: unimplemented.to_string(),
                    }),
                };
            }
        }
        Ok(None)
    }

    // todo
    fn prev_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Box<dyn ConsensusState>>, ClientError> {
        let client_record =
            self.consensus_states
                .get(client_id)
                .ok_or_else(|| ClientError::ClientNotFound {
                    client_id: client_id.clone(),
                })?;

        // Get the consensus state heights and sort them in ascending order.
        let mut heights: Vec<Height> = client_record.keys().cloned().collect();
        heights.sort();

        // Search for next state.
        for h in heights {
            if h < *height {
                // unwrap should never happen, as the consensus state for h must exist
                let result = client_record.get(&h).unwrap().clone();
                return match self.client_type(client_id)?.as_str() {
                    TENDERMINT_CLIENT_TYPE => {
                        let result: Ics07ConsensusState = Protobuf::<Any>::decode_vec(&result)
                            .map_err(|e| ClientError::Other {
                                description: format!("Decode Ics07ConsensusState failed: {:?}", e)
                                    .to_string(),
                            })?;
                        Ok(Some(Box::new(result)))
                    }
                    unimplemented => Err(ClientError::UnknownClientStateType {
                        client_state_type: unimplemented.to_string(),
                    }),
                };
            }
        }
        Ok(None)
    }

    // todo
    fn host_height(&self) -> Result<Height, ClientError> {
        todo!()
    }

    // todo
    fn host_timestamp(&self) -> Result<Timestamp, ClientError> {
        todo!()
    }

    fn host_consensus_state(
        &self,
        _height: &Height,
    ) -> Result<Box<dyn ConsensusState>, ClientError> {
        Err(ClientError::ImplementationSpecific)
    }

    fn pending_host_consensus_state(&self) -> Result<Box<dyn ConsensusState>, ClientError> {
        Err(ClientError::ImplementationSpecific)
    }

    fn client_counter(&self) -> Result<u64, ClientError> {
        Ok(self.client_ids_counter)
    }
}

impl ClientKeeper for IbcStore {
    fn store_client_type(
        &mut self,
        client_id: ClientId,
        client_type: ClientType,
    ) -> Result<(), ClientError> {
        self.client_types.insert(client_id, client_type);
        Ok(())
    }

    fn store_client_state(
        &mut self,
        client_id: ClientId,
        client_state: Box<dyn ClientState>,
    ) -> Result<(), ClientError> {
        let data = client_state
            .encode_vec()
            .map_err(|_| ClientError::ImplementationSpecific)?;
        self.client_state.insert(client_id, data);

        Ok(())
    }

    fn store_consensus_state(
        &mut self,
        client_id: ClientId,
        height: Height,
        consensus_state: Box<dyn ConsensusState>,
    ) -> Result<(), ClientError> {
        let consensus_state = consensus_state
            .encode_vec()
            .map_err(|_| ClientError::ImplementationSpecific)?;

        if let Some(height_with_consensus_state) = self.consensus_states.get_mut(&client_id) {
            height_with_consensus_state.insert(height, consensus_state);
        } else {
            let mut new_height_with_consensus_state = BTreeMap::new();
            new_height_with_consensus_state.insert(height, consensus_state);
            self.consensus_states
                .insert(client_id, new_height_with_consensus_state);
        };

        Ok(())
    }

    fn increase_client_counter(&mut self) {
        self.client_ids_counter = self.client_ids_counter.saturating_add(1);
    }

    fn store_update_time(
        &mut self,
        client_id: ClientId,
        height: Height,
        timestamp: Timestamp,
    ) -> Result<(), ClientError> {
        self.client_processed_times
            .insert((client_id, height), timestamp);

        Ok(())
    }

    fn store_update_height(
        &mut self,
        client_id: ClientId,
        height: Height,
        host_height: Height,
    ) -> Result<(), ClientError> {
        self.client_processed_heights
            .insert((client_id, height), host_height.into());

        Ok(())
    }
}
