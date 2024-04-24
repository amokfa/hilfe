use std::path::PathBuf;
use std::process::exit;
use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};
use openai::set_key;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub system_prompt: String,
}

pub const CLI_PROMPT_1: &str = "You are an interactive shell command line shell agent.
You just get things done, rather than trying to explain.
Do your best to respond with 1 command that will meet the requirements.
Start a line with a dollar symbol to have it sent directly to the shell VERBATIM.
This line must not have anything other than the command, no text, no comments, no nothing, just shell command
All other output is just echoed.
Favor 1 line shell commands.
Be terse.";

pub const CLI_PROMPT_2: &str = "Here is the query:";

pub fn config_dir_path() -> PathBuf {
    #[cfg(target_os = "linux")]
    let path = dirs::config_dir()
        .expect("Failed to find config directory"); // Replace "your_app_name" with your actual application name

    #[cfg(target_os = "macos")]
    let path = dirs::application_support_dir()
        .expect("Failed to find application support directory"); // Replace "your_app_name" with your actual application name

    #[cfg(target_os = "windows")]
    let path = dirs::config_dir()
        .expect("Failed to find config directory"); // Replace "your_app_name" with your actual application name

    path
}
pub fn config_file_path() -> PathBuf {
    config_dir_path()
        .join("hilfe.toml")
}

pub fn zsh_helper_path() -> PathBuf {
    config_dir_path()
        .join("hilfe.zsh")
}

pub fn get_system_info() -> String {
    let osi = os_info::get();
    format!("OS : {:?} {:?} {:?}\nSHELL : {:?}", osi.os_type(), osi.version(), osi.edition().unwrap_or(""), std::env::var("SHELL"))
}

#[tokio::main]
async fn main() {
    let args = std::env::args()
        .skip(1)
        .collect::<Vec<_>>();
    let config_path = config_file_path();

    let zsh_helper = zsh_helper_path();
    let md = std::fs::metadata(zsh_helper.as_path());
    if md.is_err() {
        std::fs::write(
            zsh_helper.as_path(),
            include_str!("../../resources/prompt.zsh"),
        ).unwrap();
    }

    let md = std::fs::metadata(config_path.as_path());
    if md.is_err() {
        std::fs::write(
            config_path.as_path(),
            include_str!("../../resources/config_template.toml"),
        ).unwrap();
    } else if !md.unwrap().is_file() {
        std::fs::remove_dir_all(config_path.as_path()).unwrap();
        std::fs::write(
            config_path.as_path(),
            include_str!("../../resources/config_template.toml"),
        ).unwrap();
    }

    let curr_config = match toml::from_str::<Config>(
        std::fs::read_to_string(config_path.as_path()).unwrap().as_str()
    ) {
        Ok(config) => config,
        Err(_e) => {
            println!("couldn't parse config file, please edit {config_path:?}");
            exit(1);
        }
    };
    match args.get(0).map(|s| s.as_str()) {
        Some("--alias") => {
            println!("Save this to your zsh config:");
            println!("alias '??'='source {}'", zsh_helper.to_str().unwrap());
        }
        _ => {
            let query = args.join(" ");
            set_key(curr_config.api_key.clone());
            let completion = openai::chat::ChatCompletion::builder(curr_config.model.as_str(), vec![
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::System,
                    content: Some(CLI_PROMPT_1.to_owned()),
                    name: None,
                    function_call: None,
                },
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::System,
                    content: Some(get_system_info()),
                    name: None,
                    function_call: None,
                },
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::System,
                    content: Some(format!("User has shared this information about the system: {}", curr_config.system_prompt)),
                    name: None,
                    function_call: None,
                },
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::System,
                    content: Some(CLI_PROMPT_2.to_owned()),
                    name: None,
                    function_call: None,
                },
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::User,
                    content: Some(query),
                    name: None,
                    function_call: None,
                },
            ])
                .create()
                .await.unwrap();

            let response = completion.choices.first()
                .and_then(|c| c.message.content.to_owned())
                .unwrap();
            let cmd = response
                .lines()
                .map(|l| l.trim())
                .filter(|l| l.starts_with('$'))
                .nth(0)
                .map(|s| &s[1..])
                .map(|s| s.trim())
                .unwrap_or_else(|| {
                    "echo 'I dunno man'"
                });
            println!("{cmd}");
            std::fs::write("/tmp/hilfe", response).unwrap();
        }
    }
}