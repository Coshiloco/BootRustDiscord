use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Client,
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    },
    http::Http,
    model::gateway::GatewayIntents,
};
use std::process::Command;
use tempfile::NamedTempFile;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!compile") {
            let code = &msg.content["!compile".len()..].trim(); // Extraer el código del mensaje y recortar espacios en blanco

            let mut file = NamedTempFile::new().unwrap();
            std::fs::write(file.path(), code).unwrap(); // Escribir el código en un archivo temporal

            let crate_name = "user_code"; // Define un nombre de crate válido
            let output = Command::new("rustc")
                .arg(file.path())
                .arg("--out-dir")
                .arg(file.path().parent().unwrap())
                .arg("--crate-name")
                .arg(crate_name) // Asigna el nombre de crate
                .output()
                .unwrap();

            // Enviar la salida del compilador al canal
            if output.status.success() {
                let executable = file.path().parent().unwrap().join(crate_name);
                let execution_output = Command::new(executable)
                    .output()
                    .unwrap();

                if execution_output.status.success() {
                    let message = format!("Output:\n```\n{}\n```", 
                        String::from_utf8_lossy(&execution_output.stdout));
                    let _ = msg.channel_id.say(&ctx.http, message).await;
                } else {
                    let message = format!("Execution Error:\n```\n{}\n```", 
                        String::from_utf8_lossy(&execution_output.stderr));
                    let _ = msg.channel_id.say(&ctx.http, message).await;
                }
            } else {
                let message = format!("Compilation Error:\n```\n{}\n```", 
                    String::from_utf8_lossy(&output.stderr));
                let _ = msg.channel_id.say(&ctx.http, message).await;
            }
        }
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "MTE3NTI1MTI1ODI0NDg1Nzg4Ng.GeS9Lm.P8Iw9CEy9QUPAjrMeQFbHXqJmzoP0p96ZB9L3Y"; // Asegúrate de usar un token seguro

    // Define los GatewayIntents de tu bot
    let intents = GatewayIntents::all();

    // Construye el cliente de Discord con los intents especificados
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(StandardFramework::new().configure(|c| c.prefix("!"))) // Configura el prefijo del comando aquí
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
