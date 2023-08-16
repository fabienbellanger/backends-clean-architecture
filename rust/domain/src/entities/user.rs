//! User entity

use chrono::{DateTime, Utc};

pub type UserId = uuid::Uuid;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    /// Return user full name
    ///
    /// ```
    /// use clean_architecture_domain::entities::user::User;
    /// use chrono::{DateTime, Utc};
    /// use uuid::Uuid;
    ///
    /// let mut user = User {
    ///     id: Uuid::new_v4(),
    ///     lastname: "Doe".to_owned(),
    ///     firstname: "John".to_owned(),
    ///     email: "john.doe@test.com".to_owned(),
    ///     password: "1234567890".to_owned(),
    ///     created_at: Utc::now(),
    ///     updated_at: Utc::now(),
    ///     deleted_at: None,
    /// };
    ///
    /// assert_eq!(user.fullname(), "John Doe".to_owned());
    ///
    /// user.lastname = "".to_owned();
    /// assert_eq!(user.fullname(), "John".to_owned());
    ///
    /// user.firstname = "".to_owned();
    /// assert_eq!(user.fullname(), "".to_owned());
    /// ```
    pub fn fullname(&self) -> String {
        format!("{} {}", self.firstname, self.lastname).trim().to_string()
    }
}
