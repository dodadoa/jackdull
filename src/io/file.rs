use serde_json::Value;
use std::fs;
use log::debug;

pub async fn read_file() -> String {
    let value = {
        let file_content = fs::read_to_string("./texts/test.json").expect("LogRocket: error reading file");
        serde_json::from_str::<Value>(&file_content).expect("LogRocket: error serializing to JSON")
    };
    
    debug!("{:?}", value["content"].to_string());
    value["content"].to_string()
}

