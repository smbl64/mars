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

            let payload = match request.body.payload {
                Payload::Init(ref payload) => self.handle_init(payload),
                Payload::Workload(Echo::Echo(ref payload)) => self.handle_echo(payload),
                _ => continue,
            };
            let response = Message {
                src: request.dest.clone(),
                dest: request.src.clone(),
                body: Body {
                    msg_type: "".to_owned(), // Not needed -- will be handled by deserializer
                    payload,
                },
            };

            self.ctx.transport.write_response(&response).unwrap();
        }
    }

    pub fn get_next_id(&mut self) -> u64 {
        self.ctx.id_gen.next()
    }

    fn handle_init(&mut self, payload: &InitRequest) -> Payload<Echo> {
        let body: Payload<_> = Payload::<Echo>::InitOk(InitResponse {
            in_reply_to: payload.msg_id,
        });
        body
    }

    fn handle_echo(&mut self, payload: &EchoRequest) -> Payload<Echo> {
        let body = Echo::EchoOk(EchoResponse {
            in_reply_to: payload.msg_id,
            msg_id: self.get_next_id(),
            echo: payload.echo.clone(),
        });
        Payload::<Echo>::Workload(body)
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
