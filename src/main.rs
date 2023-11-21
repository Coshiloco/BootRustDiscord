// Importaciones necesarias
use serenity::{
    async_trait,
    model::{
        channel::{Message, ReactionType, Reaction},
        gateway::{Ready, GatewayIntents},
        id::{ChannelId, MessageId},
        event::ResumedEvent,
    },
    framework::standard::{
        macros::{command, group},
        StandardFramework,
        CommandResult,
    },
    prelude::*,
    utils::Colour,
    http::Http,
};

use std::{env, process::Command};
use tempfile::NamedTempFile;
use dotenv::dotenv;




struct Handler;

impl Handler {

         // Esta función se encargará de enviar el mensaje de bienvenida.
         async fn send_welcome_message(&self, ctx: &Context) {
            let channel_id = ChannelId(1123024565178744962); // Reemplaza con el ID de tu canal real.
    
            let _ = channel_id.send_message(&ctx.http, |m| {
                m.content("¡Hola! Soy un bot que compila código Rust. Aquí puedes probar tus códigos:");
                m.embed(|e| {
                    e.title("Funcionalidades del Bot Rust")
                     .description("Este bot puede compilar y ejecutar tu código Rust. Usa el comando `!compile` seguido de tu código en un bloque de código para probarlo.")
                     .field("¿Cómo usar este bot?", "A continuación, te explico cómo puedes interactuar conmigo:", false)
                     .field("Compilar Código", "Reacciona con 🔨 y te enviaré una plantilla de código que puedes compilar.", false)
                     .field("Obtener un Ejemplo", "Reacciona con 📚 y te proporcionaré un ejemplo de código Rust.", false)
                     .field("Ejemplo de Compilación", "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```", false)
                     .colour(Colour::from_rgb(0, 255, 0))
                });
                m.reactions(vec![
                    ReactionType::Unicode(String::from("🔨")), // Compilar
                    ReactionType::Unicode(String::from("📚")), // Ejemplos
                ])
            }).await.expect("Error al enviar el mensaje.");
        }
    
    
// Asegúrate de actualizar la firma de send_compile_example para aceptar ChannelId directamente.
async fn send_compile_example(&self, ctx: &Context, channel_id: ChannelId) {
    let example_code = "¡Aquí tienes un ejemplo de código Rust que puedes compilar!\n\nEscribe en el chat el siguiente comando y código:\n\n!compile \\`\\`\\`rust\nfn main() {\n    println!(\"Esto es un ejemplo que compila\");\n}\n\\`\\`\\`";
    let _ = channel_id.say(&ctx.http, example_code).await.expect("Error sending message");
}
    

    async fn handle_example_command(&self, ctx: &Context, reaction: &Reaction) {
        // Simplemente envía un ejemplo de código fijo al canal donde se reaccionó
        let channel_id = reaction.channel_id;
        let example_code = r#"```rust
fn main() {
    println!("Hello, World!");
}
```"#;
        let _ = channel_id.say(&ctx.http, example_code).await.expect("Error sending message");
    }


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
    
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Some(guild_id) = reaction.guild_id {
            if let Ok(member) = guild_id.member(&ctx.http, &reaction.user_id.unwrap()).await {
                if !member.user.bot {
                    let channel_id = reaction.channel_id; // Obtiene el ChannelId desde la reacción.
                    match reaction.emoji {
                        ReactionType::Unicode(ref emoji) if emoji == "🔨" => {
                            // Ahora pasamos el ChannelId directamente.
                            self.send_compile_example(&ctx, channel_id).await;
                        },
                        ReactionType::Unicode(ref emoji) if emoji == "📚" => {
                            self.handle_example_command(&ctx, &reaction).await;
                        },
                        _ => {} // Maneja otros emojis si es necesario
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        // Llama a la función para enviar el mensaje de bienvenida.
        self.send_welcome_message(&ctx).await;
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
