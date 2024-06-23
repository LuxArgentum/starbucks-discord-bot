use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // Load env variables from .env
    dotenv::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![birthday()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

struct Person {
    name: String,
    birthday: Birthday,
}

impl Person {
    pub fn new(name: String, birthday: Birthday) -> Person {
        Person {
            name,
            birthday,
        }
    }
}

struct Birthday {
    month: u8,
    day: u8,
    year: u32,
}

impl Birthday {
    pub fn new(month: u8, day: u8, year: u32) -> Birthday {
        Birthday {
            month,
            day,
            year,
        }
    }
}


#[poise::command(slash_command)]
async fn birthday(
    ctx: Context<'_>,
    #[description = "Month"] month: u8,
    #[description = "Day"] day: u8,
    #[description = "Year"] year: u32,
) -> Result<(), Error> {
    let birthday: Birthday = Birthday::new(month, day, year);
    let name: String = ctx.author().to_string();
    let person: Person = Person::new(name, birthday);
    let response = format!("Hey {}! Your birthday is saved as {}/{}/{}!", person.name, person.birthday.month, person.birthday.day, person.birthday.year);
    ctx.reply(response).await?;
    Ok(())
}