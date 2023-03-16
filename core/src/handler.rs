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

            let body = match request.body {
                Body::Init(ref payload) => self.handle_init(payload),
                Body::Workload(Echo::Echo(ref payload)) => self.handle_echo(payload),
                _ => continue,
            };
            let response = Message {
                src: request.dest.clone(),
                dest: request.src.clone(),
                body,
            };

            self.ctx.transport.write_response(&response).unwrap();
        }
    }

    pub fn get_next_id(&mut self) -> u64 {
        self.ctx.id_gen.next()
    }

    fn handle_init(&mut self, payload: &InitRequest) -> Body<Echo> {
        let body: Body<_> = Body::<Echo>::InitOk(InitResponse {
            in_reply_to: payload.msg_id,
        });
        body
    }

    fn handle_echo(&mut self, payload: &EchoRequest) -> Body<Echo> {
        let body = Echo::EchoOk(EchoResponse {
            in_reply_to: payload.msg_id,
            msg_id: self.get_next_id(),
            echo: payload.echo.clone(),
        });
        Body::<Echo>::Workload(body)
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
