use std::{collections::HashMap, fs, path::Path, process};
use anyhow::{anyhow, Error};
use inquire::{self, validator::Validation};
use crate::{
    types::{AwsProfiles, CredentialsProfile, FileError, PermanentCredentials, TemporaryCredentials},
    utils::{check_config_path, get_default_config_path, parse_creds, write_creds},
};

pub fn create_profile(profile_name: &String, config_path: &Option<String>) -> Result<(), Error> {
    // If the path exists, return it; otherwise, create it
    let path = match check_config_path(config_path) {
        Ok(p) => p,
        Err(e) => {
            match e {
                FileError::NotFound => {
                    create_config_file()?
                },
                FileError::Other { message } => {
                    return Err(anyhow!("{}", message));
                }
            }
        }
    };

    let mut creds = match parse_creds(&path) {
        Ok(c) => {
            // If the profile already exists...
            if c.profiles.get(profile_name).is_some() {
                let overwrite_response = inquire::Confirm::new(&format!("WARNING: Profile `{}` already exists. Would you like to overwrite?", profile_name))
                .with_default(false)
                .prompt()
                .map_err(|_| anyhow!("failed to parse overwrite_response"))?;

                // If the user chooses not to overwrite, exit the process
                if !overwrite_response {
                    process::exit(0);
                }
            }

            c
        },
        Err(e) => {
            match e {
                FileError::NotFound => {
                    let creds = AwsProfiles{
                        default: "".to_string(),
                        profiles: HashMap::new()
                    };

                    creds
                },
                FileError::Other { message } => return Err(anyhow!("{}", message)),
            }
        }
    };

    // Ensures that the inquire::<String> methods below won't accept a blank string
    let string_validator = |input: &str| {
        if input.trim().is_empty() {
            Ok(Validation::Invalid("Name cannot be empty".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let default = inquire::Confirm::new(&format!("Would you like to set profile `{profile_name}` as default?"))
        .with_default(true)
        .prompt()
        .map_err(|_| anyhow!("failed to get user confirmation for default"))?;

    let access_key_id = inquire::Password::new("AWS_ACCESS_KEY_ID:")
        .without_confirmation()
        .with_validator(string_validator)
        .prompt()
        .map_err(|_| anyhow!("failed to get user input for access_key_id"))?;

    let secret_access_key = inquire::Password::new("AWS_SECRET_ACCESS_KEY:")
        .without_confirmation()
        .with_validator(string_validator)
        .prompt()
        .map_err(|_| anyhow!("failed to get user input for secret_access_key"))?;

    let mfa_serial_number = inquire::Text::new("AWS MFA Device Serial Number:")
        .with_validator(string_validator)
        .prompt()
        .map_err(|_| anyhow!("failed to get user input for mfa_serial_number"))?;

    let region = inquire::Text::new("AWS Region:")
        .with_default("us-east-1")
        .with_validator(string_validator)
        .prompt()
        .map_err(|_| anyhow!("failed to get user input for region"))?;

    // If the user wants the new profile to be set as default...
    if default {
        creds.default = profile_name.to_owned();
    }

    creds.profiles.insert(profile_name.to_owned(), CredentialsProfile {
        permanent_credentials: PermanentCredentials {
            access_key_id,
            secret_access_key,
            mfa_serial_number,
            region,
        },
        temporary_credentials: TemporaryCredentials {
            access_key_id: "".to_string(),
            secret_access_key: "".to_string(),
            session_token: "".to_string(),
            expiration: "".to_string(),
        }
    });


    Ok(write_creds(&creds, &path)?)
}

fn create_config_file() -> Result<String, Error> {
    let path_str = get_default_config_path()?;
    let raw_path = Path::new(&path_str);

    // Check to make sure the file ends with creds.json
    let file = raw_path.file_name().expect("DEFAULT_CONFIG_PATH doesn't have a valid file name");
    if file != "creds.json" {
        panic!("DEFAULT_CONFIG_PATH is invalid");
    }

    // Create the directory
    let dirs = match raw_path.parent() {
        Some(v) => v,
        None => panic!("DEFAULT_CONFIG_PATH isn't working"),
    };

    fs::create_dir_all(dirs)?;
    Ok(path_str)
}