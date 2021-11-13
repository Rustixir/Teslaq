

use std::sync::Arc;

use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
use engine::teslaq_server::{Teslaq, TeslaqServer};
use engine::{
    CreateNodeRequest, RemoveNodeRequest, ProduceEventRequest,
    DemandEventRequest, DemandEventReply, Empty
};

pub mod engine {
    tonic::include_proto!("engine");
}


mod teslaqu;
use teslaqu::teslaqu::TeslaQ;



pub struct Tesla {
    tq: Arc<RwLock<TeslaQ>> 
}

#[tonic::async_trait] 
impl Teslaq for Tesla {

    async fn create_node(&self, request: Request<CreateNodeRequest>) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        let tq = &mut self.tq.write().await;
        match tq.create_vnode(req.node_name, req.total_disk_size as usize, req.buffer_capacity as usize) {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(_) => Err(Status::not_found(""))
        }
    }

    async fn remove_node(&self, request: Request<RemoveNodeRequest>) -> Result<Response<Empty> , Status>  {
        let req = request.into_inner();
        let tq = &mut self.tq.write().await;
        match tq.remove_vnode(req.node_name) {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(_) => Err(Status::not_found(""))
        }
    }

    async fn produce_event(&self, request: Request<ProduceEventRequest>) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        let tq = &mut self.tq.write().await;
        match tq.produce_event(req.node_name, req.event) {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(_) => Err(Status::not_found(""))
        }
    }

    async fn demand_event(&self, request: Request<DemandEventRequest>) -> Result<Response<DemandEventReply>, Status> {
        let req = request.into_inner();
        let tq = &mut self.tq.write().await;
        match tq.demand_events(req.node_name, req.num as usize) {
            Ok(events) => Ok(Response::new(DemandEventReply { events: events })),
            Err(_) => Err(Status::not_found(""))
        }
    }

}



async fn initialize() {
    let tesla = Tesla {
        tq: Arc::new(RwLock::new(TeslaQ::open()))
    };

    let addr = "127.0.0.1:50051".parse().unwrap();
    let _ = Server::builder()
        .add_service(TeslaqServer::new(tesla))
        .serve(addr)
        .await;
}



fn main() {
     let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
     rt.block_on(async move {
        initialize().await;
     });
}

