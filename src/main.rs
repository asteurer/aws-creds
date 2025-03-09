use clap::{Parser, Subcommand};
mod cmd_show;
mod cmd_remove;
mod cmd_new;
mod cmd_default;
mod cmd_get;
mod cmd_rename;
mod cmd_list;
mod utils;
mod types;

#[derive(Parser)]
#[command(version, about = "Makes it easy to manage and use temporary AWS credentials")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a new profile")]
    New {
        /// The name of the profile
        profile: String,

        #[arg(short, long, help = "Path to the config file")]
        config: Option<String>,
    },

    #[command(about = "Print temporary credentials formatted as environment variables")]
    Show {
        #[arg(short, long, help = "The name of the AWS profile")]
        profile: Option<String>,

        #[arg(short, long, help = "Path to the config file")]
        config: Option<String>,
    },

    #[command(about = "Retrieve new temporary credentials from AWS")]
    Get {
        #[arg(short, long, help = "The name of the AWS profile")]
        profile: Option<String>,

        #[arg(short, long, help = "Path to the config file")]
        config: Option<String>,
    },

    #[command(about = "Sets a profile as default")]
    Default {
        /// The name of the profile
        profile: String,

       #[arg(short, long, help = "Path to the config file")]
       config: Option<String>,
    },

    #[command(visible_alias = "rm", about = "Deletes a profile")]
    Remove {
        /// The name of the profile
        profile: String,

        #[arg(short, long, help = "Path to the config file")]
        config: Option<String>,
    },

    #[command(visible_alias = "mv", about = "Renames a profile")]
    Rename {
        /// The profile to be renamed
        old_profile: String,

        /// The new name of the profile
        new_profile: String,

        #[arg(short, long, help = "Path to the config file")]
        config: Option<String>,
    },

    #[command(visible_alias = "ls", about = "Print a list of all profile names")]
    #[command(long_about ="Print a list of all profile names. The following are annotations that may be next to a profile name:\n- default: the default profile\n- expired: the temporary credentials for the profile have expired (fix with `aws-creds get`)\n- empty: the temporary credentials for the profile are empty (fix with `aws-creds get`)")]
    List {
        #[arg(short, long, help = "Path to the config file")]
        config: Option<String>,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Show {profile, config}) => {
            let result = cmd_show::show_creds(profile, config)?;
            return Ok(println!("{}", result));
        },
        Some(Commands::Default {profile, config}) => {
            return Ok(cmd_default::set_default(profile, config)?);
        },
        Some(Commands::Remove {profile, config}) => {
            return Ok(cmd_remove::remove_profile(profile, config)?);
        },
        Some(Commands::New {profile, config}) => {
            return Ok(cmd_new::create_profile(profile, config)?);
        },
        Some(Commands::Get { profile, config }) => {
            return Ok(cmd_get::get_new_creds(profile, config)?);
        },
        Some(Commands::Rename { old_profile, new_profile, config }) => {
            return Ok(cmd_rename::rename_profile(old_profile, new_profile, config)?);
        },
        Some(Commands::List { config }) => {
            return Ok(cmd_list::list_profiles(config)?);
        }
        None => {
            eprintln!("ERROR: missing required arguments\nFor a list of options, run `aws-creds --help`");
            std::process::exit(1);
        },
    }
}
