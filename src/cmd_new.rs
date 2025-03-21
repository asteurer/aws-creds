use std::{fs, path::Path, process};
use anyhow::{anyhow, Error};
use inquire::{self, validator::Validation};
use crate::{
    types::{AwsProfiles, CredentialsProfile, FileError, PermanentCredentials, TemporaryCredentials},
    utils::{get_default_config_path, parse_creds, write_creds},
};

pub fn create_profile(profile_name: &String, config_path: &Option<String>) -> Result<(), Error> {
    // Ensures that the inquire::<String> methods below won't accept a blank string
    let string_validator = |input: &str| {
        if input.trim().is_empty() {
            Ok(Validation::Invalid("field cannot be empty".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let default = inquire::Confirm::new(&format!("Would you like to set profile `{}` as default?", profile_name))
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

    let path = create_config_file_if_not_exists(config_path)?;
    let mut creds = match parse_creds(&path) {
        Ok(c) => {
            // If the profile already exists...
            for p in c.profiles.iter() {
                if &p.profile_name == profile_name {
                    let overwrite_response = inquire::Confirm::new(&format!("WARNING: profile `{}` already exists. Would you like to overwrite?", profile_name))
                    .with_default(false)
                    .prompt()
                    .map_err(|_| anyhow!("failed to parse overwrite_response"))?;

                    // If the user chooses not to overwrite, exit the process
                    if !overwrite_response {
                        process::exit(0);
                    }
                }
            }

            c
        },
        Err(e) => {
            match e {
                FileError::NotFound => {
                    let creds = AwsProfiles{
                        default: "".to_string(),
                        profiles: Vec::new(),
                    };

                    creds
                },
                FileError::Other { message } => return Err(anyhow!("{}", message)),
            }
        }
    };

    // If the user wants the new profile to be set as default...
    if default {
        creds.default = profile_name.to_owned();
    }

    creds.profiles.push(CredentialsProfile {
        profile_name: profile_name.to_owned(),
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

     write_creds(&creds, &path)?;

    Ok(println!("Profile `{}` created at `{}`", profile_name, path))
}

fn create_config_file_if_not_exists(config_path: &Option<String>) -> Result<String, Error> {
    let path_str = match config_path{
        Some(p) => p,
        None => &get_default_config_path()?,
    };

    let raw_path = Path::new(&path_str);

    // Check to make sure the file ends with creds.json
    let file = raw_path.file_name().expect("failed to parse file name");
    let file_str = file.to_str().expect("failed to parse file name as string");
    if !file_str.contains(".json") {
        return Err(anyhow!("the file `{}` is invalid (missing the `.json` extension)", file_str))
    }

    // Create the directory
    let dirs = raw_path.parent().expect("failed to parse parent path as directory");

    fs::create_dir_all(dirs)?;
    Ok(path_str.to_string())
}