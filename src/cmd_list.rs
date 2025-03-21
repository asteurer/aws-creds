use crate::utils::{check_config_path, get_temp_cred_status, parse_creds, TempCredStatus};

pub fn list_profiles(config_path: &Option<String>) -> Result<(), anyhow::Error>{
    let path = check_config_path(config_path)?;
    let all_creds = parse_creds(&path)?;
    let mut output: Vec<String> = Vec::new();

    for profile in all_creds.profiles {
        let name = profile.profile_name;
        let status = match get_temp_cred_status(
            &profile.temporary_credentials.expiration
        )? {
            TempCredStatus::Empty => "<- empty",
            TempCredStatus::Expired => "<- expired",
            TempCredStatus::Ok => "",
        };

        if name == all_creds.default {
            output.push(format!("{} {} {}", name, "<- default".to_string(), &status))
        } else {
            output.push(format!("{} {}", name, &status));
        }
    }

    // Sort alphabetically
    output.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    Ok(println!("{}", output.join("\n")))
}