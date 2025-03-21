use aws_config::Region;
use aws_sdk_sts::{config::Credentials, Config};
use inquire::{self, validator::Validation};
use anyhow::{Error, anyhow};
use crate::{
    types::{AwsProfiles, PermanentCredentials, TemporaryCredentials},
    utils::{check_config_path, parse_creds, parse_profile_name, write_creds},
};

#[::tokio::main]
pub async fn get_new_creds(profile_name: &Option<String>, config_path: &Option<String>) -> Result<(), Error> {

    let path = check_config_path(config_path)?;
    let mut all_creds = parse_creds(&path)?;
    let name = parse_profile_name(profile_name, &all_creds.default)?;

    // Copy perm_creds to appease the borrow checker gods
    let perm_creds = {
        let profile =  match all_creds.profiles.iter().find(|p| p.profile_name == name) {
            Some(p) => p,
            None => return Err(anyhow::anyhow!("profile `{}` doesn't exist", name)),
        };

            profile.permanent_credentials.clone()
    };

    let session_token= inquire::Text::new("MFA Code:")
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Ok(Validation::Invalid("field cannot be empty".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .map_err(|_| anyhow!("failed to retrieve MFA Code"))?;

    let sts_client = create_sts_client(&perm_creds)?;
    let temp_creds = get_temporary_credentials(&sts_client, &perm_creds, &session_token).await?;

    update_credentials(&mut all_creds, temp_creds, name);
    Ok(write_creds(&all_creds, &path)?)
}

fn create_sts_client(perm_creds: &PermanentCredentials) -> Result<aws_sdk_sts::Client, Error> {
     let creds = Credentials::new(
        &perm_creds.access_key_id,
        &perm_creds.secret_access_key,
        None,
        None,
         ""
    );

     let conf = Config::builder()
         .region(Region::new(perm_creds.region.to_owned()))
         .credentials_provider(creds)
         .behavior_version_latest()
         .build();

  Ok(aws_sdk_sts::Client::from_conf(conf))
}

async fn get_temporary_credentials(
    sts_client: &aws_sdk_sts::Client,
    perm_creds: &PermanentCredentials,
    session_token: &str,
) -> Result<aws_sdk_sts::types::Credentials, Error> {
    let token_result = sts_client
    .get_session_token()
    .serial_number(&perm_creds.mfa_serial_number)
    .token_code(session_token)
    .send()
    .await
    .map_err(|e| anyhow!("failed to get session token: {}", e.to_string()))?;

    token_result
        .credentials()
        .ok_or_else(|| anyhow!("no credentials returned in response"))
        .cloned()
}

fn update_credentials(
    all_creds: &mut AwsProfiles,
    aws_creds: aws_sdk_sts::types::Credentials,
    profile_name: String,

) {
    let temporary_credentials = TemporaryCredentials {
        access_key_id: aws_creds.access_key_id().to_string(),
        secret_access_key: aws_creds.secret_access_key.to_string(),
        session_token: aws_creds.session_token().to_string(),
        expiration: aws_creds.expiration().to_string(),
    };

    for p in all_creds.profiles.iter_mut() {
        if p.profile_name == profile_name {
            p.temporary_credentials = temporary_credentials.to_owned();
        }
    }
}