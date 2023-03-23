use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "type")]
    pub msg_type: String,

    #[serde(flatten)]
    pub payload: Value,
}

impl Body {
    pub fn new<P>(msg_type: &str, payload: P) -> Self
    where
        P: Serialize,
    {
        Self {
            msg_type: msg_type.to_string(),
            payload: serde_json::to_value(&payload).unwrap(),
        }
    }

    pub fn payload_as<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        // TODO do we need clone?
        serde_json::from_value(self.payload.clone())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub in_reply_to: u64,
    pub code: u64,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitRequest {
    pub msg_id: u64,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitResponse {
    pub in_reply_to: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoRequest {
    pub msg_id: u64,
    pub echo: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoResponse {
    pub msg_id: u64,
    pub in_reply_to: u64,
    pub echo: String,
}

#[cfg(test)]
mod tests {

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TestPayload {
        pub in_reply_to: u64,
        pub text: String,
    }

    use super::*;
    #[test]
    fn serialize_init_req() {
        let payload = InitRequest {
            msg_id: 1,
            node_id: "n1".to_string(),
            node_ids: vec!["n1".to_string(), "n2".to_string()],
        };
        let e = Body::new("init", payload);

        let json = serde_json::to_string(&e).unwrap();
        println!("{}", json);
        let expected = r#"{"type":"init","msg_id":1,"node_id":"n1","node_ids":["n1","n2"]}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn deserialize_error_req() {
        let json = r#"
            {
                "type": "error",
                "in_reply_to": 123,
                "code": 42,
                "text": "Hello"
            }
        "#;
        let b: Body = serde_json::from_str(&json).unwrap();
        assert_eq!(b.msg_type, "error".to_owned());
        let payload: Result<Error, serde_json::Error> = b.payload_as();
        assert!(payload.is_ok());
    }

    #[test]
    fn deserialize_init_req() {
        let json = r#"
            {
                "type": "init",
                "msg_id": 1,
                "node_id": "n1",
                "node_ids": ["n1", "n2"]
            }
        "#;
        let b: Body = serde_json::from_str(&json).unwrap();
        assert_eq!(b.msg_type, "init".to_owned());
        let payload: Result<InitRequest, serde_json::Error> = b.payload_as();
        assert!(payload.is_ok());
    }

    #[test]
    fn deserialize_workload() {
        let json = r#"
            {
                "type": "test_payload",
                "in_reply_to": 1,
                "text": "hey there"
            }
        "#;

        let b: Body = serde_json::from_str(&json).unwrap();
        assert_eq!(b.msg_type, "test_payload".to_owned());
        let payload: Result<TestPayload, serde_json::Error> = b.payload_as();
        assert!(payload.is_ok());
    }
}
