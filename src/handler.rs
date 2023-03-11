use std::{collections::HashMap, io};

use color_eyre::Report;

use crate::models::*;
pub struct Handler;

type HandlerResult<T> = Result<T, Report>;

impl Handler {
    pub fn read_request(&self) -> HandlerResult<Message> {
        let msg_str = self.read_stdin()?;
        eprintln!("Received {}", msg_str);

        let msg: Message = serde_json::from_str(&msg_str)?;
        Ok(msg)
    }

    pub fn write_response(&self, msg: &Message) -> HandlerResult<()> {
        let output = serde_json::to_string(&msg)?;
        eprintln!("Sending {}", output);

        println!("{}", output);
        Ok(())
    }

    pub fn write_error(&self, in_reply_to: u64, code: u64, message: &str) -> HandlerResult<()> {
        let mut body = Body {
            msg_type: "error".to_owned(),
            in_reply_to: Some(in_reply_to),
            msg_id: None,
            other: HashMap::new(),
        };
        body.other.insert("code".to_owned(), code.into());
        body.other.insert("text".to_owned(), message.into());

        let response = Message {
            src: "TODO".to_owned(),
            dest: "TODO".to_owned(),
            body,
        };
        self.write_response(&response)
    }

    fn read_stdin(&self) -> Result<String, Report> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }
}
