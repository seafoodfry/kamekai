use anyhow::{Context, Result};
use aws_sdk_bedrockruntime::operation::converse::ConverseOutput;

pub fn get_converse_output_text(output: ConverseOutput) -> Result<String> {
    output
        .output()
        .ok_or_else(|| anyhow!("Error no output"))?
        .as_message()
        .map_err(|| anyhow!("Error output not as message"))?
        .content()
        .first()
        .ok_or_else(|| anyhow!("Error no content in first message"))?
        .as_text()
        .map_err(|| anyhow!("Error content is not text"))?
        .map(|s| s.to_string())
}
