use std::{collections::HashMap, io};

use color_eyre::Report;
use serde::Serialize;

use crate::models::*;
pub struct Handler;

type HandlerResult<T> = Result<T, Report>;

impl Handler {
    pub fn read_request(&self) -> HandlerResult<Message> {
        let msg_str = self.read_stdin()?;
        // eprintln!("Received {}", msg_str);

        let msg: Message = serde_json::from_str::<Message>(&msg_str)?;
        Ok(msg)
    }

    pub fn write_response<T>(&self, msg: &T) -> HandlerResult<()>
    where
        T: Serialize,
    {
        let output = serde_json::to_string(&msg)?;
        // eprintln!("Sending {}", output);

        println!("{}", output);
        Ok(())
    }

    pub fn write_error(
        &self,
        in_reply_to: u64,
        code: u64,
        text: impl AsRef<str>,
    ) -> HandlerResult<()> {
        let e = Error::new(in_reply_to, code, text);
        self.write_response(&e)
    }

    fn read_stdin(&self) -> Result<String, Report> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }
}
