use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Client,
    framework::standard::StandardFramework,
    model::gateway::GatewayIntents,
};
use std::process::Command;
use tempfile::NamedTempFile;
use dotenv::dotenv;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!compile") {
            if let Some(code_block_start) = msg.content.find("```rust") {
                let code_block_trimmed = &msg.content[code_block_start + 7..];
                if let Some(code_block_end) = code_block_trimmed.find("```") {
                    let code = &code_block_trimmed[..code_block_end].trim();

                    let full_code = format!("fn main() {{\n{}\n}}", code);
                    let mut file = NamedTempFile::new().expect("Failed to create temp file");
                    std::fs::write(file.path(), full_code).expect("Failed to write to temp file");

                    let output = Command::new("rustc")
                        .arg(file.path())
                        .arg("--out-dir")
                        .arg(file.path().parent().unwrap())
                        .arg("--crate-name")
                        .arg("user_code")
                        .output()
                        .expect("Failed to execute process");

                    if output.status.success() {
                        let executable = file.path().parent().unwrap().join("user_code");
                        let execution_output = Command::new(&executable)
                            .output()
                            .expect("Failed to execute binary");

                        if execution_output.status.success() {
                            let stdout = String::from_utf8_lossy(&execution_output.stdout);
                            let message = if stdout.is_empty() {
                                "No output".to_string()
                            } else {
                                format!("Output:\n```\n{}\n```", stdout)
                            };
                            let _ = msg.channel_id.say(&ctx.http, message).await;
                        } else {
                            let stderr = String::from_utf8_lossy(&execution_output.stderr);
                            let message = format!("Execution Error:\n```\n{}\n```", stderr);
                            let _ = msg.channel_id.say(&ctx.http, message).await;
                        }
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let message = format!("Compilation Error:\n```\n{}\n```", stderr);
                        let _ = msg.channel_id.say(&ctx.http, message).await;
                    }
                } else {
                    let _ = msg.channel_id.say(&ctx.http, "Error: No se encontró el final del bloque de código Rust.").await;
                }
            } else {
                let _ = msg.channel_id.say(&ctx.http, "Error: Por favor, proporciona el código en un bloque de código Rust.").await;
            }
        } else if msg.content == "!example" {
            let example_code = r#"```rust
fn main() {
    println!("Hello, world!");
}
```"#;
            let _ = msg.channel_id.say(&ctx.http, example_code).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(StandardFramework::new().configure(|c| c.prefix("!")))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
