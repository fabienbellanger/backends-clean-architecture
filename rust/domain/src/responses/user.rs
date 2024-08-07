//! User responses

use crate::entities::user::User;
use crate::responses::pagination::PaginateResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub access_token: String,
    pub access_token_expired_at: String,
    pub refresh_token: String,
    pub refresh_token_expired_at: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for GetUserResponse {
    /// Convert a `User` into `GetUserResponse`
    /// ```
    /// use clean_architecture_domain::entities::user::User;
    /// use clean_architecture_domain::responses::user::GetUserResponse;
    /// use clean_architecture_domain::value_objects::email::Email;
    /// use clean_architecture_domain::value_objects::password::Password;
    /// use chrono::{DateTime, Utc};
    /// use uuid::Uuid;
    ///
    /// let user_id = Uuid::new_v4();
    /// let now = Utc::now();
    /// let user = User {
    ///     id: user_id,
    ///     lastname: "Doe".to_owned(),
    ///     firstname: "John".to_owned(),
    ///     email: Email::new("john.doe@test.com").unwrap(),
    ///     password: Password::new("1234567890", false).unwrap(),
    ///     created_at: now,
    ///     updated_at: now,
    ///     deleted_at: None,
    /// };
    ///
    /// let response = GetUserResponse {
    ///     id: user_id.to_string(),
    ///     lastname: "Doe".to_owned(),
    ///     firstname: "John".to_owned(),
    ///     email: "john.doe@test.com".to_owned(),
    ///     created_at: now.to_rfc3339(),
    ///     updated_at: now.to_rfc3339(),
    /// };
    ///
    /// assert_eq!(response, user.into());
    /// ```
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            lastname: user.lastname,
            firstname: user.firstname,
            email: user.email.value(),
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}

pub type GetUsersResponse = PaginateResponse<Vec<GetUserResponse>>;

impl From<(Vec<User>, i64)> for GetUsersResponse {
    /// Convert a `(Vec<User>, i64)` into `GetUsersResponse`
    /// ```
    /// use clean_architecture_domain::entities::user::User;
    /// use clean_architecture_domain::responses::user::{GetUserResponse, GetUsersResponse};
    /// use clean_architecture_domain::responses::pagination::PaginateResponse;
    /// use clean_architecture_domain::value_objects::email::Email;
    /// use clean_architecture_domain::value_objects::password::Password;
    /// use chrono::{DateTime, Utc};
    /// use uuid::Uuid;
    ///
    /// let user_id_1 = Uuid::new_v4();
    /// let user_id_2 = Uuid::new_v4();
    /// let now = Utc::now();
    /// let users: (Vec<User>, i64) =(vec![
    ///     User {
    ///         id: user_id_1,
    ///         lastname: "Doe".to_owned(),
    ///         firstname: "John".to_owned(),
    ///         email: Email::new("john.doe@test.com").unwrap(),
    ///         password: Password::new("1234567890", false).unwrap(),
    ///         created_at: now,
    ///         updated_at: now,
    ///         deleted_at: None,
    ///     },
    ///     User {
    ///         id: user_id_2,
    ///         lastname: "Doe1".to_owned(),
    ///         firstname: "John".to_owned(),
    ///         email: Email::new("john.doe.1@test.com").unwrap(),
    ///         password: Password::new("1234567899", false).unwrap(),
    ///         created_at: now,
    ///         updated_at: now,
    ///         deleted_at: None,
    ///     }
    /// ], 5);
    ///
    /// let users_response: Vec<GetUserResponse> = vec![
    ///     GetUserResponse {
    ///         id: user_id_1.to_string(),
    ///         lastname: "Doe".to_owned(),
    ///         firstname: "John".to_owned(),
    ///         email: "john.doe@test.com".to_owned(),
    ///         created_at: now.to_rfc3339(),
    ///         updated_at: now.to_rfc3339(),
    ///     },
    ///     GetUserResponse {
    ///         id: user_id_2.to_string(),
    ///         lastname: "Doe1".to_owned(),
    ///         firstname: "John".to_owned(),
    ///         email: "john.doe.1@test.com".to_owned(),
    ///         created_at: now.to_rfc3339(),
    ///         updated_at: now.to_rfc3339(),
    ///     },
    /// ];
    /// let response = GetUsersResponse { data: users_response, total: 5 };
    ///
    /// assert_eq!(response, users.into());
    /// ```
    fn from(data: (Vec<User>, i64)) -> Self {
        let users = data.0.into_iter().map(|user| user.into()).collect();
        Self {
            data: users,
            total: data.1,
        }
    }
}
