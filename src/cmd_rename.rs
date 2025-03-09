use crate::utils::{check_config_path, parse_creds, write_creds};

pub fn rename_profile(
    old_profile: &str,
    new_profile: &str,
    config_path: &Option<String>
) -> Result<(), anyhow::Error> {
    let path = check_config_path(config_path)?;
    let mut all_creds = parse_creds(&path)?;
    let profile_creds = all_creds
    .profiles
    .get(old_profile)
    .ok_or_else(|| anyhow::anyhow!("profile not found"))?
    .clone();

    // If the profile was the default, update the default profile name
    if all_creds.default == old_profile {
        all_creds.default = new_profile.to_owned();
    }

    all_creds
        .profiles
        .remove(old_profile)
        .expect("Failed to remove old_profile");

    all_creds.profiles.insert(new_profile.to_string(), profile_creds);

    Ok(write_creds(&all_creds, &path)?)
}