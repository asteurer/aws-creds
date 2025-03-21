use crate::utils::{check_config_path, parse_creds, write_creds};

pub fn rename_profile(
    old_profile: &str,
    new_profile: &str,
    config_path: &Option<String>
) -> Result<(), anyhow::Error> {
    let path = check_config_path(config_path)?;
    let mut all_creds = parse_creds(&path)?;

    for p in all_creds.profiles.iter_mut() {
        if p.profile_name == old_profile {
            p.profile_name = new_profile.to_string();

            if old_profile == all_creds.default {
                all_creds.default = new_profile.to_owned();
            }

            write_creds(&all_creds, &path)?;

            return Ok(println!("Profile `{}` renamed to `{}`", old_profile, new_profile));
        }
    }

    Err(anyhow::anyhow!("profile `{}` doesn't exist", old_profile))
}