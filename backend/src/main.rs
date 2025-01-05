use clap::{Parser, Subcommand};

use backend::server::run_server;
use backend::Language;
use backend::{create_conversation, AppError};

#[derive(Parser)]
#[command(version)]
#[command(about = "Kamkai backend")]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Lesson {
        #[arg(short, long, value_enum)]
        language: Language,
    },
    Server {
        #[arg(short, long, default_value = "8080")]
        port: u16,
        #[arg(long, default_value = "0.0.0.0")]
        host: String,

        #[arg(long, env = "APP_ENABLE_ANSI_LOGS", default_value = "true")]
        enable_ansi: bool,

        #[arg(long, short, env = "APP_REQ_TIMEOUT", default_value_t = 60)]
        request_timeout: u64,

        #[arg(long, short, env = "APP_USER_POOL")]
        cognito_user_pool: String,
    },
}

async fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Some(Commands::Lesson { language }) => {
            let response = create_conversation(language)
                .await
                .map_err(|e| AppError::Bedrock(format!("Failed to call bedrock: {:#?}", e)))?;
            println!("Claude's response:\n{}", response);
        }
        Some(Commands::Server {
            port,
            host,
            enable_ansi,
            request_timeout,
            cognito_user_pool,
        }) => {
            run_server(host, port, enable_ansi, request_timeout, cognito_user_pool)
                .await
                .map_err(|e| AppError::Server(format!("Error on server: {:#?}", e)))?;
        }
        None => {
            println!("No subcommand provided. Run with the -h flag to see usage.");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Doing it this way so that the error messages are properly formated.
    if let Err(e) = run(cli).await {
        eprintln!("Error: {}", e);
    }
}
