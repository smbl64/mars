use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message<W> {
    pub src: String,
    pub dest: String,
    pub body: Body<W>,
}

#[derive(Debug, Serialize)]
pub struct Body<W> {
    #[serde(rename = "type")]
    #[serde(skip_serializing)]
    pub msg_type: String,

    #[serde(flatten)]
    pub payload: Payload<W>,
}

impl<'de, W> Deserialize<'de> for Body<W>
where
    W: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Similar to BodyWrapper, but its body is a Value
        #[derive(Deserialize)]
        pub struct DataInner {
            #[serde(rename = "type")]
            pub msg_type: String,

            #[serde(flatten)]
            pub body: serde_json::Value,
        }

        // We will inject the msg_type as "type" into the body, then we will deserialize it as
        // a concerete Body struct. Body variants need a "type" as tag for deserialization.
        let mut data = DataInner::deserialize(deserializer)?;
        data.body
            .as_object_mut()
            .ok_or(serde::de::Error::custom("payload is not an object"))?
            .insert("type".to_owned(), data.msg_type.clone().into());
        let payload = Payload::<W>::deserialize(data.body).map_err(serde::de::Error::custom)?;

        Ok(Body {
            msg_type: data.msg_type,
            payload,
        })
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Payload<W> {
    Error(Error),
    Init(InitRequest),
    InitOk(InitResponse),

    Workload(W),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "error")]
#[serde(tag = "type")]
pub struct Error {
    pub in_reply_to: u64,
    pub code: u64,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "init")]
#[serde(tag = "type")]
pub struct InitRequest {
    pub msg_id: u64,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "init_ok")]
#[serde(tag = "type")]
pub struct InitResponse {
    pub in_reply_to: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Echo {
    #[serde(rename = "echo")]
    Echo(EchoRequest),

    #[serde(rename = "echo_ok")]
    EchoOk(EchoResponse),
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
    #[serde(rename = "test_payload")]
    #[serde(tag = "type")]
    pub struct TestPayload {
        pub in_reply_to: u64,
        pub text: String,
    }

    type TestBody = Body<TestPayload>;

    use super::*;
    #[test]
    fn serialize_init_req() {
        let e = InitRequest {
            msg_id: 1,
            node_id: "n1".to_string(),
            node_ids: vec!["n1".to_owned(), "n2".to_owned()],
        };

        let e = TestBody {
            msg_type: "woot".to_owned(), // It doesn't matter!
            payload: Payload::<TestPayload>::Init(e),
        };

        println!("SERIALIZE");
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
        let b: TestBody = serde_json::from_str(&json).unwrap();
        assert_eq!(b.msg_type, "error".to_owned());
        assert!(matches!(b.payload, Payload::Error(..)));
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
        let b: TestBody = serde_json::from_str(&json).unwrap();
        assert_eq!(b.msg_type, "init".to_owned());
        assert!(matches!(b.payload, Payload::Init(..)));
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

        let b: TestBody = serde_json::from_str(&json).unwrap();
        dbg!(&b);
        assert_eq!(b.msg_type, "test_payload".to_owned());
        assert!(matches!(b.payload, Payload::Workload(TestPayload { .. })));
    }
}
