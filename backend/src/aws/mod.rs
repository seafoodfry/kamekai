pub mod bedrock;

use bedrock::get_converse_output_text;

use anyhow::{anyhow, Context, Result};
use aws_config::SdkConfig;
use aws_sdk_bedrockruntime::{types::InferenceConfiguration, types::Message, Client};
use aws_sdk_sts::Client as StsClient;

const AWS_REGION: &str = "us-east-1";
const INFERENCE_PROFILE_TEMPLATE: &str =
    "arn:aws:bedrock:us-east-1:{}:inference-profile/us.anthropic.claude-3-5-sonnet-20241022-v2:0";

async fn get_aws_account_id(config: &SdkConfig) -> Result<String> {
    let sts_client = StsClient::new(config);
    let identity = sts_client
        .get_caller_identity()
        .send()
        .await
        .context("Error calling sts get-caller-identity")?;

    // Return the account ID from the identity.
    identity
        .account()
        .map(|account_id| account_id.to_string())
        .ok_or_else(|| anyhow!("Error STS get-caller-identity did not contain an account ID"))
}

#[derive(Debug)]
pub struct InferenceParameters {
    pub temperature: f32,
    pub max_tokens: i32,
    pub top_p: f32,
}

impl Default for InferenceParameters {
    fn default() -> Self {
        Self {
            temperature: 0.8,
            max_tokens: 1024,
            top_p: 0.95,
        }
    }
}

pub struct AWSClient {
    bedrock_client: Client,
    inference_profile: String,
    inference_parameters: InferenceParameters,
}

impl AWSClient {
    pub async fn new(params: Option<InferenceParameters>) -> Result<Self> {
        let sdk_config = aws_config::defaults(aws_config::BehaviorVersion::v2024_03_28())
            .region(AWS_REGION)
            .load()
            .await;
        let client = aws_sdk_bedrockruntime::Client::new(&sdk_config);

        let aws_account_id = get_aws_account_id(&sdk_config)
            .await
            .context("Error getting AWS account ID")?;
        let aws_inference_profile = format!("{} {}", INFERENCE_PROFILE_TEMPLATE, aws_account_id);

        let inference_params = params.unwrap_or_default();

        Ok(Self {
            bedrock_client: client,
            inference_profile: aws_inference_profile,
            inference_parameters: inference_params,
        })
    }

    pub async fn create_conversation(&self, messages: Vec<Message>) -> Result<String> {
        let inference_config = InferenceConfiguration::builder()
            .temperature(self.inference_parameters.temperature)
            .max_tokens(self.inference_parameters.max_tokens)
            .top_p(self.inference_parameters.top_p)
            .build();

        let response = self
            .bedrock_client
            .converse()
            .model_id(&self.inference_profile)
            .set_messages(Some(messages))
            .set_inference_config(Some(inference_config))
            .send()
            .await
            .context("Error conversing with AWS bedrock")?;

        get_converse_output_text(response)
    }
}
