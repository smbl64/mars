mod models;
mod transport;

pub mod prelude {
    pub use crate::models::*;
    pub use crate::transport::*;
}

use crate::prelude::*;

pub const MALFORMED_REQUEST: u64 = 12;

type EchoMessage = Message<Echo>;

pub fn run() {
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

        match msg.body {
            Body::Workload(Echo::Echo(ref payload)) => {
                handle_echo(&transport, &mut id_gen, &msg, payload)
            }
            Body::Init(Init::Init(ref payload)) => handle_init(&transport, &msg, payload),
            _ => {}
        }
    }
}

fn handle_init(handler: &Transport, msg: &EchoMessage, payload: &InitRequest) {
    let body = Init::InitOk {
        in_reply_to: payload.msg_id,
    };
    let response = Message {
        src: msg.dest.clone(),
        dest: msg.src.clone(),
        body: Body::<Echo>::Init(body),
    };

    handler.write_response(&response).unwrap();
}

fn handle_echo(
    handler: &Transport,
    id_gen: &mut IdGenerator,
    msg: &EchoMessage,
    payload: &EchoRequest,
) {
    let body = Echo::EchoOk(EchoResponse {
        in_reply_to: payload.msg_id,
        msg_id: id_gen.next(),
        echo: payload.echo.clone(),
    });
    let response = Message {
        src: msg.dest.clone(),
        dest: msg.src.clone(),
        body: Body::<Echo>::Workload(body),
    };
    handler.write_response(&response).unwrap();
    // let Some(echo_msg) = req.body.other.get("echo").map(Value::as_str) else {
    //     let Some(msg_id) = req.body.msg_id else {
    //         log::warn!("Error: Request has no `msg_id` field");
    //     return;
    //     };
    //
    //     handler
    //         .write_error(msg_id, MALFORMED_REQUEST, "Error: Echo workload has no `echo` field")
    //         .expect("cannot send error");
    //     return;
    // };
    //
    // let body = Body::new("echo_ok", id_gen.next(), req.body.msg_id.unwrap())
    //     .with_extra_field("echo", echo_msg);
    //
    // let response = req.create_response(body);
    // handler.write_response(&response).unwrap();
}
