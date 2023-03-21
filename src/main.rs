use log::{debug, error, info};
use rand::{seq::SliceRandom, thread_rng};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env, fs, path::PathBuf, process::exit};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "speak_bot",
    about = "Run a Discord bot that responds to mentions"
)]
struct Opt {
    #[structopt(short, long, help = "Show more logging")]
    debug: bool,

    #[structopt(help = "Path to the text file with quotes")]
    quotes_files: PathBuf,

    #[structopt(short = "t", long, help = "Discord token; pass via arg or env var")]
    discord_token: Option<String>,
}

struct Handler {
    quotes: Vec<String>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.mentions_me(&ctx).await {
            Ok(true) => {
                let quote = match self.quotes.choose(&mut thread_rng()) {
                    Some(q) => q,
                    None => {
                        error!("Somehow, could not choose a quote");
                        return;
                    }
                };
                if let Err(e) = msg.channel_id.say(&ctx.http, quote).await {
                    error!("Could not send quote: {}", e);
                }
            }
            Err(e) => {
                error!("Could not get message mentions: {}", e);
            }
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    if env::var("RUST_LOG").is_err() {
        let log_level = if opt.debug { "debug" } else { "info" };
        env::set_var("RUST_LOG", format!("speak_bot={}", log_level));
    }
    pretty_env_logger::init();
    debug!("Setting up");

    let token = match opt.discord_token {
        Some(t) => t,
        None => match env::var("DISCORD_TOKEN") {
            Ok(t) => t,
            Err(_) => {
                error!("Could not find a Discord token passed via '-t <token>' or via the 'DISCORD_TOKEN' environment variable");
                exit(1);
            }
        },
    };

    if !opt.quotes_files.exists() {
        error!("Quotes file does not exist");
        exit(1);
    }
    let quotes: Vec<String> = match fs::read_to_string(&opt.quotes_files) {
        Ok(s) => s
            .split('\n')
            .filter(|&s| !s.is_empty())
            .map(str::to_owned)
            .collect(),
        Err(e) => {
            error!("Could not load from quotes file: {}", e);
            exit(1);
        }
    };
    debug!("Loaded {} quotes", quotes.len());
    if quotes.is_empty() {
        error!("Quotes file was loaded, but is empty");
        exit(1);
    }

    let mut client = match Client::builder(&token, GatewayIntents::default())
        .event_handler(Handler { quotes })
        .await
    {
        Ok(c) => c,
        Err(e) => {
            error!("Error creating client: {}", e);
            exit(1);
        }
    };

    if let Err(why) = client.start().await {
        error!("Error connecting to Discord: {}", why);
    }
}
