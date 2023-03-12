mod handler;
mod models;

use models::*;
use serde_json::Value;

use crate::handler::Handler;

pub const MALFORMED_REQUEST: u64 = 12;

fn main() {
    let handler = Handler;
    let mut id_gen = IdGenerator::default();

    eprintln!("Online");

    loop {
        let msg = match handler.read_request() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error: Cannot parse the request: {}", e);
                continue;
            }
        };

        match msg.body.msg_type.as_str() {
            "echo" => handle_echo(&handler, &mut id_gen, msg),
            "init" => handle_init(&handler, &mut id_gen, msg),
            _ => {}
        }
    }
}

fn handle_init(handler: &Handler, id_gen: &mut IdGenerator, req: Message) {
    let body = Body::new("init_ok", id_gen.next(), req.body.msg_id.unwrap());
    let response = req.create_response(body);
    handler.write_response(&response).unwrap();
}

fn handle_echo(handler: &Handler, id_gen: &mut IdGenerator, req: Message) {
    let Some(echo_msg) = req.body.other.get("echo").map(Value::as_str) else {
        let Some(msg_id) = req.body.msg_id else {
            eprint!("Error: Request has no `msg_id` field");
        return;
        };

        handler
            .write_error(msg_id, MALFORMED_REQUEST, "Error: Echo workload has no `echo` field")
            .expect("cannot send error");
        return;
    };

    let body = Body::new("echo_ok", id_gen.next(), req.body.msg_id.unwrap())
        .with_extra_field("echo", echo_msg);

    let response = req.create_response(body);
    handler.write_response(&response).unwrap();
}
