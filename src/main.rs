//Importaciones actualizadas
use serenity::{
    async_trait,
    framework::standard::StandardFramework,
    model::{
        application::component::ButtonStyle,
        channel::{Message, Reaction, ReactionType},
        gateway::{GatewayIntents, Ready},
        id::ChannelId,
        prelude::Interaction,
    },
    prelude::*,
    utils::Colour,
};

use dotenv::dotenv;
use serenity::builder::CreateActionRow;
use serenity::model::id::RoleId;
use serenity::model::prelude::ChannelType;
use std::collections::HashMap;
use serenity::model::prelude::PermissionOverwrite;
use serenity::model::Permissions;
use serenity::model::prelude::PermissionOverwriteType;
use std::{env, process::Command};
use serenity::model::prelude::GuildId;
use std::sync::Arc;
use tempfile::NamedTempFile;

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
        self.roles
            .insert("role_android".to_string(), RoleId(1168603086235902012)); // Android
        self.roles
            .insert("role_ios".to_string(), RoleId(1169202492953858048)); // iOS
        self.roles
            .insert("role_web".to_string(), RoleId(1168603506891034684)); // Web
        self.roles
            .insert("role_devops".to_string(), RoleId(1168603873238323220)); // DevOps
        self.roles
            .insert("role_kotlin".to_string(), RoleId(1168604780277547128)); // Kotlin
        self.roles
            .insert("role_python".to_string(), RoleId(1168605004312084612)); // Python
        self.roles
            .insert("role_java".to_string(), RoleId(1168605218754281504)); // Java
        self.roles
            .insert("role_javascript".to_string(), RoleId(1168605273204723733)); // JavaScript
        self.roles
            .insert("role_php".to_string(), RoleId(1168605349402648697)); // PHP
        self.roles
            .insert("role_rust".to_string(), RoleId(1168605383288422550)); // Rust
                                                                           // Agrega más roles sirole_androides necesario
        self.roles
            .insert("role_aws".to_string(), RoleId(1168605473818284062)); // AWS
        self.roles
            .insert("role_google_cloud".to_string(), RoleId(1168605507074928670)); // Backend
        self.roles
            .insert("role_backend".to_string(), RoleId(1168605571289710764)); // Frontend
        self.roles
            .insert("role_frontend".to_string(), RoleId(1168605654529871882)); // Trainee Player
        self.roles
            .insert("role_trainee".to_string(), RoleId(1168605956633022476)); // Junior Player
        self.roles
            .insert("role_junior".to_string(), RoleId(1168606003764404294)); // Mid Player
        self.roles
            .insert("role_mid".to_string(), RoleId(1168606283381870652)); // Senior Player
        self.roles
            .insert("role_senior".to_string(), RoleId(1168606105459507230)); // Expert Player
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
            label: label.to_string(),
        }
    }
}

// Definición de la estructura Handler ...
struct Handler {
    role_manager: RoleManager,
    // Utiliza Arc<RwLock<>> para permitir el acceso seguro entre múltiples tareas asincrónicas
    stats_category_id: Arc<RwLock<Option<ChannelId>>>,
    all_members_channel_id: Arc<RwLock<Option<ChannelId>>>,
    members_channel_id: Arc<RwLock<Option<ChannelId>>>,
    bots_channel_id: Arc<RwLock<Option<ChannelId>>>,
}

