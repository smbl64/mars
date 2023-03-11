use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, Clone, Builder, Deserialize, Serialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn build_response(&self) -> MessageBuilder {
        let mut builder = MessageBuilder::default();
        builder.src(self.dest.to_owned()).dest(self.src.to_owned());
        builder
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Body {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,

    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl Body {
    pub fn new(msg_type: impl AsRef<str>, msg_id: u64, in_reply_to: u64) -> Self {
        Self {
            msg_type: msg_type.as_ref().to_owned(),
            msg_id: Some(msg_id),
            in_reply_to: Some(in_reply_to),
            other: HashMap::new(),
        }
    }

    pub fn with_extra_field(mut self, name: &str, value: impl Into<Value>) -> Self {
        self.other.insert(name.to_owned(), value.into());
        self
    }
}

#[derive(Default)]
pub struct IdGenerator {
    value: AtomicU64,
}

impl IdGenerator {
    pub fn next(&mut self) -> u64 {
        self.value.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub in_reply_to: u64,
    pub code: u64,
    pub text: String,
}

impl Error {
    pub fn new(in_reply_to: u64, code: u64, text: impl AsRef<str>) -> Self {
        Self {
            msg_type: "error".to_owned(),
            in_reply_to,
            code,
            text: text.as_ref().to_owned(),
        }
    }
}
