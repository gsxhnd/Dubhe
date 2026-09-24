//! Persistent client session state across reconnects.

use std::collections::{HashMap, VecDeque};

use bytes::Bytes;

use crate::client::Command;
use crate::inflight::InflightStore;

/// Client-side session state retained across TCP/TLS reconnects.
#[derive(Debug)]
pub(crate) struct SessionState {
    pub next_packet_id: u16,
    /// Active subscriptions keyed by topic filter → requested QoS.
    pub subscriptions: HashMap<String, u8>,
    pub inflight: InflightStore,
    /// Incoming QoS 2 messages awaiting PUBREL.
    pub qos2_incoming: HashMap<u16, (String, Bytes, bool)>,
    /// Commands received while offline (reconnect backoff).
    pub pending_commands: VecDeque<Command>,
}

impl SessionState {
    pub fn new(max_inflight: u16) -> Self {
        Self {
            next_packet_id: 1,
            subscriptions: HashMap::new(),
            inflight: InflightStore::new(max_inflight),
            qos2_incoming: HashMap::new(),
            pending_commands: VecDeque::new(),
        }
    }

    /// Clear state for a clean session / clean start.
    pub fn clear_for_clean_start(&mut self) {
        self.subscriptions.clear();
        self.inflight.clear();
        self.qos2_incoming.clear();
        self.next_packet_id = 1;
        // Keep pending_commands — user intent survives clean reconnect.
    }

    pub fn alloc_packet_id(&mut self) -> u16 {
        let id = self.next_packet_id;
        self.next_packet_id = if id == u16::MAX { 1 } else { id + 1 };
        id
    }

    pub fn remember_subscription(&mut self, filter: String, qos: u8) {
        self.subscriptions.insert(filter, qos);
    }

    pub fn forget_subscriptions(&mut self, filters: &[String]) {
        for filter in filters {
            self.subscriptions.remove(filter);
        }
    }
}
