use clap::{Parser, Subcommand};

pub mod server;

use backend::server::run_server;
use backend::Language;
use backend::{create_conversation, AppError};

#[derive(Parser)]
#[command(version)]
#[command(about = "Kamkai bckend")]
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
        Some(Commands::Server { port, host }) => {
            run_server(host, port)
                .await
                .map_err(|e| AppError::Server(format!("Error on server: {:#?}", e)))?;
            println!("Claude's response:\n{}", response);
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
