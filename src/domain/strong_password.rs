use secrecy::{Secret, Zeroize};

#[derive(Debug, Clone)]
pub struct StrongPassword(String);

impl StrongPassword {
    pub fn parse(s: String) -> Result<StrongPassword, String> {
        // combine multiple spaces
        let s = s.split_whitespace().collect::<Vec<&str>>().join(" ");

        match s.len() {
            0..=11 => Err("Password is too short.".to_string()),
            12..=128 => Ok(Self(s)),
            _ => Err("Password is too long.".to_string()),
        }
    }
}

impl Zeroize for StrongPassword {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

pub type SecretStrongPassword = Secret<StrongPassword>;

#[cfg(test)]
mod tests {
    use super::StrongPassword;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_64_char_long_password_is_valid() {
        let password = "a".repeat(64);
        assert_ok!(StrongPassword::parse(password));
    }

    #[test]
    fn a_password_longer_than_128_chars_is_rejected() {
        let password = "a".repeat(129);
        assert_err!(StrongPassword::parse(password));
    }

    #[test]
    fn password_containing_non_ascii_alphanumeric_is_allowed() {
        let password = "٣".repeat(25);
        assert_ok!(StrongPassword::parse(password));
    }
}
