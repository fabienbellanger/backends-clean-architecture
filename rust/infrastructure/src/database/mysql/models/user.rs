//! User model

use chrono::{DateTime, NaiveDateTime, Utc};
use clean_architecture_domain::{
    entities::user::User,
    value_objects::{email::Email, password::Password},
};
use clean_architecture_shared::{
    api_error,
    error::{ApiError, ApiErrorCode},
};
use sqlx::FromRow;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, FromRow)]
pub struct UserModel {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            lastname: user.lastname,
            firstname: user.firstname,
            email: user.email.value(),
            password: user.password.value(),
            created_at: user.created_at.naive_utc(),
            updated_at: user.updated_at.naive_utc(),
            deleted_at: user.deleted_at.map(|dt| dt.naive_utc()),
        }
    }
}

impl TryInto<User> for UserModel {
    type Error = ApiError;

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User {
            id: Uuid::from_str(&self.id).map_err(|err| api_error!(ApiErrorCode::InternalError, err))?,
            lastname: self.lastname,
            firstname: self.firstname,
            email: Email::new(&self.email)?,
            password: Password::new(&self.password)?,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(self.created_at, Utc),
            updated_at: DateTime::<Utc>::from_naive_utc_and_offset(self.updated_at, Utc),
            deleted_at: self
                .deleted_at
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clean_architecture_domain::entities::user::User;

    #[test]
    fn test_from_user_entity() {
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let now_naive = now.naive_utc();
        let user = User {
            id: user_id,
            lastname: "Doe".to_owned(),
            firstname: "John".to_owned(),
            email: Email::new("john.doe@test.com").unwrap(),
            password: Password::new("0000000").unwrap(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        let expected = UserModel {
            id: user_id.to_string(),
            lastname: "Doe".to_owned(),
            firstname: "John".to_owned(),
            email: "john.doe@test.com".to_owned(),
            password: "0000000".to_owned(),
            created_at: now_naive,
            updated_at: now_naive,
            deleted_at: None,
        };

        assert_eq!(UserModel::from(user), expected);
    }

    #[test]
    fn test_try_into_user_entity_with_valid_model() {
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let now_naive = now.naive_utc();

        let model = UserModel {
            id: user_id.to_string(),
            lastname: "Doe".to_owned(),
            firstname: "John".to_owned(),
            email: "john.doe@test.com".to_owned(),
            password: "0000000".to_owned(),
            created_at: now_naive,
            updated_at: now_naive,
            deleted_at: None,
        };
        let user = User {
            id: user_id,
            lastname: "Doe".to_owned(),
            firstname: "John".to_owned(),
            email: Email::new("john.doe@test.com").unwrap(),
            password: Password::new("0000000").unwrap(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        let expected: User = model.try_into().unwrap();

        assert_eq!(expected, user);
    }

    #[test]
    fn test_try_into_user_entity_with_invalid_uuid_model() {
        let now = Utc::now();
        let now_naive = now.naive_utc();
        let model = UserModel {
            id: "1234567890".to_string(),
            lastname: "Doe".to_owned(),
            firstname: "John".to_owned(),
            email: "john.doe@test.com".to_owned(),
            password: "0000000".to_owned(),
            created_at: now_naive,
            updated_at: now_naive,
            deleted_at: None,
        };
        let user = TryInto::<User>::try_into(model);
        assert!(user.is_err());
    }
}
