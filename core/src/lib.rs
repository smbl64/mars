mod handler;
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

pub struct Context {
    pub transport: Transport,
    pub id_gen: IdGenerator,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            transport: Transport,
            id_gen: IdGenerator::default(),
        }
    }
}

pub fn run() {
    let mut ctx = Context::default();

    log::info!("Online");

    loop {
        let request = match ctx.transport.read_request() {
            Ok(m) => m,
            Err(e) => {
                log::warn!("Error: Cannot parse the request: {}", e);
                continue;
            }
        };

        match request.body {
            Body::Workload(Echo::Echo(ref payload)) => handle_echo(&mut ctx, &request, payload),
            Body::Init(Init::Init(ref payload)) => handle_init(&mut ctx, &request, payload),
            _ => {}
        }
    }
}

fn handle_init(ctx: &mut Context, request: &EchoMessage, payload: &InitRequest) {
    let body = Init::InitOk {
        in_reply_to: payload.msg_id,
    };
    let response = Message {
        src: request.dest.clone(),
        dest: request.src.clone(),
        body: Body::<Echo>::Init(body),
    };

    ctx.transport.write_response(&response).unwrap();
}

fn handle_echo(ctx: &mut Context, request: &EchoMessage, payload: &EchoRequest) {
    let body = Echo::EchoOk(EchoResponse {
        in_reply_to: payload.msg_id,
        msg_id: ctx.id_gen.next(),
        echo: payload.echo.clone(),
    });
    let response = Message {
        src: request.dest.clone(),
        dest: request.src.clone(),
        body: Body::<Echo>::Workload(body),
    };
    ctx.transport.write_response(&response).unwrap();
}
