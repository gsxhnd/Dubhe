use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};
use tracing::info;

use crate::config::RaftConfig;
use crate::raft_service::raft_service_server::{RaftService, RaftServiceServer};
use crate::raft_service::{IdRequestReponse, IdRequestRequest};

pub struct RaftServer {
    // snd: mpsc::Sender<Message>,
    cfg: RaftConfig,
}

#[derive(Debug, Default)]
pub struct RaftServiceGrpcServer {}

#[tonic::async_trait]
impl RaftService for RaftServiceGrpcServer {
    async fn request_id(
        &self,
        request: Request<IdRequestRequest>,
    ) -> Result<Response<IdRequestReponse>, Status> {
        info!("grpc server request, id: {}", request.into_inner().id);
        Ok(Response::new(IdRequestReponse { data: vec![] }))
    }
}

impl RaftServer {
    pub fn new(cfg: RaftConfig) -> Self {
        RaftServer { cfg }
    }
    pub async fn run(self) {
        let svc = RaftServiceGrpcServer::default();
        let addr: SocketAddr = self
            .cfg
            .listener_addr
            .parse()
            .expect("address is not valid");

        info!("raft service started in {}", self.cfg.listener_addr);
        Server::builder()
            .add_service(RaftServiceServer::new(svc))
            .serve(addr)
            .await
            .expect("");
    }
}

#[cfg(test)]
#[tokio::test]
async fn client_test() {
    use crate::raft_service::raft_service_client::RaftServiceClient;
    use crate::raft_service::IdRequestRequest;

    let mut client = RaftServiceClient::connect("http://127.0.0.1:8888")
        .await
        .unwrap();
    let resp = client
        .request_id(Request::new(IdRequestRequest { id: 1 }))
        .await
        .expect("msg");
    println!("responese: {:?}", resp);
}
