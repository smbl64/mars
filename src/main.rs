mod handler;
mod models;
use std::collections::HashMap;

use models::*;
use serde_json::Value;

use crate::handler::Handler;

fn main() {
    let handler = Handler;
    let mut id_gen = IdGenerator::default();

    eprintln!("Online");

    loop {
        let msg = handler.read_request().unwrap();

        let Some(echo_msg) = msg.body.other.get("echo").map(Value::as_str) else {
            let Some(msg_id) = msg.body.msg_id else {
                eprint!("Error: Request has no `msg_id` field");
                continue;
            };

            handler.write_error(msg_id, 66, "Error: Echo workload has no `echo` field").expect("cannot send error");
            continue;
        };

        let mut response = Message {
            src: msg.dest.clone(),
            dest: msg.src.clone(),
            body: Body {
                msg_type: "echo_ok".to_owned(),
                msg_id: Some(id_gen.next()),
                in_reply_to: msg.body.msg_id,
                other: HashMap::new(),
            },
        };

        response
            .body
            .other
            .insert("echo".to_owned(), echo_msg.into());

        handler.write_response(&response).unwrap();
    }
}
