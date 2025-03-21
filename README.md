# Overview

This is a command line tool that makes it easier to retrieve and use AWS temporary credentials for multiple accounts.

### Features
- Generates and stores temporary AWS credentials and their expiration for multiple profiles
- Prompts the user to generate new credentials when tokens have expired
- Allows the user to export a profile's credentials as environment variables (i.e. `AWS_ACCESS_KEY_ID=youraccesskeyid`)

# Install

### Download the release

You can download the binary from the [releases page](https://github.com/asteurer/aws-creds/releases).

To set up your environment like mine, feel free to run the commands below:

```bash
# Download the compressed file
wget https://github.com/asteurer/aws-creds/releases/download/<VERSION>/aws-creds-linux-amd64-<RELEASE>.tar.gz

# Uncompress the file
tar -xvf aws-creds-linux-amd64-<RELEASE>.tar.gz

# Move the uncompressed file to a folder in your PATH
sudo mv aws-creds /usr/local/bin
```

### Build from source

Make sure that you have [Cargo](https://www.rust-lang.org/tools/install) installed.

Clone the repository and navigate to the root of the repository. Once there, you can run `cargo build --release`, and find the binary at `target/release/aws-creds`.

# Examples

### Using the temporary credentials

If needed, the `export` commands below can be substituted with `eval`.

```bash
# With a profile and config path specified
export $(aws-creds show --profile prod --config ./creds.json)

# With defaults
export $(aws-creds show)

# If you want the shell session to have access
export $(aws-creds show)
aws s3 ls
terraform apply

# If you only want a specific process to have access
(export $(aws-creds show --profile prod --config ./creds.json); aws s3 ls)
```

# Caveats

- This was built and tested for Linux
- This was tested on AWS user profiles with MFA enabled
- This was tested on AWS user profiles with virtual authentication apps as the MFA method

# Design

Once created, the configuration by default will be found at `~/.config/aws-creds/creds.json`. Below is the structure of the file:

```json
{
    "default": "YOUR_DEFAULT_PROFILE_NAME",
    "profiles": [
       {
            "profile_name": "YOUR_PROFILE_NAME",
            "permanent_credentials": {
                "access_key_id": "",
                "secret_access_key": "",
                "mfa_serial_number": "",
                "region": "",
            },
            "temporary_credentials": {
                "access_key_id": "",
                "secret_access_key": "",
                "session_token": "",
                "expiration": "",
            }
        }
    ]
}
```
