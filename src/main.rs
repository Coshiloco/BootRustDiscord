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
        prelude::Interaction, guild,
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

use std::{env, process::Command, fmt::format};
use tempfile::NamedTempFile;
use serenity::model::id::RoleId;
use serenity::builder::CreateActionRow;
use serenity::model::prelude::ChannelType;
use std::collections::HashMap;
use dotenv::dotenv;

// Definicion de la estructura RoleManager , que gestiona los roles del servidor 
struct RoleManager {
    roles: HashMap<String, RoleId>,
}

impl RoleManager {
    // Constructor para crear una nueva instancia de RoleManager
    fn new() -> Self {
        let mut manager = RoleManager {
            roles: HashMap::new(),
        };
        manager.initialize_roles();
        manager
    }
    
    // Metodo para inicializar el RoleManager con los roles disponibles
    fn initialize_roles(&mut self) {
        // Insertar cada rol y su respectivo ID
        self.roles.insert("role_android".to_string(), RoleId(1168603086235902012)); // Android
        self.roles.insert("role_ios".to_string(), RoleId(1169202492953858048)); // iOS
        self.roles.insert("role_web".to_string(), RoleId(1168603506891034684)); // Web
        self.roles.insert("role_devops".to_string(), RoleId(1168603873238323220)); // DevOps
        self.roles.insert("role_kotlin".to_string(), RoleId(1168604780277547128)); // Kotlin
        self.roles.insert("role_python".to_string(), RoleId(1168605004312084612)); // Python
        self.roles.insert("role_java".to_string(), RoleId(1168605218754281504)); // Java
        self.roles.insert("role_javascript".to_string(), RoleId(1168605273204723733)); // JavaScript
        self.roles.insert("role_php".to_string(), RoleId(1168605349402648697)); // PHP
        self.roles.insert("role_rust".to_string(), RoleId(1168605383288422550)); // Rust
        // Agrega más roles sirole_androides necesario
        self.roles.insert("role_aws".to_string(), RoleId(1168605473818284062)); // AWS
        self.roles.insert("role_google_cloud".to_string(), RoleId(1168605507074928670)); // Backend
        self.roles.insert("role_backend".to_string(), RoleId(1168605571289710764)); // Frontend
        self.roles.insert("role_frontend".to_string(), RoleId(1168605654529871882)); // Trainee Player
        self.roles.insert("role_trainee".to_string(), RoleId(1168605956633022476)); // Junior Player
        self.roles.insert("role_junior".to_string(), RoleId(1168606003764404294)); // Mid Player
        self.roles.insert("role_mid".to_string(), RoleId(1168606283381870652)); // Senior Player
        self.roles.insert("role_senior".to_string(), RoleId(1168606105459507230)); // Expert Player
    }
    
    // Encontrar el id del usuario
    fn find_role_id_by_custom_id(&self, custom_id: &str) -> Option<&RoleId> {
        self.roles.get(custom_id)
    }
}

// Definición de la estructura RoleButton para representar botones de rol
struct RoleButton {
    custom_id: String,
    label: String,
}

impl RoleButton {
    // Constructor para crear un nuevo RoleButton
    fn new(custom_id: &str, label: &str) -> Self {
        RoleButton { 
            custom_id: custom_id.to_string(), 
            label: label.to_string() 
        }
    }
}



// Definición de la estructura Handler, que manejará los eventos del bot
struct Handler {
    role_manager: RoleManager,
}

impl Handler {

         // Esta función se encargará de enviar el mensaje de bienvenida.
         async fn send_welcome_message(&self, ctx: &Context) {
            let channel_id = ChannelId(1123024565178744962); // Reemplaza con el ID de tu canal real.
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
                    .style(ButtonStyle::Secondary)
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
    
// Dentro de tu EventHandler, agrega el método para manejar interacciones de componente de mensaje.
async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::MessageComponent(mc) = interaction {
        if mc.data.custom_id.starts_with("role_") {
            // Comprueba si la interacción es dentro de un servidor (Guild)
            if let Some(guild_id) = mc.guild_id {
                let user_id = mc.user.id; // UserId directo, no es Option

                // Intentar obtener una instancia mutable del Member
                if let Ok(mut member) = guild_id.member(&ctx.http, user_id).await {
                    // Procesar la asignación del rol
                    if let Some(role_id) = self.role_manager.find_role_id_by_custom_id(&mc.data.custom_id) {
                        if let Err(why) = member.add_role(&ctx.http, role_id).await {
                            println!("Error al asignar rol: {:?}", why);
                        } else {
                            // Formar el mensaje de respuesta
                            let response = if let Ok(user) = user_id.to_user(&ctx.http).await {
                                let username = format!("{}#{}", user.name, user.discriminator);
                                let guild_name = guild_id.to_guild_cached(&ctx.cache)
                                    .map(|g| g.name)
                                    .unwrap_or_else(|| "Desconocido".to_string());
                                format!("Hola {}, he asignado el rol {} correctamente en el servidor {}.", username, mc.data.custom_id, guild_name)
                            } else {
                                "Error al obtener el usuario.".to_string()
                            };

                            // Enviar mensaje directo al usuario
                            if let Ok(dm_channel) = user_id.create_dm_channel(&ctx.http).await {
                                let _ = dm_channel.say(&ctx.http, &response).await;
                            }

                            // Enviar mensaje al canal role-assignment-log
if let Ok(channels) = guild_id.channels(&ctx.http).await {
    if let Some((channel_id, _)) = channels.iter().find(|(_id, channel)| channel.name == "role-assignment-log") {
        if let Err(why) = channel_id.say(&ctx.http, &response).await {
            println!("Error al enviar mensaje al canal role-assignment-log: {:?}", why);
        }
    } else {
        // Canal no encontrado, intenta crearlo
        println!("Canal role-assignment-log no encontrado, intentando crearlo...");
        match guild_id.create_channel(&ctx.http, |c| c.name("role-assignment-log").kind(ChannelType::Text)).await {
            Ok(channel) => {
                println!("Canal creado con éxito.");
                if let Err(why) = channel.say(&ctx.http, &response).await {
                    println!("Error al enviar mensaje al nuevo canal role-assignment-log: {:?}", why);
                }
            }
            Err(why) => println!("Error al crear el canal role-assignment-log: {:?}", why),
        }
    }
} else {
    println!("Error al obtener la lista de canales del servidor.");
}
                        }
                    } else {
                        println!("ID de rol no encontrado para: {}", mc.data.custom_id);
                    }
                } else {
                    println!("No se pudo obtener el miembro del servidor.");
                }
            } else {
                println!("La interacción no ocurrió en un servidor.");
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