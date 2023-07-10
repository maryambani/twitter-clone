use nutype::nutype;

use crate::UserFacingError;

#[nutype(validate(min_len = 3, max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Username(String);

impl UserFacingError for UsernameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            UsernameError::TooLong => "Username is too long. Must be at most 30 characters.",
            &UsernameError::TooShort => "Username is too short. Must be at least 3 characters.",
        }
    }
}

#[nutype(validate(min_len = 8))]
#[derive(AsRef, Clone, Serialize, Deserialize, PartialEq)]
pub struct Password(String);

impl UserFacingError for PasswordError {
    fn formatted_error(&self) -> &'static str {
        match self {
            &PasswordError::TooShort => "Password is too short. Must be at least 8 characters.",
        }
    }
}