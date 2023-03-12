use std::io;

use color_eyre::Report;
use serde::Serialize;

use crate::models::*;
pub struct Transport;

type HandlerResult<T> = Result<T, Report>;

impl Transport {
    /// Reads a new request from stdin and deserialize it to a `Message`.
    pub fn read_request(&self) -> HandlerResult<Message> {
        let msg_str = self.read_stdin()?;
        // eprintln!("Received {}", msg_str);

        let msg: Message = serde_json::from_str(&msg_str)?;
        Ok(msg)
    }

    /// Serializes the response and writes it to stdout.
    pub fn write_response<T>(&self, response: &T) -> HandlerResult<()>
    where
        T: Serialize,
    {
        let output = serde_json::to_string(&response)?;
        // eprintln!("Sending {}", output);

        println!("{}", output);
        Ok(())
    }

    /// A helper method to create an error response and send it.
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
