mod id_gen;
mod models;
mod transport;

pub mod prelude {
    pub use crate::id_gen::IdGenerator;
    pub use crate::models::*;
    pub use crate::transport::*;
}

use crate::prelude::*;

type EchoMessage = Message<Echo>;

pub fn run() {
    let transport = Transport;
    let mut id_gen = IdGenerator::default();

    log::info!("Online");

    loop {
        let request = match transport.read_request() {
            Ok(m) => m,
            Err(e) => {
                log::warn!("Error: Cannot parse the request: {}", e);
                continue;
            }
        };

        match request.body {
            Body::Workload(Echo::Echo(ref payload)) => {
                handle_echo(&transport, &mut id_gen, &request, payload)
            }
            Body::Init(Init::Init(ref payload)) => handle_init(&transport, &request, payload),
            _ => {}
        }
    }
}

fn handle_init(handler: &Transport, request: &EchoMessage, payload: &InitRequest) {
    let body = Init::InitOk {
        in_reply_to: payload.msg_id,
    };
    let response = Message {
        src: request.dest.clone(),
        dest: request.src.clone(),
        body: Body::<Echo>::Init(body),
    };

    handler.write_response(&response).unwrap();
}

fn handle_echo(
    handler: &Transport,
    id_gen: &mut IdGenerator,
    request: &EchoMessage,
    payload: &EchoRequest,
) {
    let body = Echo::EchoOk(EchoResponse {
        in_reply_to: payload.msg_id,
        msg_id: id_gen.next(),
        echo: payload.echo.clone(),
    });
    let response = Message {
        src: request.dest.clone(),
        dest: request.src.clone(),
        body: Body::<Echo>::Workload(body),
    };
    handler.write_response(&response).unwrap();
}
