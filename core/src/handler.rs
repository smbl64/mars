use serde::Serialize;

use crate::prelude::*;

pub trait Service<M>
where
    M: Serialize,
{
    fn handle(&mut self, msg: &M);
}

pub struct Server {
    ctx: Context,
}

pub type EchoMessage = Message<Echo>;

impl Default for Server {
    fn default() -> Self {
        Self {
            ctx: Context::default(),
        }
    }
}
impl Server {
    pub fn run(&mut self) {
        log::info!("Online");

        loop {
            let request = match self.ctx.transport.read_request() {
                Ok(m) => m,
                Err(e) => {
                    log::warn!("Error: Cannot parse the request: {}", e);
                    continue;
                }
            };

            match request.body {
                Body::Init(ref payload) => self.handle_init(&request, payload),
                Body::Workload(Echo::Echo(ref payload)) => self.handle_echo(&request, payload),
                _ => {}
            }
        }
    }

    fn handle_init(&mut self, request: &EchoMessage, payload: &InitRequest) {
        let body: Body<_> = Body::<Echo>::InitOk(InitResponse {
            in_reply_to: payload.msg_id,
        });
        let response = Message {
            src: request.dest.clone(),
            dest: request.src.clone(),
            body,
        };

        self.ctx.transport.write_response(&response).unwrap();
    }

    fn handle_echo(&mut self, request: &EchoMessage, payload: &EchoRequest) {
        let body = Echo::EchoOk(EchoResponse {
            in_reply_to: payload.msg_id,
            msg_id: self.ctx.id_gen.next(),
            echo: payload.echo.clone(),
        });
        let response = Message {
            src: request.dest.clone(),
            dest: request.src.clone(),
            body: Body::<Echo>::Workload(body),
        };
        self.ctx.transport.write_response(&response).unwrap();
    }
}

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
