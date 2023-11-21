// Importaciones necesarias
use serenity::{
    async_trait,
    model::{ // Este bloque importa las estructuras necesarias del modelo de datos de serenity
        prelude::*,
        channel::{Message, ReactionType, Reaction},
        gateway::{Ready, GatewayIntents},
        id::{ChannelId, MessageId, RoleId}, // Asegúrate de que RoleId esté importado una sola vez
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

use std::{env, process::Command, sync::Arc};
use tempfile::NamedTempFile;
use std::collections::HashMap;
use serenity::prelude::TypeMapKey; // Importación para TypeMapKey
use dotenv::dotenv;

// Estructura para manejar la información de los roles.
struct RoleManager {
    roles: HashMap<String, RoleId>,
}



impl RoleManager {
    async fn initialize_roles(&mut self) {
        self.roles.insert("🤖".to_string(), RoleId(1168603086235902012)); // Suponiendo que este es el ID para el rol "Android"
        self.roles.insert("🍏".to_string(), RoleId(1169202492953858048)); // Suponiendo que este es el ID para el rol "iOS"
        self.roles.insert("🌐".to_string(), RoleId(1168603506891034684)); // Suponiendo que este es el ID para el rol "Web"
        self.roles.insert("⚙️".to_string(), RoleId(1168603873238323220)); // Suponiendo que este es el ID para el rol "DevOps"
        self.roles.insert("📱".to_string(), RoleId(1168604780277547128)); // Suponiendo que este es el ID para el rol "Kotlin"
        self.roles.insert("🐍".to_string(), RoleId(1168605004312084612)); // Suponiendo que este es el ID para el rol "Python"
        self.roles.insert("☕".to_string(), RoleId(1168605218754281504)); // Suponiendo que este es el ID para el rol "Java"
        self.roles.insert("🟨".to_string(), RoleId(1168605273204723733)); // Suponiendo que este es el ID para el rol "JavaScript"
        self.roles.insert("🐘".to_string(), RoleId(1168605349402648697)); // Suponiendo que este es el ID para el rol "PHP"
        self.roles.insert("<:rust:4504>".to_string(), RoleId(1168605349402648697)); // Suponiendo que este es el ID para el rol "RUST"
        // Repite el proceso para los roles adicionales que has proporcionado
        self.roles.insert("☁️".to_string(), RoleId(889900112233445566)); // Suponiendo que este es el ID para el rol "AWS"
        self.roles.insert("☁️".to_string(), RoleId(990011223344556677)); // Suponiendo que este es el ID para el rol "Google Cloud"
        self.roles.insert("🔙".to_string(), RoleId(991122334455667788)); // Suponiendo que este es el ID para el rol "Backend"
        self.roles.insert("🔚".to_string(), RoleId(992233445566778899)); // Suponiendo que este es el ID para el rol "Frontend"
        // Para "Trainee Player", "Junior Player", etc., podrías usar emojis de medallas o trofeos:
        self.roles.insert("🔰".to_string(), RoleId(993344556677889900)); // Suponiendo que este es el ID para el rol "Trainee Player"
        self.roles.insert("🥉".to_string(), RoleId(994455667788990011)); // Suponiendo que este es el ID para el rol "Junior Player"
        self.roles.insert("🥈".to_string(), RoleId(995566778899001122)); // Suponiendo que este es el ID para el rol "Mid Player"
        self.roles.insert("🥇".to_string(), RoleId(996677889900112233)); // Suponiendo que este es el ID para el rol "Senior Player"
        self.roles.insert("🏆".to_string(), RoleId(997788990011223344)); // Suponiendo que este es el ID para el rol "Expert Player"
    }
}

// Implementación de TypeMapKey para que RoleManager pueda ser almacenado en el TypeMap de serenity.
impl TypeMapKey for RoleManager {
    type Value = Arc<RwLock<RoleManager>>;
}

// Estructura Handler que ahora incluye un campo para el RoleManager.
struct Handler {
    role_manager: Arc<RwLock<RoleManager>>,
}

impl Handler {

         // Esta función se encargará de enviar el mensaje de bienvenida.
         async fn send_welcome_message(&self, ctx: &Context) {
            let channel_id = ChannelId(1123024565178744962); // Reemplaza con el ID de tu canal real.
            let data = ctx.data.read().await;
            let role_manager = data.get::<RoleManager>().expect("Expected RoleManager in TypeMap.").clone();
            let role_manager = role_manager.read().await;
        
            // Envía el mensaje de bienvenida y crea el embed
            let send_result = channel_id.send_message(&ctx.http, |m| {
                m.content("¡Hola! Soy un bot que compila código Rust. Aquí puedes probar tus códigos:")
                .embed(|e| {
                    e.title("Funcionalidades del Bot Rust")
                    .description("Este bot puede compilar y ejecutar tu código Rust. Usa el comando `!compile` seguido de tu código en un bloque de código para probarlo.")
                    .field("¿Cómo usar este bot?", "A continuación, te explico cómo puedes interactuar conmigo:", false)
                    .field("Compilar Código", "Reacciona con 🔨 y te enviaré una plantilla de código que puedes compilar.", false)
                    .field("Obtener un Ejemplo", "Reacciona con 📚 y te proporcionaré un ejemplo de código Rust.", false)
                    .field("Ejemplo de Compilación", "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```", false)
                    .colour(Colour::from_rgb(0, 255, 0))
                })
            }).await;
    
            // Verifica si el mensaje se envió correctamente
            if let Ok(message) = send_result {
                // Añade reacciones para cada rol al mensaje
                for emoji in role_manager.roles.keys() {
                    let _ = message.react(&ctx.http, ReactionType::Unicode(emoji.to_string())).await;
                }
            } else {
                eprintln!("Error al enviar el mensaje de bienvenida.");
            }
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
        if let Some(guild_id) = reaction.guild_id {
            // Ignorar reacciones del propio bot
            if reaction.user_id == Some(ctx.cache.current_user_id().await) {
                return;
            }

            // Verificar si la reacción es en el mensaje correcto
            if let Ok(channel) = reaction.channel_id.to_channel(&ctx).await {
                if let Some(message) = channel.guild().and_then(|c| c.message(&ctx.http, reaction.message_id).ok()).await {
                    // Aquí es donde comprobamos si es el mensaje de bienvenida por el contenido o autor
                    // Si es el caso, procedemos con la asignación del rol
                    let data = ctx.data.read().await;
                    let role_manager_lock = data.get::<RoleManager>().expect("Expected RoleManager in TypeMap.");
                    let role_manager = role_manager_lock.read().await;
                    if let Some(role_id) = role_manager.roles.get(&reaction.emoji.to_string()) {
                        // Asigna el rol al usuario que reaccionó
                        let _ = guild_id.member(&ctx.http, &reaction.user_id.unwrap()).await.unwrap().add_role(&ctx.http, role_id).await;

                        // Envía un mensaje directo al usuario confirmando la asignación del rol
                        if let Ok(user) = reaction.user(&ctx).await {
                            let dm = user.create_dm_channel(&ctx).await;
                            if let Ok(dm) = dm {
                                let _ = dm.say(&ctx.http, format!("Se te ha asignado el rol correspondiente al emoji {}.", reaction.emoji)).await;
                            }
                        }
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

    // Inicialización de RoleManager y adición al contexto compartido
    let role_manager = Arc::new(RwLock::new(RoleManager { roles: HashMap::new() }));
    role_manager.write().await.initialize_roles().await; // Aquí inicializamos los roles

    // Crea un nuevo Handler con el RoleManager
    let handler = Handler {
        role_manager: role_manager.clone(),
    };

    // Crea el cliente de Discord con nuestro Handler
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .framework(StandardFramework::new().configure(|c| c.prefix("!")))
        .await
        .expect("Error creating client");

    // Inserta el RoleManager en el TypeMap del cliente
    {
        let mut data = client.data.write().await;
        data.insert::<RoleManager>(role_manager);
    }

    // Inicia el cliente de Discord
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

}
