use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    Get,
    Put,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage {
    pub src: String,
    pub dst: String,
    pub leader: String,
    pub mid: String,
    pub value: Option<String>,
    pub request_type: RequestType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    Get,
    Put,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub src: String,
    pub dst: String,
    pub leader: String,
    pub mid: String,
    pub value: Option<String>,
    pub response_type: ResponseType,
}
