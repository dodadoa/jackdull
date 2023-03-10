use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct TypingFile {
    pub from: String,
    pub content: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct TypingFileDisplay {
    pub from: String,
    pub content: String,
    pub url: String,
    pub words_count: u32,
}

pub async fn read_file() -> TypingFileDisplay {
    let value= {
        let file_content = fs::read_to_string("./texts/1.json").expect("LogRocket: error reading file");
        serde_json::from_str::<TypingFile>(&file_content).expect("error serializing to JSON")
    };
    
    TypingFileDisplay {
        from: value.from,
        content: value.content.clone(),
        url: value.url,
        words_count: value.content.split_whitespace().count() as u32,
    }
}
