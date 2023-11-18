use libp2p::request_response::cbor::Behaviour;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Action {
    SendFile {
        filename: String,
    },
}


#[derive(Debug, Deserialize, Serialize)]
struct Request {
    action: Action,
}


#[derive(Debug, Deserialize, Serialize)]
struct Response {
    action: Action,
}

pub type CborReqResp = Behaviour<Request, Response>;