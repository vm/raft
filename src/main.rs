extern crate serde;

pub mod message;

use message::{RequestMessage, RequestType};

fn main() {
    let msg = RequestMessage {
        src: "xgjoweijg".to_string(),
        dst: "gojweogij".to_string(),
        leader: "jgowejgwiajg".to_string(),
        mid: "wiogjoweijg".to_string(),
        value: Option::None,
        request_type: RequestType::Get,
    };

    let serialized = serde_json::to_string(&msg).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: RequestMessage = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
