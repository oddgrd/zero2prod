use super::StrongPassword;
use secrecy::Secret;

pub struct Credentials {
    pub username: String,
    pub password: Secret<StrongPassword>,
}
