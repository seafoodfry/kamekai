use crate::error::AppError;
use aws_config::SdkConfig;
use aws_sdk_sts::Client as StsClient;

pub async fn get_aws_account_id(config: &SdkConfig) -> Result<String, AppError> {
    let sts_client = StsClient::new(config);
    let identity =
        sts_client.get_caller_identity().send().await.map_err(|e| {
            AppError::Sts(format!("Failed to call sts get-caller-identity: {:#?}", e))
        })?;

    // Return the account ID from the identity.
    identity
        .account()
        .map(|account_id| account_id.to_string())
        .ok_or_else(|| {
            AppError::Sts("STS get-caller-identity did not contain an account ID".into())
        })
}
