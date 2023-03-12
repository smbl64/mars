mod models;
mod transport;

use models::*;
use serde_json::Value;

use crate::transport::Transport;

pub const MALFORMED_REQUEST: u64 = 12;

fn main() {
    std::env::set_var(env_logger::DEFAULT_FILTER_ENV, "debug");

    // By default env_logger logs to stderr, which is what we want
    env_logger::init();

    let transport = Transport;
    let mut id_gen = IdGenerator::default();

    log::info!("Online");

    loop {
        let msg = match transport.read_request() {
            Ok(m) => m,
            Err(e) => {
                log::warn!("Error: Cannot parse the request: {}", e);
                continue;
            }
        };

        match msg.body.msg_type.as_str() {
            "echo" => handle_echo(&transport, &mut id_gen, msg),
            "init" => handle_init(&transport, &mut id_gen, msg),
            _ => {}
        }
    }
}

fn handle_init(handler: &Transport, id_gen: &mut IdGenerator, req: Message) {
    let body = Body::new("init_ok", id_gen.next(), req.body.msg_id.unwrap());
    let response = req.create_response(body);
    handler.write_response(&response).unwrap();
}

fn handle_echo(handler: &Transport, id_gen: &mut IdGenerator, req: Message) {
    let Some(echo_msg) = req.body.other.get("echo").map(Value::as_str) else {
        let Some(msg_id) = req.body.msg_id else {
            log::warn!("Error: Request has no `msg_id` field");
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
