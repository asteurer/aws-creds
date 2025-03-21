use anyhow::anyhow;
use crate::utils::{check_config_path, get_temp_cred_status, parse_creds, parse_profile_name, TempCredStatus};

pub fn show_creds(profile_name: &Option<String>, config_path: &Option<String>) -> Result<String, anyhow::Error> {
    let path = check_config_path(config_path)?;
    let all_creds = parse_creds(&path)?;
    let name = parse_profile_name(profile_name, &all_creds.default)?;
    let profile_creds = match all_creds.profiles.iter().find(|p| p.profile_name == name) {
        Some(p) => &p.temporary_credentials,
        _ => return Err(anyhow!("profile `{}` doesn't exist", name)),
    };

    match get_temp_cred_status(&profile_creds.expiration)? {
        TempCredStatus::Empty => {
                Err(anyhow!("the temporary credentials for profile `{}` haven't yet been retrieved\nPlease run `aws-creds get` to fix", name))
        },
        TempCredStatus::Expired => {
                Err(anyhow!("the temporary credentials for profile `{}` have expired\nPlease run `aws-creds get` to fix", name))
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

    #[test]
    fn test_show_creds() {
        // Intentionally omitted tests for default path
        let dir = std::env::current_dir().expect("failed to retrieve working directory").join("test_assets").join("creds.json");
        let dir_str = dir.to_str().expect("failed to parse path to creds file as string").to_string();
        if !std::path::Path::exists(&dir) {
            panic!{"file `{}` does not exist", dir_str};
        }

        let config_path: Option<String> = Some(dir_str.to_owned());
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
                assert_eq!("unable to find file: please check the config path and try again", e.to_string());
            },
        };
    }

}