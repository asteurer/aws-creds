use crate::types::{AwsProfiles, FileError};
use anyhow::{Error, anyhow};
use std::fs;

pub fn get_default_config_path() -> Result<String, Error> {
    let home_dir = dirs::home_dir().expect("Failed to get user's home directory");
    let result = home_dir
        .join(".config")
        .join("aws-creds")
        .join("creds.json");

    Ok(result.to_str().expect("Failed to create path to creds.json").to_string())
}

/// If the config_path is empty, return the default config path; otherwise, verify that the config_path exists.
pub fn check_config_path(config_path: &Option<String>) -> Result<String, FileError> {
      let path = match config_path {
        Some(p) => p,
        _ => {
            match get_default_config_path() {
                Ok(p) => &p.to_owned(),
                Err(e) => return Err(FileError::Other { message: e.to_string() }),
            }
        },
    };

    // Make sure that the file exists
    match fs::exists(path) {
        Ok(exists) => {
            if !exists {
                Err(FileError::NotFound)
            } else {
                Ok(path.to_owned())
            }
        },
        Err(e) => {
            Err(FileError::Other { message:e.to_string() })
        }
    }
}

pub fn parse_creds(config_path: &String) -> Result<AwsProfiles, FileError> {
    // Parse the credentials file
    let result: AwsProfiles = match fs::read_to_string(config_path) {
        Ok(s) => {
            let creds: AwsProfiles = match serde_json::from_str(&s) {
                Ok(v) => v,
                Err(e) => return Err(FileError::Other { message: e.to_string() }),
            };
            creds
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Err(FileError::NotFound)
            }
            return Err(FileError::Other { message: e.to_string() });
        },
    };

    Ok(result)
}

pub fn write_creds(creds: &AwsProfiles, config_path: &String) -> Result<(), Error> {
    let contents = serde_json::to_string(creds)?;
    fs::write(config_path, contents)?;
    Ok(())
}

/// Returns the default profile name if no profile_name is passed; otherwise, makes sure that
/// no one tries to name their profile `default`
pub fn parse_profile_name(profile_name: &Option<String>, default_profile: &str) -> Result<String, Error> {
    match profile_name {
        Some(n) => {
            if n == "default" {
                return Err(anyhow!("`default` is not a valid profile name. Please choose a different profile name or run again without any arguments to use the profile set as default"));
            }

            Ok(n.to_owned())
        },
        _ => Ok(String::from(default_profile)),
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TempCredStatus {
    Empty,
    Expired,
    Ok,
}

/// Validates whether the temporary credentials are empty, ok, or expired
pub fn get_temp_cred_status(expiration_timestamp: &str) -> Result<TempCredStatus, Error> {
    let now = chrono::Utc::now();
    let cred_expiration = match chrono::DateTime::parse_from_rfc3339(expiration_timestamp)
        .map(|dt| dt.with_timezone(&chrono::Utc)) {
            Ok(t) => t,
            Err(e) => {
                // If the timestamp arg == ""...
                if e.kind() == chrono::format::ParseErrorKind::TooShort {
                    return Ok(TempCredStatus::Empty)
                } else {
                    return Err(e.into());
                }
            }
        };

    if cred_expiration < now {
        return Ok(TempCredStatus::Expired);
    } else {
        return Ok(TempCredStatus::Ok);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;

    #[test]
    fn test_get_temp_cred_status () {
        let expired_timestamp = "2025-03-13T10:57:34Z";
        let current_time = chrono::Utc::now() + chrono::Duration::hours(1);
        let current_timestamp = current_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let exp_output = match get_temp_cred_status(expired_timestamp){
            Ok(v) => v,
            Err(e) => panic!("{}", e.to_string()),
        };

        let curr_output = match get_temp_cred_status(&current_timestamp) {
            Ok(v) => v,
            Err(e) => panic!("{}", e.to_string()),
        };

        let empty_output = match get_temp_cred_status("") {
            Ok(v) => v,
            Err(e) => panic!("{}", e.to_string()),
        };

        assert_eq!(exp_output, TempCredStatus::Expired);
        assert_eq!(curr_output, TempCredStatus::Ok);
        assert_eq!(empty_output, TempCredStatus::Empty);
    }

    #[test]
    fn test_parse_profile_name() {
        match parse_profile_name(&None, "test1") {
            Ok(v) => {
                assert_eq!(v, "test1".to_string());
            },
            Err(e) => panic!("{}", e.to_string()),
        };

        match parse_profile_name(&Some("test2".to_string()), "test1") {
            Ok(v) => {
                assert_eq!(v, "test2".to_string());
            },
            Err(e) => panic!("{}", e.to_string()),
        };

        match parse_profile_name(&Some("default".to_string()), "test1") {
            Ok(_) => panic!("ERROR: parse_profile_name allows user create profile named `default`"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "`default` is not a valid profile name. Please choose a different profile name or run again without any arguments to use the profile set as default",
                )
            },
        };
    }

}