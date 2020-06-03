use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "put")]
    Put,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage {
    pub src: String,
    pub dst: String,
    pub leader: String,
    pub mid: String,
    pub value: Option<String>,
    #[serde(rename = "type")]
    pub request_type: RequestType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "redirect")]
    Redirect,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub src: String,
    pub dst: String,
    pub leader: String,
    pub mid: String,
    pub value: Option<String>,
    #[serde(rename = "type")]
    pub response_type: ResponseType,
}
