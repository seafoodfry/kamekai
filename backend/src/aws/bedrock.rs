use anyhow::{anyhow, Result};
use aws_sdk_bedrockruntime::operation::converse::ConverseOutput;

pub fn get_converse_output_text(output: ConverseOutput) -> Result<String> {
    Ok(output
        .output()
        .ok_or_else(|| anyhow!("Error no output"))?
        .as_message()
        .map_err(|e| anyhow!("Error output not as message: {:#?}", e))?
        .content()
        .first()
        .ok_or_else(|| anyhow!("Error no content in first message"))?
        .as_text()
        .map_err(|e| anyhow!("Error content is not text: {:#?}", e))?
        .to_string())
}
