//! User responses module

use crate::entities::user::User;

#[derive(Debug, PartialEq, Eq)]
pub struct LoginResponse {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub username: String,
    pub token: String,
    pub created_at: String,
}

impl From<User> for LoginResponse {
    /// Convert a `User` into `LoginResponse`
    /// ```
    /// use clean_architecture_domain::entities::user::User;
    /// use clean_architecture_domain::ports::responses::user::LoginResponse;
    /// use chrono::{DateTime, Utc};
    /// use uuid::Uuid;
    ///
    /// let user_id = Uuid::new_v4();
    /// let now = Utc::now();
    /// let user = User {
    ///     id: user_id,
    ///     lastname: "Doe".to_owned(),
    ///     firstname: "John".to_owned(),
    ///     username: "john.doe@test.com".to_owned(),
    ///     password: "1234567890".to_owned(),
    ///     created_at: now,
    /// };
    ///
    /// let response = LoginResponse {
    ///     id: user_id.to_string(),
    ///     lastname: "Doe".to_owned(),
    ///     firstname: "John".to_owned(),
    ///     username: "john.doe@test.com".to_owned(),
    ///     token: "token".to_owned(),
    ///     created_at: now.to_rfc3339(),
    /// };
    ///
    /// assert_eq!(response, user.into());
    /// ```
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            lastname: user.lastname,
            firstname: user.firstname,
            username: user.username,
            token: String::from("token"),
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GetUserResponse {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub username: String,
    pub created_at: String,
}

impl From<User> for GetUserResponse {
    /// Convert a `User` into `GetUserResponse`
    /// ```
    /// use clean_architecture_domain::entities::user::User;
    /// use clean_architecture_domain::ports::responses::user::GetUserResponse;
    /// use chrono::{DateTime, Utc};
    /// use uuid::Uuid;
    ///
    /// let user_id = Uuid::new_v4();
    /// let now = Utc::now();
    /// let user = User {
    ///     id: user_id,
    ///     lastname: "Doe".to_owned(),
    ///     firstname: "John".to_owned(),
    ///     username: "john.doe@test.com".to_owned(),
    ///     password: "1234567890".to_owned(),
    ///     created_at: now,
    /// };
    ///
    /// let response = GetUserResponse {
    ///     id: user_id.to_string(),
    ///     lastname: "Doe".to_owned(),
    ///     firstname: "John".to_owned(),
    ///     username: "john.doe@test.com".to_owned(),
    ///     created_at: now.to_rfc3339(),
    /// };
    ///
    /// assert_eq!(response, user.into());
    /// ```
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            lastname: user.lastname,
            firstname: user.firstname,
            username: user.username,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GetUsersResponse {
    pub users: Vec<GetUserResponse>,
}

impl From<Vec<User>> for GetUsersResponse {
    /// Convert a `Vec<User>` into `GetUsersResponse`
    /// ```
    /// use clean_architecture_domain::entities::user::User;
    /// use clean_architecture_domain::ports::responses::user::{GetUserResponse, GetUsersResponse};
    /// use chrono::{DateTime, Utc};
    /// use uuid::Uuid;
    ///
    /// let user_id_1 = Uuid::new_v4();
    /// let user_id_2 = Uuid::new_v4();
    /// let now = Utc::now();
    /// let users: Vec<User> = vec![
    ///     User {
    ///         id: user_id_1,
    ///         lastname: "Doe".to_owned(),
    ///         firstname: "John".to_owned(),
    ///         username: "john.doe@test.com".to_owned(),
    ///         password: "1234567890".to_owned(),
    ///         created_at: now,
    ///     },
    ///     User {
    ///         id: user_id_2,
    ///         lastname: "Doe1".to_owned(),
    ///         firstname: "John".to_owned(),
    ///         username: "john.doe.1@test.com".to_owned(),
    ///         password: "1234567899".to_owned(),
    ///         created_at: now,
    ///     }
    /// ];
    ///
    /// let users_response: Vec<GetUserResponse> = vec![
    ///     GetUserResponse {
    ///         id: user_id_1.to_string(),
    ///         lastname: "Doe".to_owned(),
    ///         firstname: "John".to_owned(),
    ///         username: "john.doe@test.com".to_owned(),
    ///         created_at: now.to_rfc3339(),
    ///     },
    ///     GetUserResponse {
    ///         id: user_id_2.to_string(),
    ///         lastname: "Doe1".to_owned(),
    ///         firstname: "John".to_owned(),
    ///         username: "john.doe.1@test.com".to_owned(),
    ///         created_at: now.to_rfc3339(),
    ///     },
    /// ];
    /// let response = GetUsersResponse { users: users_response };
    ///
    /// assert_eq!(response, users.into());
    /// ```
    fn from(users: Vec<User>) -> Self {
        let users_res = users.into_iter().map(|user| user.into()).collect();
        Self { users: users_res }
    }
}
