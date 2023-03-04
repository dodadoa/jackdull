use serde::{Deserialize, Serialize};
use std::fs;
use log::debug;

#[derive(Serialize, Deserialize, Debug)]
struct TypingFile {
    from: String,
    content: String,
    url: String
}

pub async fn read_file() -> String {
    let value= {
        let file_content = fs::read_to_string("./texts/1.json").expect("LogRocket: error reading file");
        serde_json::from_str::<TypingFile>(&file_content).expect("error serializing to JSON")
    };
    
    debug!("{:?}", value);
    value.content.to_string()
}
