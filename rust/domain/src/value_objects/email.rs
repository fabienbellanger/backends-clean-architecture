//! Email value object representation

use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::validation::validate_request_data;
use std::fmt::{Display, Formatter};
use validator::Validate;

#[derive(Debug, Default, Clone, PartialEq, Eq, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    /// Create and validate a new email
    ///
    /// # Example
    /// ```rust
    /// use fake::Fake;
    /// use fake::faker::internet::fr_fr::FreeEmail;
    /// use pos_async_api_domain::value_objects::email::Email;
    ///
    /// let valid_email: String = FreeEmail().fake();
    /// let email = Email::new(&valid_email).unwrap();
    /// assert_eq!(email.value(), valid_email.to_owned());
    ///
    /// // `Email` implements the `Display` trait
    /// println!("{email}");
    ///
    /// assert!(Email::new("").is_err());
    /// assert!(Email::new("invalid_email").is_err());
    /// ```
    pub fn new(value: &str) -> ApiResult<Self> {
        let email = Self {
            value: value.to_string(),
        };

        validate_request_data(&email)?;

        Ok(email)
    }

    /// Get email value
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
