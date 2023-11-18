use libp2p::request_response::{Event, Message, ResponseChannel};
use libp2p::request_response::cbor::Behaviour;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::core::{Context, SLSwarm};
use crate::core::behaviour::SwiftLink;

pub type CborReqResp = Behaviour<Request, Response>;

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


pub fn process_cbor_event(_context: Context, event: CborEvent, swarm: &mut SLSwarm) {
    match event {
        Event::Message { peer, message } => {
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
            match behaviour.cbor.send_response(channel, response) {
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