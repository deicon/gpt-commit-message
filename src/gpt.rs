use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize)]
pub struct RequestBody {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f64,
}
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Message{
            role: role.to_string(),
            content: content.to_string()
        }
    }
}

pub fn read_preparation(preparation_path: &PathBuf) -> Vec<Message> {
        // read list of messages as json from File
        let mut content = vec![];
        let _ = File::open(&preparation_path).unwrap().read_to_end(&mut content);
        let system_prompt_from_file = String::from_utf8(content).expect("Not valid UTF 8");
        let from_file: Vec<Message> = from_str(&system_prompt_from_file).expect("Unable to parse Result as Messages");
        from_file
}


#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
    system_fingerprint: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Choice {
    index: u32,
    pub message: ResultMessage,
    logprobs: Option<serde_json::Value>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ResultMessage {
    role: String,
    pub content: String,
    refusal: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
    completion_tokens_details: CompletionTokensDetails,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct CompletionTokensDetails {
    reasoning_tokens: u32,
}

