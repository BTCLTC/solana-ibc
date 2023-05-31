use crate::applications::transfer::TransferModule;
use crate::SolanaIbcStoreHost;
use ibc::applications::transfer::context::{
    TokenTransferExecutionContext, TokenTransferValidationContext,
};
use ibc::applications::transfer::error::TokenTransferError;
use ibc::applications::transfer::PrefixedCoin;
use ibc::core::events::IbcEvent;
use ibc::core::ics02_client::client_state::ClientState;
use ibc::core::ics02_client::consensus_state::ConsensusState;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics04_channel::channel::ChannelEnd;
use ibc::core::ics04_channel::commitment::PacketCommitment;
use ibc::core::ics04_channel::context::{SendPacketExecutionContext, SendPacketValidationContext};
use ibc::core::ics04_channel::error::PacketError;
use ibc::core::ics04_channel::packet::Sequence;
use ibc::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use ibc::core::ics24_host::path::{
    ChannelEndPath, ClientConsensusStatePath, CommitmentPath, SeqSendPath,
};
use ibc::core::ContextError;
use ibc::core::ValidationContext;
use ibc::Signer;

impl TokenTransferValidationContext for TransferModule {
    type AccountId = Signer;

    fn get_port(&self) -> Result<PortId, TokenTransferError> {
        Ok(PortId::transfer())
    }

    fn get_escrow_account(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<Self::AccountId, TokenTransferError> {
        // let addr = cosmos_adr028_escrow_address(port_id, channel_id);
        // Ok(bech32::encode("cosmos", addr).into())
        todo!()
    }

    fn can_send_coins(&self) -> Result<(), TokenTransferError> {
        Ok(())
    }

    fn can_receive_coins(&self) -> Result<(), TokenTransferError> {
        Ok(())
    }

    fn send_coins_validate(
        &self,
        _from_account: &Self::AccountId,
        _to_account: &Self::AccountId,
        _coin: &PrefixedCoin,
    ) -> Result<(), TokenTransferError> {
        Ok(())
    }

    fn mint_coins_validate(
        &self,
        _account: &Self::AccountId,
        _coin: &PrefixedCoin,
    ) -> Result<(), TokenTransferError> {
        Ok(())
    }

    fn burn_coins_validate(
        &self,
        _account: &Self::AccountId,
        _coin: &PrefixedCoin,
    ) -> Result<(), TokenTransferError> {
        Ok(())
    }
}

impl TokenTransferExecutionContext for TransferModule {
    fn send_coins_execute(
        &mut self,
        _from_account: &Self::AccountId,
        _to_account: &Self::AccountId,
        _coin: &PrefixedCoin,
    ) -> Result<(), TokenTransferError> {
        Ok(())
    }

    fn mint_coins_execute(
        &mut self,
        _account: &Self::AccountId,
        _coin: &PrefixedCoin,
    ) -> Result<(), TokenTransferError> {
        Ok(())
    }

    fn burn_coins_execute(
        &mut self,
        _account: &Self::AccountId,
        _coin: &PrefixedCoin,
    ) -> Result<(), TokenTransferError> {
        Ok(())
    }
}

impl SendPacketValidationContext for TransferModule {
    /// Returns the ChannelEnd for the given `port_id` and `chan_id`.
    fn channel_end(&self, channel_end_path: &ChannelEndPath) -> Result<ChannelEnd, ContextError> {
        ValidationContext::channel_end(&Self::get_solana_ibc_store(), channel_end_path)
    }

    /// Returns the ConnectionState for the given identifier `connection_id`.
    fn connection_end(&self, connection_id: &ConnectionId) -> Result<ConnectionEnd, ContextError> {
        ValidationContext::connection_end(&Self::get_solana_ibc_store(), connection_id)
    }

    /// Returns the ClientState for the given identifier `client_id`. Necessary dependency towards
    /// proof verification.
    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, ContextError> {
        ValidationContext::client_state(&Self::get_solana_ibc_store(), client_id)
    }

    fn client_consensus_state(
        &self,
        client_cons_state_path: &ClientConsensusStatePath,
    ) -> Result<Box<dyn ConsensusState>, ContextError> {
        ValidationContext::consensus_state(&Self::get_solana_ibc_store(), client_cons_state_path)
    }

    fn get_next_sequence_send(
        &self,
        seq_send_path: &SeqSendPath,
    ) -> Result<Sequence, ContextError> {
        ValidationContext::get_next_sequence_send(&Self::get_solana_ibc_store(), seq_send_path)
    }
}

impl SendPacketExecutionContext for TransferModule {
    fn store_next_sequence_send(
        &mut self,
        seq_send_path: &SeqSendPath,
        seq: Sequence,
    ) -> Result<(), ContextError> {
        let mut store = Self::get_solana_ibc_store();
        let port_id = seq_send_path.0.clone();
        let channel_id = seq_send_path.1.clone();

        store
            .next_sequence_send
            .entry(port_id)
            .or_default()
            .insert(channel_id, seq);
        Self::set_solana_ibc_store(&store);

        Ok(())
    }

    fn store_packet_commitment(
        &mut self,
        commitment_path: &CommitmentPath,
        commitment: PacketCommitment,
    ) -> Result<(), ContextError> {
        let mut store = Self::get_solana_ibc_store();

        store
            .packet_commitment
            .entry(commitment_path.port_id.clone())
            .or_default()
            .entry(commitment_path.channel_id.clone())
            .or_default()
            .insert(commitment_path.sequence, commitment);

        Self::set_solana_ibc_store(&store);

        Ok(())
    }

    /// Ibc events
    fn emit_ibc_event(&mut self, event: IbcEvent) {
        let mut store = Self::get_solana_ibc_store();

        store.events.push(event);

        Self::set_solana_ibc_store(&store);
    }

    /// Logging facility
    fn log_message(&mut self, message: String) {
        let mut store = Self::get_solana_ibc_store();

        store.logs.push(message);

        Self::set_solana_ibc_store(&store);
    }
}
