use libp2p::PeerId;
use libp2p::request_response::{Event, Message, ResponseChannel};
use libp2p::request_response::json::Behaviour;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::core::{Context, SLSwarm};
use crate::core::behaviour::SwiftLink;

pub type JsonReqResp = Behaviour<Request, Response>;

pub type CborEvent = Event<Request, Response>;

pub type Channel = ResponseChannel<Response>;


#[derive(Debug, Deserialize, Serialize)]
pub enum Request {
    SendFileRequest {
        filename: String,
    },
    ExchangeInfo {
        username: String,
        device_name: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    SendFileResponse {
        accept: bool,
    },
}


pub fn process_json_event(_context: Context, event: CborEvent, swarm: &mut SLSwarm) {
    match event {
        Event::Message { message, .. } => {
            match message {
                Message::Request { request, channel, .. } => {
                    let behaviour = swarm.behaviour_mut();
                    respond_to_request(request, channel, behaviour);
                }
                Message::Response { response, .. } => {
                    process_response(response);
                }
            }
        }
        Event::OutboundFailure { peer, request_id, error } => {
            warn!("outbound failure: peer: {}, request_id: {}, error: {}",peer,request_id,error);
        }
        Event::InboundFailure { error, peer, request_id } => {
            warn!("inbound failure: peer: {}, request_id: {}, error: {}",peer,request_id,error);
        }
        Event::ResponseSent { request_id, peer } => {
            info!("response sent: peer: {}, request_id: {}",peer,request_id);
        }
    };
}

pub fn process_response(resp: Response) {
    match resp {
        Response::SendFileResponse { accept } => {
            info!("received SendFileResponse: accept: {}",accept);
        }
    };
}

pub fn respond_to_request(req: Request, channel: Channel, behaviour: &mut SwiftLink) {
    match req {
        Request::SendFileRequest { filename } => {
            let response = Response::SendFileResponse { accept: true };
            match behaviour.json.send_response(channel, response) {
                Ok(_) => {
                    info!("successfully sent response for SendFileRequest: {}",filename);
                }
                Err(_) => {
                    info!("failed to send response for SendFileRequest: {}",filename);
                }
            }
        }
        Request::ExchangeInfo { username, device_name } => {
            info!("received ExchangeInfo request from {} for {}",username,device_name);
        }
    };
}

pub fn exchange_info(swarm: &mut SLSwarm, peer_id: &PeerId) {
    let behaviour = swarm.behaviour_mut();
    behaviour.json.send_request(
        &peer_id,
        Request::ExchangeInfo {
            username: "test".to_string(),
            device_name: "test".to_string(),
        },
    );
    info!("sent a peer info request to {}", peer_id);
}
