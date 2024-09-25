use std::fs::File;
use std::io::{self, Read};
use std::process::exit;
use clap::Parser;
use serde_json::{from_str, to_string};
use crate::cli::{Cli, Commands};
use crate::gpt::{read_preparation, ChatCompletion, Message, RequestBody};

mod gpt;
mod cli;


#[tokio::main]
async fn main() {
    let default_prompt =
        Message::new(
            "system",
            "You are a friendly assistant that generates conventional commit messages. You only print the commit message without any additional text.",
        );

    let cli = Cli::parse();

    if let Some(Commands::CONFIG) = cli.command {
        let defaults = vec![default_prompt];
        println!("{}", to_string(&defaults).expect("Message can not be printed as json"));
        exit(0)
    }

    let apikey = std::env::var("GPT_API_KEY");
    let gpt_url = "https://api.openai.com/v1/chat/completions";

    let mut user_prompt: String = String::new();
    let mut buff: Vec<u8> = vec![];
    if let Some(input_file) = cli.file {
        if let Ok(_n) = File::open(input_file).expect("Unable to open input file").read_to_end(&mut buff) {
            user_prompt = String::from_utf8(buff).expect("Not valid UTF 8");
        }
    } else {
        let mut stdin = io::stdin().lock();
        if let Ok(_n) = stdin.read_to_end(&mut buff) {
            user_prompt = String::from_utf8(buff).expect("Not valid UTF 8");
        }
    }
    // first get input from stdin
    if let Ok(key) = apikey {
        let mut messages: Vec<Message> = vec![];
        if let Some(preparation_path) = cli.preparation {
            // read system prompt from file
            messages.extend(read_preparation(&preparation_path));
        } else {
            messages.insert(messages.len(), default_prompt);
        }

        let client = reqwest::Client::new();
        messages.insert(messages.len(),
                        Message::new("user", &user_prompt),
        );

        let res = client.post(gpt_url)
            .bearer_auth(key)
            .header("Content-Type", "application/json")
            .json(&RequestBody {
                model: "gpt-4o-mini".to_string(),
                temperature: 0.3,
                messages,
            })
            .send()
            .await;
        if let Ok(result) = res {
            let json_text = result.text().await.expect("Commit message not received");
            let chat_completion: ChatCompletion = from_str(&json_text).expect("Failed to deserialize JSON");

            for choice in chat_completion.choices {
                println!("{}", choice.message.content);
            }
        }
    }
}
