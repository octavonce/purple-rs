/*
  Copyright (C) 2018-2020 The Purple Core Developers.
  This file is part of the Purple Core Library.

  The Purple Core Library is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The Purple Core Library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the Purple Core Library. If not, see <http://www.gnu.org/licenses/>.
*/

use crate::bootstrap::cache::BootstrapCache;
use crate::error::NetworkErr;
use crate::interface::NetworkInterface;
use crate::packets::{RequestBlocks, SendBlocks};
use crate::protocol_flow::request_blocks::receiver_state::RequestBlocksReceiverState;
use crate::validation::receiver::Receiver;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct RequestBlocksReceiver {
    state: RequestBlocksReceiverState,
    bootstrap_cache: BootstrapCache,
}

impl RequestBlocksReceiver {
    pub fn new(bootstrap_cache: BootstrapCache) -> RequestBlocksReceiver {
        RequestBlocksReceiver {
            state: RequestBlocksReceiverState::default(),
            bootstrap_cache,
        }
    }
}

impl Receiver<RequestBlocks, SendBlocks> for RequestBlocksReceiver {
    fn receive<N: NetworkInterface>(
        &mut self,
        network: &N,
        sender: &SocketAddr,
        packet: &RequestBlocks,
    ) -> Result<SendBlocks, NetworkErr> {
        unimplemented!();
    }

    fn done(&self) -> bool {
        unimplemented!();
    }

    /// Returns true if the receiver is able to receive packets.
    fn can_receive(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        unimplemented!();
    }
}
