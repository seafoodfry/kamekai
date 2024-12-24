use backend::{create_conversation, AppError};

async fn run() -> Result<(), AppError> {
    let response = create_conversation(
        "Please teach me something nontrivial in japanese and explain the grammar.",
    )
    .await?;
    println!("Claude's response:\n{}", response);
    Ok(())
}

#[tokio::main]
async fn main() {
    // Doing it this way so that the error messages are properly formated.
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}