impl Handler {
    // Esta función se encargará de enviar el mensaje de bienvenida.
    async fn send_welcome_message(&self, ctx: &Context) {
        let channel_id = ChannelId(1123024565178744962); // Reemplaza con el ID de tu canal real.
        let role_buttons = create_role_buttons(); // Llamamos a la función para obtener los botones de roles

        let _ = channel_id.send_message(&ctx.http, |m| {
                m.content("¡Hola! Soy un bot que compila código Rust. Aquí puedes probar tus códigos:");
                m.embed(|e| {
                    e.title("Funcionalidades del Bot Rust")
                     .description("Este bot puede compilar y ejecutar tu código Rust. Usa el comando `!compile` seguido de tu código en un bloque de código para probarlo.")
                     .field("¿Cómo usar este bot?", "A continuación, te explico cómo puedes interactuar conmigo:", false)
                     .field("Compilar Código", "Reacciona con 🔨 y te enviaré una plantilla de código que puedes compilar.", false)
                     .field("Obtener un Ejemplo", "Reacciona con 📚 y te proporcionaré un ejemplo de código Rust.", false)
                     .field("Ejemplo de Compilación", "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```", false)
                     .field("Selecciona un boton para asignarte un rol", "Si le das al boton de los roles que tienen su icono y su nombre correspondiente que tienes disponibles de las tecnoologias se te asignara en el servidor", false)
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
    
    async fn initialize_or_update_stats_channels(&self, ctx: &Context, guild_id: GuildId) -> Result<(), serenity::Error> {
        // Primero, tratamos de leer si ya tenemos un category_id almacenado.
        let category_id = {
            let read_guard = self.stats_category_id.read().await;
            if let Some(id) = *read_guard {
                id
            } else {
                // Si no hay un category_id almacenado, creamos la categoría y actualizamos el RwLock.
                drop(read_guard); // Suelta el guard antes de realizar operaciones de bloqueo.
                let category = guild_id.create_channel(&ctx.http, |c| {
                    c.name("SERVER STATS").kind(ChannelType::Category)
                }).await?;
                let mut write_guard = self.stats_category_id.write().await;
                *write_guard = Some(category.id);
                category.id
            }
        };

        // Ahora que tenemos el category_id, podemos proceder a obtener o crear los canales.
        let all_members_channel_id = self.get_or_create_stats_channel(ctx, guild_id, category_id, "All Members").await?;
        let members_channel_id = self.get_or_create_stats_channel(ctx, guild_id, category_id, "Members").await?;
        let bots_channel_id = self.get_or_create_stats_channel(ctx, guild_id, category_id, "Bots").await?;

        // Actualizamos los RwLocks con los IDs de los canales.
        {
            let mut write_guard = self.all_members_channel_id.write().await;
            *write_guard = Some(all_members_channel_id);
        }
        {
            let mut write_guard = self.members_channel_id.write().await;
            *write_guard = Some(members_channel_id);
        }
        {
            let mut write_guard = self.bots_channel_id.write().await;
            *write_guard = Some(bots_channel_id);
        }

        Ok(())
    }
    
    async fn get_or_create_stats_channel(&self, ctx: &Context, guild_id: GuildId, category_id: ChannelId, channel_name: &str) -> Result<ChannelId, serenity::Error> {
        // Definir los permisos de sobreescritura para el canal de voz.
        let overwrites = vec![
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL,
                deny: Permissions::CONNECT, // Los usuarios no pueden unirse a los canales de voz de estadísticas.
                kind: PermissionOverwriteType::Role(guild_id.0.into()), // Aplica a todos los miembros.
            },
        ];
    
        // Buscar el canal por nombre dentro de la categoría especificada.
        let channels = guild_id.channels(&ctx.http).await?;
        if let Some((id, _)) = channels.iter().find(|(_, c)| c.name.starts_with(channel_name) && c.kind == ChannelType::Voice && c.parent_id == Some(category_id)) {
            Ok(*id)
        } else {
            // Si no se encuentra, crear el canal de voz con los permisos definidos.
            let channel = guild_id.create_channel(&ctx.http, |c| {
                c.name(format!("{}: 0", channel_name)) // Inicialmente, ponemos un contador a 0.
                 .kind(ChannelType::Voice)
                 .category(category_id)
                 .permissions(overwrites)
            }).await?;
            Ok(channel.id)
        }
    }
    
    // Método para iniciar el bucle de actualización de estadísticas.
    async fn start_stats_update_loop(
        ctx: Arc<Context>, 
        guild_id: GuildId,
        all_members_channel_id: Arc<RwLock<Option<ChannelId>>>,
        members_channel_id: Arc<RwLock<Option<ChannelId>>>,
        bots_channel_id: Arc<RwLock<Option<ChannelId>>>
    ) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(600));
        loop {
            interval.tick().await;
            if let Err(e) = update_stats_channels(
                &ctx, 
                guild_id, 
                &all_members_channel_id, 
                &members_channel_id, 
                &bots_channel_id
            ).await {
                println!("Error updating stats channels: {:?}", e);
            }
        }
    }
    
    
    
    // Asegúrate de actualizar la firma de send_compile_example para aceptar ChannelId directamente.
    async fn send_compile_example(&self, ctx: &Context, channel_id: ChannelId) {
        let example_code = "¡Aquí tienes un ejemplo de código Rust que puedes compilar!\n\nEscribe en el chat el siguiente comando y código:\n\n!compile \\`\\`\\`rust\nfn main() {\n    println!(\"Esto es un ejemplo que compila\");\n}\n\\`\\`\\`";
        let _ = channel_id
            .say(&ctx.http, example_code)
            .await
            .expect("Error sending message");
    }

    async fn handle_example_command(&self, ctx: &Context, reaction: &Reaction) {
        // Simplemente envía un ejemplo de código fijo al canal donde se reaccionó
        let channel_id = reaction.channel_id;
        let example_code = r#"```rust
fn main() {
    println!("Hello, World!");
}
```"#;
        let _ = channel_id
            .say(&ctx.http, example_code)
            .await
            .expect("Error sending message");
    }

    fn extract_rust_code(content: &str) -> Option<String> {
        let start_pattern = "```rust";
        let end_pattern = "```";
        if let Some(start) = content.find(start_pattern) {
            if let Some(end) = content[start + start_pattern.len()..].find(end_pattern) {
                let code =
                    &content[start + start_pattern.len()..start + start_pattern.len() + end].trim();
                return Some(code.to_string());
            }
        }
        None
    }

    async fn compile_and_execute_rust_code(
        ctx: &Context,
        msg: &Message,
        code: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
            let error_message = format!(
                "Compilation error:\n```\n{}\n```",
                String::from_utf8_lossy(&output.stderr)
            );
            msg.channel_id.say(&ctx.http, &error_message).await?;
            return Ok(());
        }

        let executable_path = file
            .path()
            .parent()
            .unwrap()
            .join(if cfg!(target_os = "windows") {
                "user_code.exe"
            } else {
                "user_code"
            });
        let execution_output = Command::new(&executable_path).output()?;

        if !execution_output.status.success() {
            let error_message = format!(
                "Execution error:\n```\n{}\n```",
                String::from_utf8_lossy(&execution_output.stderr)
            );
            msg.channel_id.say(&ctx.http, &error_message).await?;
            return Ok(());
        }

        let output_message = format!(
            "Output:\n```\n{}\n```",
            String::from_utf8_lossy(&execution_output.stdout)
        );
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
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "Please provide Rust code inside a code block.")
                    .await;
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
                        if let Some(role_id) = self
                            .role_manager
                            .find_role_id_by_custom_id(&mc.data.custom_id)
                        {
                            if let Err(why) = member.add_role(&ctx.http, role_id).await {
                                println!("Error al asignar rol: {:?}", why);
                            } else {
                                // Formar el mensaje de respuesta
                                let response = if let Ok(user) = user_id.to_user(&ctx.http).await {
                                    let username = format!("{}#{}", user.name, user.discriminator);
                                    let guild_name = guild_id
                                        .to_guild_cached(&ctx.cache)
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
                                    if let Some((channel_id, _)) =
                                        channels.iter().find(|(_id, channel)| {
                                            channel.name == "role-assignment-log"
                                        })
                                    {
                                        if let Err(why) = channel_id.say(&ctx.http, &response).await
                                        {
                                            println!("Error al enviar mensaje al canal role-assignment-log: {:?}", why);
                                        }
                                    } else {
                                        // Canal no encontrado, intenta crearlo
                                        println!("Canal role-assignment-log no encontrado, intentando crearlo...");
                                        match guild_id
                                            .create_channel(&ctx.http, |c| {
                                                c.name("role-assignment-log")
                                                    .kind(ChannelType::Text)
                                            })
                                            .await
                                        {
                                            Ok(channel) => {
                                                println!("Canal creado con éxito.");
                                                if let Err(why) =
                                                    channel.say(&ctx.http, &response).await
                                                {
                                                    println!("Error al enviar mensaje al nuevo canal role-assignment-log: {:?}", why);
                                                }
                                            }
                                            Err(why) => println!(
                                                "Error al crear el canal role-assignment-log: {:?}",
                                                why
                                            ),
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
                        }
                        ReactionType::Unicode(ref emoji) if emoji == "📚" => {
                            self.handle_example_command(&ctx, &reaction).await;
                        }
                        _ => {} // Maneja otros emojis si es necesario
                    }
                }
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // ID del servidor donde el bot está operando.
        let guild_id = GuildId(1117873488334700615); // Reemplaza con el ID de tu servidor.

        // Inicializar o actualizar canales de estadísticas.
        if let Err(e) = self.initialize_or_update_stats_channels(&ctx, guild_id).await {
            println!("Error initializing or updating stats channels: {:?}", e);
        }

        // Envío del mensaje de bienvenida y otras inicializaciones.
        self.send_welcome_message(&ctx).await;

        // Iniciar el bucle para actualizar las estadísticas periódicamente.
        let ctx_arc = Arc::new(ctx);
        let all_members_channel_id = self.all_members_channel_id.clone();
        let members_channel_id = self.members_channel_id.clone();
        let bots_channel_id = self.bots_channel_id.clone();

        tokio::spawn(async move {
            Handler::start_stats_update_loop(
                ctx_arc, 
                guild_id, 
                all_members_channel_id,
                members_channel_id,
                bots_channel_id
            ).await;
        });
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::all();

    let role_manager = RoleManager::new();

    // Inicializa los campos de `Handler` con valores predeterminados.
    let handler = Handler { 
        role_manager,
        stats_category_id: Arc::new(RwLock::new(None)),
        all_members_channel_id: Arc::new(RwLock::new(None)),
        members_channel_id: Arc::new(RwLock::new(None)),
        bots_channel_id: Arc::new(RwLock::new(None)),
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

async fn update_stats_channels(
    ctx: &Context,
    guild_id: GuildId,
    all_members_channel_id: &Arc<RwLock<Option<ChannelId>>>,
    members_channel_id: &Arc<RwLock<Option<ChannelId>>>,
    bots_channel_id: &Arc<RwLock<Option<ChannelId>>>
) -> Result<(), serenity::Error> {
    let guild = match guild_id.to_guild_cached(&ctx.cache) {
        Some(guild) => guild,
        None => return Err(serenity::Error::Other("Guild not found in cache")),
    };

    let all_members = guild.member_count;

    // Obtener la lista de bots con un límite de 1000 (o el límite que desees).
    let bots = guild.members(&ctx.http, Some(1000), None).await?
        .iter()
        .filter(|member| member.user.bot)
        .count();
    let members = all_members - bots as u64;

    // Actualizar nombres de canales.
    if let Some(id) = *all_members_channel_id.read().await {
        id.edit(&ctx.http, |c| c.name(format!("All Members: {}", all_members))).await?;
    }
    if let Some(id) = *members_channel_id.read().await {
        id.edit(&ctx.http, |c| c.name(format!("Members: {}", members))).await?;
    }
    if let Some(id) = *bots_channel_id.read().await {
        id.edit(&ctx.http, |c| c.name(format!("Bots: {}", bots))).await?;
    }

    Ok(())
}


