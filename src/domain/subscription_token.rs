use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct SubscriptionToken(String);

impl SubscriptionToken {
    pub fn parse(s: String) -> Result<SubscriptionToken, String> {
        let is_incorrect_length = s.len() != 25;
        let contains_forbidden_characters = s.chars().all(|char| !char.is_ascii_alphanumeric());

        if is_incorrect_length || contains_forbidden_characters {
            Err(format!("{} is not a valid subscription token.", s))
        } else {
            Ok(Self(s))
        }
    }

    /// Generate a random 25-characters-long case-sensitive subscription token.
    pub fn generate() -> SubscriptionToken {
        let mut rng = thread_rng();
        Self(
            std::iter::repeat_with(|| rng.sample(Alphanumeric))
                .map(char::from)
                .take(25)
                .collect(),
        )
    }
}

impl AsRef<str> for SubscriptionToken {
    // The caller gets a shared reference to the inner string.
    // This gives the caller **read-only** access,
    // they have no way to compromise our invariants!
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriptionToken;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_25_char_long_token_is_valid() {
        let token = "a".repeat(25);
        assert_ok!(SubscriptionToken::parse(token));
    }

    #[test]
    fn a_token_longer_than_25_chars_is_rejected() {
        let token = "a".repeat(26);
        assert_err!(SubscriptionToken::parse(token));
    }

    #[test]
    fn token_containing_non_ascii_alphanumeric_is_rejected() {
        let token = "٣".repeat(25);
        assert_err!(SubscriptionToken::parse(token));
    }

    #[test]
    fn a_valid_token_is_parsed_successfully() {
        let token = "91F5qB9X9HT1UfBjURPkHq4oC".to_string();
        assert_ok!(SubscriptionToken::parse(token));
    }
}
