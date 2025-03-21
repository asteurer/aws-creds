use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialsProfile {
    pub profile_name: String,
    pub permanent_credentials: PermanentCredentials,
    pub temporary_credentials: TemporaryCredentials,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermanentCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub mfa_serial_number: String,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemporaryCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AwsProfiles{
    pub default: String,
    pub profiles: Vec<CredentialsProfile>,
}

// This custom type makes it possible to handle file not found errors with more precision
#[derive(thiserror::Error, Debug)]
pub enum FileError {
    #[error("unable to find file: please check the config path and try again")]
    NotFound,

    #[error("ERROR: {message}")]
    Other {
        message: String,
    },
}