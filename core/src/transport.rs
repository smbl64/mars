use std::io;

use color_eyre::Report;
use serde::Serialize;

use crate::models::*;
pub struct Transport;

type TransportResult<T> = Result<T, Report>;

impl Transport {
    /// Reads a new request from stdin and deserialize it to a `Message`.
    pub fn read_request(&self) -> TransportResult<Message> {
        let msg_str = self.read_stdin()?;
        log::debug!("Received {}", msg_str);

        let msg: Message = serde_json::from_str(&msg_str)?;
        Ok(msg)
    }

    /// Serializes the response and writes it to stdout.
    pub fn write_response<T>(&self, response: &T) -> TransportResult<()>
    where
        T: Serialize,
    {
        let output = serde_json::to_string(&response)?;
        log::debug!("Sending {}", output);

        println!("{}", output);
        Ok(())
    }

    fn read_stdin(&self) -> Result<String, Report> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }
}
