use std::{
    fs::File,
    io::{BufRead, Read},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use rev_buf_reader::RevBufReader;
use serde_derive::{Deserialize, Serialize};

mod prompts;

const HISTORY_LINE_COUNT: u32 = 10;
const USAGE_ENDPOINT: &str = "https://api.openai.com/v1/usage";

#[derive(Parser)]
struct Cli {
    #[arg(trailing_var_arg = true)]
    /// optional input to send in prompt
    user_input: Option<Vec<String>>,

    #[arg(short, long, value_name = "FILE")]
    /// set a config file
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // explain a command / concept
    Explanation {
        #[arg(trailing_var_arg = true)]
        /// input to send in prompt
        user_input: Option<Vec<String>>,

        #[arg(short, long)]
        verbose: bool,
    },

    // read terminal history and existing aliases
    // then provide suggestions of new aliases to improve workflow
    Suggestions {},

    // read file data and send it to gpt
    // itll summarize and use it to answer questions
    Summary {
        file: PathBuf,
    },

    // get usage in tokens, and bill
    Usage {},
}

#[derive(Serialize, Deserialize)]
struct MyConfig {
    api_key: String,
    history_file: PathBuf,
}

#[derive(Deserialize, Debug)]
struct UsageResponse {
    object: String,
    data: Vec<serde_json::Value>, // Adjust types based on actual response structure
    tpm_data: Vec<serde_json::Value>,
    ft_data: Vec<serde_json::Value>,
    dalle_api_data: Vec<serde_json::Value>,
    whisper_api_data: Vec<serde_json::Value>,
    tts_api_data: Vec<serde_json::Value>,
    assistant_code_interpreter_data: Vec<serde_json::Value>,
    retrieval_storage_data: Vec<serde_json::Value>,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            api_key: "".into(),
            history_file: "".into(),
        }
    }
}

fn read_history(history_file: &PathBuf, lines: u32) -> Vec<String> {
    let file = File::open(history_file).expect("could not open history file!");
    let mut reader = RevBufReader::new(file);
    let mut history: Vec<String> = Vec::new();
    for _ in 0..lines {
        let mut res: Vec<String> = Vec::new();
        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf).ok();
            let semicolon_idx = buf.find(';');
            if let Some(idx) = semicolon_idx {
                buf = buf[idx + 1..].to_string();
                res.push(buf);
                break;
            }
            res.push(buf);
        }
        res.reverse();
        history.push(res.join(""));
    }
    history
}

fn read_file_content(path: &PathBuf) -> String {
    let mut file = File::open(path).expect("unable to open file!");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("could not read file contents");
    buf
}

async fn get_usage(cfg: &MyConfig) {
    let client = reqwest::Client::new();
    let resp = client
        .get(USAGE_ENDPOINT.to_string() + "?date=2024-07-01")
        .bearer_auth(&cfg.api_key)
        .send()
        .await
        .expect("failed to send request for usage");
    let data: UsageResponse = resp.json().await.expect("failed to get text from response");
    println!("{:?}", data);
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    let cfg: MyConfig = confy::load("her", None).expect("error with config");
    use Commands::*;
    match &cli.command {
        Some(Suggestions {}) => {
            let history = read_history(&cfg.history_file, HISTORY_LINE_COUNT);
            for (i, line) in history.iter().enumerate() {
                print!("line {}: {}", i + 1, line)
            }
        }
        Some(Summary { file }) => {
            println!("summarizing file: {:?}", file);
            let data = read_file_content(file);
            print!("contents:\n{}", data);
        }
        Some(Explanation {
            user_input,
            verbose,
        }) => match user_input {
            Some(input) => {
                println!("explaining {} with verbose: {:?}", input.join(" "), verbose);
            }
            None => {
                println!("asking user what they want to explain");
            }
        },
        Some(Usage {}) => {
            println!("getting usage");
            get_usage(&cfg).await;
        }
        None => match &cli.user_input {
            Some(words) => {
                println!("opening cli chat with input: {:?}", words.join(" "));
            }
            None => {
                println!("opening chat");
            }
        },
    }
    Ok(())
}
