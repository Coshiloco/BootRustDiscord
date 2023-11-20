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

impl Handler {
    fn extract_rust_code(content: &str) -> Option<String> {
        let start_pattern = "```rust";
        let end_pattern = "```";
        if let Some(start) = content.find(start_pattern) {
            if let Some(end) = content[start + start_pattern.len()..].find(end_pattern) {
                let code = &content[start + start_pattern.len()..start + start_pattern.len() + end].trim();
                return Some(code.to_string());
            }
        }
        None
    }

    async fn compile_and_execute_rust_code(ctx: &Context, msg: &Message, code: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        std::fs::write(file.path(), code)?;

        let output = Command::new("rustc")
            .arg(file.path())
            .arg("--out-dir")
            .arg(file.path().parent().unwrap())
            .arg("--crate-name")
            .arg("user_code")
            .output()?;

        if !output.status.success() {
            let error_message = format!("Compilation error:\n```\n{}\n```", String::from_utf8_lossy(&output.stderr));
            msg.channel_id.say(&ctx.http, &error_message).await?;
            return Ok(());
        }

        let executable_path = file.path().parent().unwrap().join(if cfg!(target_os = "windows") { "user_code.exe" } else { "user_code" });
        let execution_output = Command::new(&executable_path).output()?;

        if !execution_output.status.success() {
            let error_message = format!("Execution error:\n```\n{}\n```", String::from_utf8_lossy(&execution_output.stderr));
            msg.channel_id.say(&ctx.http, &error_message).await?;
            return Ok(());
        }

        let output_message = format!("Output:\n```\n{}\n```", String::from_utf8_lossy(&execution_output.stdout));
        msg.channel_id.say(&ctx.http, &output_message).await?;

        Ok(())
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!compile") {
            if let Some(code) = Handler::extract_rust_code(&msg.content) {
                if let Err(why) = Handler::compile_and_execute_rust_code(&ctx, &msg, &code).await {
                    println!("Error handling rust code: {:?}", why);
                }
            } else {
                let _ = msg.channel_id.say(&ctx.http, "Please provide Rust code inside a code block.").await;
            }
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
