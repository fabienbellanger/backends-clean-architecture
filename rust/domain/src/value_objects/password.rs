//! Password value object representation

use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::validation::validate_request_data;
use std::fmt::{Display, Formatter};
use validator::Validate;

#[derive(Debug, Default, Clone, PartialEq, Eq, Validate)]
pub struct Password {
    #[validate(length(min = 4))]
    value: String,
}

impl Password {
    /// Create and validate a new password
    ///
    /// # Example
    /// ```rust
    /// use fake::Fake;
    /// use fake::faker::internet::fr_fr::Password as FakePassword;
    /// use clean_architecture_domain::value_objects::password::Password;
    ///
    /// let valid_password: String = FakePassword(6..9).fake();
    /// let password = Password::new(&valid_password).unwrap();
    /// assert_eq!(password.value(), valid_password);
    ///
    /// // `Password` implements the `Display` trait
    /// println!("{password}");
    ///
    /// assert!(Password::new("").is_err());
    /// let invalid_password: String = FakePassword(2..3).fake();
    /// assert!(Password::new(&invalid_password).is_err());
    /// ```
    pub fn new(value: &str) -> ApiResult<Self> {
        let password = Self {
            value: value.to_string(),
        };

        validate_request_data(&password)?;

        Ok(password)
    }

    /// Get password value
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
