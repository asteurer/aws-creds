use crate::utils::{check_config_path, parse_creds, write_creds};

pub fn set_default(profile_name: &String, config_path: &Option<String>) -> Result<(), anyhow::Error> {
    let path = check_config_path(config_path)?;
    let mut creds = parse_creds(&path)?;

    match creds.profiles.iter().find(|p| &p.profile_name == profile_name) {
        Some(_) => creds.default = profile_name.to_owned(),
        None => return Err(anyhow::anyhow!("profile `{}` doesn't exist", profile_name)),
    }

    write_creds(&creds, &path)?;

    Ok(())
}