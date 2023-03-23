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

            let body = match request.body.msg_type.as_str() {
                "init" => self.handle_init(&request),
                "echo" => self.handle_echo(&request),
                _ => continue,
            };

            let body = match body {
                Ok(b) => b,
                Err(s) => {
                    log::error!("Cannot process request: {}", s);
                    continue;
                }
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

    fn handle_init(&mut self, request: &Message) -> Result<Body, String> {
        let request: InitRequest = request.body.payload_as().map_err(|e| e.to_string())?;
        let payload = InitResponse {
            in_reply_to: request.msg_id,
        };

        Ok(Body::new("init_ok", payload))
    }

    fn handle_echo(&mut self, request: &Message) -> Result<Body, String> {
        let request: EchoRequest = request.body.payload_as().map_err(|e| e.to_string())?;
        let payload = EchoResponse {
            in_reply_to: request.msg_id,
            msg_id: self.get_next_id(),
            echo: request.echo.clone(),
        };

        Ok(Body::new("echo_ok", payload))
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
