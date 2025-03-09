use anyhow::anyhow;
use crate::{
    types::TemporaryCredentials,
    utils::{check_config_path, get_temp_cred_status, parse_creds, parse_profile_name, TempCredStatus},
};

pub fn show_creds(profile_name: &Option<String>, config_path: &Option<String>) -> Result<String, anyhow::Error> {
    let path = check_config_path(config_path)?;
    let all_creds = parse_creds(&path)?;
    let name = parse_profile_name(profile_name, &all_creds.default)?;
    let profile_creds: &TemporaryCredentials = match all_creds.profiles.get(&name) {
        Some(p) => &p.temporary_credentials,
        _ => return Err(anyhow!("profile '{}' not found", name)),
    };

    match get_temp_cred_status(&profile_creds.expiration)? {
        TempCredStatus::Empty => {
                Err(anyhow!("The temporary credentials for profile `{}` haven't yet been retrieved\nPlease run `aws-creds get` to fix", name))
        },
        TempCredStatus::Expired => {
                Err(anyhow!("The temporary credentials for profile `{}` have expired\nPlease run `aws-creds get` to fix", name))
        },
        TempCredStatus::Ok => {
            Ok(
                format!(
                    "AWS_ACCESS_KEY_ID={} AWS_SECRET_ACCESS_KEY={} AWS_SESSION_TOKEN={}",
                    profile_creds.access_key_id,
                    profile_creds.secret_access_key,
                    profile_creds.session_token
                )
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::panic;
    use std::{fs, env::current_dir};

    #[test]
    fn test_show_creds() {
        // Intentionally omitted tests for default path
        let dir = current_dir().expect("failed to retrieve working directory").join("test_assets").join("creds.json");
        let dir_str = dir.to_str().expect("Failed to parse path to creds file as string").to_string();
        let config_path: Option<String> = Some(dir_str.to_owned());
        fs::exists(dir_str).unwrap();
        let correct_response =
            "AWS_ACCESS_KEY_ID=test1_temp_access_key_id AWS_SECRET_ACCESS_KEY=test1_temp_secret_access_key AWS_SESSION_TOKEN=test1_temp_session_token";

        // No profile passed
        match show_creds(&None, &config_path) {
            Ok(r) => {
                assert_eq!(r, correct_response);
            },
            Err(e) => panic!("{}", e),
        };

        // Profile passed
        match show_creds(&Some("test1".to_string()), &config_path) {
            Ok(r) => {
                assert_eq!(r, correct_response);
            },
            Err(e) => {
                panic!("{}", e);
            }
        }

        // Non-existent path
        match show_creds(&None, &Some("/i_dont_exist".to_string())) {
            Ok(_) => {
                panic!("This should not have passed");
            },
            Err(e) => {
                assert_eq!("unable to find config file at `/i_dont_exist`\nplease check for errors in the file path, or run `aws-creds new` to create a config file", e.to_string());
            },
        };
    }

}