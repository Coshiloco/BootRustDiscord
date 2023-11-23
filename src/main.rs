// Importaciones actualizadas
use serenity::{
    async_trait,
    model::{
        channel::{Message, ReactionType, Reaction},
        gateway::{Ready, GatewayIntents},
        id::ChannelId,
        application::{
            interaction::InteractionResponseType,
            component::ButtonStyle,
        },
        event::ResumedEvent,
        prelude::Interaction,
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
use serenity::model::id::RoleId;
use serenity::builder::CreateActionRow;
use std::collections::HashMap;
use dotenv::dotenv;


struct RoleManager {
    roles: HashMap<String, RoleId>,
}

impl RoleManager {
    fn new() -> Self {
        let mut manager = RoleManager {
            roles: HashMap::new(),
        };
        manager.initialize_roles();
        manager
    }

    fn initialize_roles(&mut self) {
        // Insertar cada rol y su respectivo ID
        self.roles.insert("🤖".to_string(), RoleId(1168603086235902012)); // Android
        self.roles.insert("🍏".to_string(), RoleId(1169202492953858048)); // iOS
        self.roles.insert("🌐".to_string(), RoleId(1168603506891034684)); // Web
        self.roles.insert("⚙️".to_string(), RoleId(1168603873238323220)); // DevOps
        self.roles.insert("📱".to_string(), RoleId(1168604780277547128)); // Kotlin
        self.roles.insert("🐍".to_string(), RoleId(1168605004312084612)); // Python
        self.roles.insert("☕".to_string(), RoleId(1168605218754281504)); // Java
        self.roles.insert("🟨".to_string(), RoleId(1168605273204723733)); // JavaScript
        self.roles.insert("🐘".to_string(), RoleId(1168605349402648697)); // PHP
        self.roles.insert("🦀".to_string(), RoleId(1168605349402648697)); // Rust
        // Agrega más roles si es necesario
        self.roles.insert("☁️".to_string(), RoleId(889900112233445566)); // AWS
        self.roles.insert("🔙".to_string(), RoleId(991122334455667788)); // Backend
        self.roles.insert("🔚".to_string(), RoleId(992233445566778899)); // Frontend
        self.roles.insert("🔰".to_string(), RoleId(993344556677889900)); // Trainee Player
        self.roles.insert("🥉".to_string(), RoleId(994455667788990011)); // Junior Player
        self.roles.insert("🥈".to_string(), RoleId(995566778899001122)); // Mid Player
        self.roles.insert("🥇".to_string(), RoleId(996677889900112233)); // Senior Player
        self.roles.insert("🏆".to_string(), RoleId(997788990011223344)); // Expert Player
    }
}


struct RoleButton {
    custom_id: String,
    label: String,
}

impl RoleButton {
    fn new(custom_id: &str, label: &str) -> Self {
        RoleButton { 
            custom_id: custom_id.to_string(), 
            label: label.to_string() 
        }
    }
}




struct Handler {
    role_manager: RoleManager,
}

impl Handler {

         // Esta función se encargará de enviar el mensaje de bienvenida.
         async fn send_welcome_message(&self, ctx: &Context) {
            let channel_id = ChannelId(1022397353736548385); // Reemplaza con el ID de tu canal real.
            let role_buttons = create_role_buttons();// Llamamos a la función para obtener los botones de roles

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
  // Creamos los botones aquí
  // Creamos los botones aquí
  m.components(|c| {
    let buttons_per_row = 5;
    let mut current_row_buttons = 0;
    let mut action_rows: Vec<CreateActionRow> = Vec::new();

    for button in role_buttons.iter() {
        if current_row_buttons == buttons_per_row {
            // Reiniciar el contador para la nueva fila
            current_row_buttons = 0;
        }

        if current_row_buttons == 0 {
            // Comenzar una nueva fila si la actual está llena o es la primera vez
            action_rows.push(CreateActionRow::default());
        }

        if let Some(last_row) = action_rows.last_mut() {
            last_row.create_button(|b| {
                b.custom_id(&button.custom_id)
                    .label(&button.label)
                    .style(ButtonStyle::Primary)
            });
            current_row_buttons += 1; // Incrementar el contador de botones para la fila actual
        }
    }

    // Añadir todas las filas de acción al componente
    for row in action_rows {
        c.add_action_row(row);
    }
    
    c
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
        let file = NamedTempFile::new()?;
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
    
     // Agregamos un nuevo método para manejar las interacciones de los botones
     async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::MessageComponent(mc) = interaction {
            if mc.data.custom_id.starts_with("role_") {
                let response = format!("Botón {} presionado", mc.data.custom_id);
                if let Err(why) = mc.create_interaction_response(&ctx.http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource) // Actualizado
                     .interaction_response_data(|m| m.content(response))
                }).await {
                    println!("Error al enviar respuesta de interacción: {:?}", why);
                }
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
        self.send_welcome_message(&ctx).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::all();

    let role_manager = RoleManager::new();

    let handler = Handler {
        role_manager,
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler) // Usar la instancia 'handler' en lugar de la estructura 'Handler'
        .framework(StandardFramework::new().configure(|c| c.prefix("!")))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


fn create_role_buttons() -> Vec<RoleButton> {
    vec![
        RoleButton::new("role_android", "🤖 Android"),
        RoleButton::new("role_ios", "🍏 iOS"),
        RoleButton::new("role_web", "🌐 Web"),
        RoleButton::new("role_devops", "⚙️ DevOps"),
        RoleButton::new("role_kotlin", "📱 Kotlin"),
        RoleButton::new("role_python", "🐍 Python"),
        RoleButton::new("role_java", "☕ Java"),
        RoleButton::new("role_javascript", "🟨 JavaScript"),
        RoleButton::new("role_php", "🐘 PHP"),
        RoleButton::new("role_rust", "🦀 Rust"),
        RoleButton::new("role_aws", "☁️ AWS"),
        RoleButton::new("role_google_cloud", "☁️ Google Cloud"),
        RoleButton::new("role_backend", "🔙 Backend"),
        RoleButton::new("role_frontend", "🔚 Frontend"),
        RoleButton::new("role_trainee", "🔰 Trainee Player"),
        RoleButton::new("role_junior", "🥉 Junior Player"),
        RoleButton::new("role_mid", "🟠 Mid Player"),
        RoleButton::new("role_senior", "🔵 Senior Player"),
        RoleButton::new("role_expert", "🏆 Expert Player"),
    ]
}
