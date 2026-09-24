//! Outbound QoS 1/2 inflight tracking and pending publish queue.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use bytes::Bytes;

/// Publish waiting for an inflight slot.
#[derive(Debug, Clone)]
pub(crate) struct PendingPublish {
    pub topic: String,
    pub payload: Bytes,
    pub qos: u8,
    pub retain: bool,
}

/// QoS handshake stage for an outbound publish.
///
/// Variant names mirror the broker packet we are waiting for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InflightStage {
    /// QoS 1 — waiting for PUBACK.
    Ack,
    /// QoS 2 — waiting for PUBREC.
    Rec,
    /// QoS 2 — PUBREL sent, waiting for PUBCOMP.
    Comp,
}

/// One outbound QoS 1/2 message in flight.
#[derive(Debug, Clone)]
pub(crate) struct InflightPublish {
    pub topic: String,
    pub payload: Bytes,
    pub qos: u8,
    pub retain: bool,
    pub stage: InflightStage,
    pub sent_at: Instant,
}

/// Bounded inflight window plus overflow queue.
#[derive(Debug)]
pub(crate) struct InflightStore {
    max: usize,
    map: HashMap<u16, InflightPublish>,
    pending: VecDeque<PendingPublish>,
}

impl InflightStore {
    pub fn new(max_inflight: u16) -> Self {
        Self {
            max: usize::from(max_inflight.max(1)),
            map: HashMap::new(),
            pending: VecDeque::new(),
        }
    }

    pub fn has_capacity(&self) -> bool {
        self.map.len() < self.max
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.pending.clear();
    }

    pub fn insert(&mut self, packet_id: u16, entry: InflightPublish) {
        self.map.insert(packet_id, entry);
    }

    pub fn get_mut(&mut self, packet_id: u16) -> Option<&mut InflightPublish> {
        self.map.get_mut(&packet_id)
    }

    pub fn remove(&mut self, packet_id: u16) -> Option<InflightPublish> {
        self.map.remove(&packet_id)
    }

    pub fn push_pending(&mut self, publish: PendingPublish) {
        self.pending.push_back(publish);
    }

    pub fn pop_pending(&mut self) -> Option<PendingPublish> {
        if self.has_capacity() {
            self.pending.pop_front()
        } else {
            None
        }
    }

    /// Packet IDs whose last send exceeded `timeout`.
    pub fn due_for_retry(&self, timeout: Duration) -> Vec<u16> {
        let now = Instant::now();
        self.map
            .iter()
            .filter(|(_, entry)| now.duration_since(entry.sent_at) >= timeout)
            .map(|(&id, _)| id)
            .collect()
    }

    /// Snapshot of inflight entries for session resume / reconnect republish.
    pub fn snapshot(&self) -> Vec<(u16, InflightPublish)> {
        self.map
            .iter()
            .map(|(&id, entry)| (id, entry.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inflight_store_should_queue_when_at_capacity() {
        let mut store = InflightStore::new(1);
        store.insert(
            1,
            InflightPublish {
                topic: "a".into(),
                payload: Bytes::from_static(b"1"),
                qos: 1,
                retain: false,
                stage: InflightStage::Ack,
                sent_at: Instant::now(),
            },
        );
        assert!(!store.has_capacity());
        store.push_pending(PendingPublish {
            topic: "b".into(),
            payload: Bytes::from_static(b"2"),
            qos: 1,
            retain: false,
        });
        assert!(store.pop_pending().is_none());
        store.remove(1);
        let pending = store.pop_pending().expect("pending publish");
        assert_eq!(pending.topic, "b");
    }

    #[test]
    fn inflight_store_should_report_due_for_retry() {
        let mut store = InflightStore::new(10);
        store.insert(
            7,
            InflightPublish {
                topic: "t".into(),
                payload: Bytes::new(),
                qos: 1,
                retain: false,
                stage: InflightStage::Ack,
                sent_at: Instant::now() - Duration::from_secs(10),
            },
        );
        let due = store.due_for_retry(Duration::from_secs(1));
        assert_eq!(due, vec![7]);
    }
}
