/*
  Copyright 2018 The Purple Library Authors
  This file is part of the Purple Library.

  The Purple Library is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The Purple Library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the Purple Library. If not, see <http://www.gnu.org/licenses/>.
*/

use crate::error::NetworkErr;
use crate::peer::Peer;
use crate::packet::Packet;
use crate::node_id::NodeId;
use crate::chain::{EasyChainRef, HardChainRef, HardBlock, EasyBlock};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::Sender;

/// Generic network layer interface.
pub trait NetworkInterface {
    /// Attempts to connect to the peer with the given ip.
    fn connect(&mut self, address: &SocketAddr) -> Result<(), NetworkErr>;

    /// Attempts to connect to a previously encountered peer 
    fn connect_to_known(&self, peer: &NodeId) -> Result<(), NetworkErr>;

    /// Returns true if the network has the given address in its peer list.
    fn is_connected_to(&self, address: &SocketAddr) -> bool;

    /// Disconnects from the peer with the given `NodeId`.
    fn disconnect(&mut self, peer: &NodeId) -> Result<(), NetworkErr>;

    /// Disconnects from the peer with the given ip address.
    fn disconnect_from_ip(&mut self, ip: &SocketAddr) -> Result<(), NetworkErr>;

    /// Sends a packet to a specific peer.
    fn send_to_peer(&self, peer: &SocketAddr, packet: &[u8]) -> Result<(), NetworkErr>;

    /// Sends a packet to all peers.
    fn send_to_all(&self, packet: &[u8]) -> Result<(), NetworkErr>;

    /// Attempts to send a packet to the specific peer. This
    /// function will also sign the packet if it does not yet
    /// have a signature and it will also serialize it to binary.
    fn send_unsigned<P: Packet>(&self, peer: &SocketAddr, packet: &mut P) -> Result<(), NetworkErr>;

    /// Sends a raw packet to a specific peer. This 
    /// means that the packet will be un-encrypted.
    fn send_raw(&self, peer: &SocketAddr, packet: &[u8]) -> Result<(), NetworkErr>;

    /// This behaves similarly to `send_unsigned()` but it sends a raw packet.
    fn send_raw_unsigned<P: Packet>(&self, peer: &SocketAddr, packet: &mut P) -> Result<(), NetworkErr>;

    /// Callback that processes each packet that is received from any peer.
    fn process_packet(&mut self, peer: &SocketAddr, packet: &[u8]) -> Result<(), NetworkErr>;

    /// Bans the peer with the node id
    fn ban_peer(&self, peer: &NodeId) -> Result<(), NetworkErr>;

    /// Bans any further connections from the given ip. 
    fn ban_ip(&self, peer: &SocketAddr) -> Result<(), NetworkErr>;

    /// Attempts to retrieve a reference to 
    /// the peer entry of the given `NodeId`.
    fn fetch_peer(&self, peer: &SocketAddr) -> Result<&Peer, NetworkErr>;

    /// Attempts to retrieve a mutable reference to 
    /// the peer entry of the given ip.
    fn fetch_peer_mut(&mut self, peer: &SocketAddr) -> Result<&mut Peer, NetworkErr>;

    /// Returns a reference to our node id.
    fn our_node_id(&self) -> &NodeId;

    /// Returns an iterator on the listed peers
    fn peers<'a>(&'a self) -> Box<dyn Iterator<Item = (&SocketAddr, &Peer)> + 'a>;

    /// Returns a reference to the `EasyChain`.
    fn easy_chain_ref(&self) -> EasyChainRef;

    /// Returns a reference to the `HardChain`.
    fn hard_chain_ref(&self) -> HardChainRef;

    /// Returns a reference to a `EasyChain` mpsc sender.
    /// Use this to buffer blocks that are to be appended
    /// to the chain.
    fn easy_chain_sender(&self) -> &Sender<Arc<EasyBlock>>;

    /// Returns a reference to a `HardChain` mpsc sender.
    /// Use this to buffer blocks that are to be appended
    /// to the chain.
    fn hard_chain_sender(&self) -> &Sender<Arc<HardBlock>>;
}
