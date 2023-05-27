use crate::{raft::Store, raft_service::raft_service_client::RaftServiceClient};

// use raft::raw_node::RawNode;
// use raft::storage::MemStorage;
use std::collections::HashMap;
use tonic::transport::channel::Channel;

type Error = Box<dyn std::error::Error>;
pub struct Peer {
    addr: String,
    client: RaftServiceClient<Channel>,
}

impl Peer {
    pub async fn new(addr: &str) -> Result<Peer, Error> {
        let client = RaftServiceClient::connect("dst").await?;
        let addr = addr.to_string();
        Ok(Peer { addr, client })
    }
    pub fn test(self) {
        print!("{}", self.addr);
        print!("{:?}", self.client);
    }
}

pub struct RaftNode<S: Store> {
    // inner: RawNode<MemStorage>,
    pub peers: HashMap<u64, Option<Peer>>,
    _store: S,
}

impl<S: Store> RaftNode<S> {
    pub fn new_leader() {}
    pub fn new_follower() {}
    pub fn is_leader(self) -> bool {
        todo!()
    }
    // return current node id
    pub fn id() -> u64 {
        todo!()
    }
    // cluster add peer
    pub async fn add_peer() {}
    // return leader id
    pub fn leader() -> u64 {
        todo!()
    }
    // return all node address of cluster
    pub fn peer_addrs() -> HashMap<u64, String> {
        todo!()
    }
}
