use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message<W> {
    pub src: String,
    pub dest: String,
    pub body: Body<W>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Body<W> {
    Error(Error),
    Init(Init),
    Workload(W),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "error")]
#[serde(tag = "type")]
pub struct Error {
    pub in_reply_to: u64,
    pub code: u64,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Init {
    #[serde(rename = "init")]
    Init(InitRequest),

    #[serde(rename = "init_ok")]
    InitOk(InitResponse),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitRequest {
    pub msg_id: u64,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitResponse {
    pub in_reply_to: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Echo {
    #[serde(rename = "echo")]
    Echo(EchoRequest),

    #[serde(rename = "echo_ok")]
    EchoOk(EchoResponse),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoRequest {
    pub msg_id: u64,
    pub echo: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoResponse {
    pub msg_id: u64,
    pub in_reply_to: u64,
    pub echo: String,
}
