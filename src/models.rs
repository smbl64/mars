use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,

    #[serde(flatten)]
    pub other: HashMap<String, Value>,
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
