use crate::{types::CredentialsProfile, utils::{check_config_path, parse_creds, write_creds}};

pub fn remove_profile(profile_name: &String, config_path: &Option<String>) -> Result<(), anyhow::Error> {
    let path = check_config_path(config_path)?;
    let mut all_creds= parse_creds(&path)?;
    let mut profiles_list: Vec<CredentialsProfile> = Vec::new();

    for p in all_creds.profiles.iter_mut() {
        if &p.profile_name != profile_name {
            profiles_list.push(p.to_owned());
        }
    }

    all_creds.profiles = profiles_list;

    Ok(write_creds(&all_creds, &path)?)
}