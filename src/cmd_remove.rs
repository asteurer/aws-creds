use crate::utils::{check_config_path, parse_creds, write_creds};

pub fn remove_profile(profile_name: &String, config_path: &Option<String>) -> Result<(), anyhow::Error> {
    let path = check_config_path(config_path)?;
    let mut all_creds= parse_creds(&path)?;

    match all_creds.profiles.remove(profile_name){
        Some(_) => Ok(write_creds(&all_creds, &path)?),
        None => Err(anyhow::anyhow!("profile `{profile_name}` doesn't exist")),
    }
}