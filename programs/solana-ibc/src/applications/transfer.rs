pub mod impls;
use crate::SolanaIbcStoreHost;
use ibc::core::ics04_channel::channel::{Counterparty, Order};
use ibc::core::ics04_channel::error::{ChannelError, PacketError};
use ibc::core::ics04_channel::packet::{Acknowledgement, Packet};
use ibc::core::ics04_channel::Version;
use ibc::core::ics24_host::identifier::PortId;
use ibc::core::ics24_host::identifier::{ChannelId, ConnectionId};
use ibc::core::router::Module;
use ibc::core::router::ModuleExtras;
use ibc::Signer;

#[derive(Debug)]
pub struct TransferModule;

impl SolanaIbcStoreHost for TransferModule {}

impl Module for TransferModule {
    fn on_chan_open_init_validate(
        &self,
        order: Order,
        connection_hops: &[ConnectionId],
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty: &Counterparty,
        version: &Version,
    ) -> Result<Version, ChannelError> {
        todo!()
    }

    fn on_chan_open_init_execute(
        &mut self,
        order: Order,
        connection_hops: &[ConnectionId],
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty: &Counterparty,
        version: &Version,
    ) -> Result<(ModuleExtras, Version), ChannelError> {
        todo!()
    }

    fn on_chan_open_try_validate(
        &self,
        order: Order,
        connection_hops: &[ConnectionId],
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty: &Counterparty,
        counterparty_version: &Version,
    ) -> Result<Version, ChannelError> {
        todo!()
    }

    fn on_chan_open_try_execute(
        &mut self,
        order: Order,
        connection_hops: &[ConnectionId],
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty: &Counterparty,
        counterparty_version: &Version,
    ) -> Result<(ModuleExtras, Version), ChannelError> {
        todo!()
    }

    // Note: no `on_recv_packet_validate()`
    // the `onRecvPacket` callback always succeeds
    // if any error occurs, than an "error acknowledgement"
    // must be returned

    fn on_recv_packet_execute(
        &mut self,
        packet: &Packet,
        relayer: &Signer,
    ) -> (ModuleExtras, Acknowledgement) {
        todo!()
    }

    fn on_acknowledgement_packet_validate(
        &self,
        _packet: &Packet,
        _acknowledgement: &Acknowledgement,
        _relayer: &Signer,
    ) -> Result<(), PacketError> {
        todo!()
    }

    fn on_acknowledgement_packet_execute(
        &mut self,
        _packet: &Packet,
        _acknowledgement: &Acknowledgement,
        _relayer: &Signer,
    ) -> (ModuleExtras, Result<(), PacketError>) {
        todo!()
    }

    /// Note: `MsgTimeout` and `MsgTimeoutOnClose` use the same callback

    fn on_timeout_packet_validate(
        &self,
        packet: &Packet,
        relayer: &Signer,
    ) -> Result<(), PacketError> {
        todo!()
    }

    /// Note: `MsgTimeout` and `MsgTimeoutOnClose` use the same callback

    fn on_timeout_packet_execute(
        &mut self,
        packet: &Packet,
        relayer: &Signer,
    ) -> (ModuleExtras, Result<(), PacketError>) {
        todo!()
    }
}
