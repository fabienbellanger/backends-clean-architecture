//! User entity

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct User {
    pub id: Uuid,
    pub lastname: String,
    pub firstname: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Return user fullname
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
    ///     username: "john.doe@test.com".to_owned(),
    ///     password: "1234567890".to_owned(),
    ///     created_at: Utc::now(),
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
