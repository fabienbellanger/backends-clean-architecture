//! Forgotten password email

use super::Message;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};

#[derive(Debug, Clone)]
pub struct ForgottenPassword {
    pub app_name: String,
    pub base_url: String,
    pub token: String,
    pub email_from: String,
    pub email_to: String,
}

impl ForgottenPassword {
    /// Return link and subject
    fn get_link_subject(&self) -> ApiResult<(String, String)> {
        let link = format!("{}/{}", self.base_url, self.token);
        let subject = match validator::validate_url(&link) {
            true => format!("{} - Forgotten password", self.app_name),
            false => Err(api_error!(
                ApiErrorCode::InternalError,
                "cannot send password reset email because: invalid link"
            ))?,
        };

        Ok((link, subject))
    }

    /// Construct HTML and TEXT body
    fn construct_bodies(&self, link: String) -> ApiResult<(String, String)> {
        Ok((self.construct_html_body(&link)?, self.construct_text_body(&link)?))
    }

    /// Construct TEXT body
    fn construct_text_body(&self, link: &str) -> ApiResult<String> {
        Ok(format!(
            r#"Forgotten password
==================

You told us you forgot your password. If you really did, click here to choose a new one:

{link}

If you didn't mean to reset your password, then you can just ignore this email; your password will not change."#
        ))
    }

    /// Construct HTML body
    fn construct_html_body(&self, link: &str) -> ApiResult<String> {
        Ok(format!(
            r#"
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body style="margin: 16px; color: #212121; font-size: 13px; font-weight: 400">
  <h1 style="font-size: 24px; font-weight: 600">Forgotten password</h1>
  <section>
    <p>You told us you forgot your password. If you really did, click here to choose a new one:</p>
    <a href="{link}"
      style="display: inline-block; background-color: #1976D2; color: white; padding: 16px 24px; text-decoration: none; margin: 16px; text-align: center; font-size: 16px">
      Choose a new password
    </a>
    <p>
      If you didn't mean to reset your password, then you can just ignore this email; your password will not change.
    </p>
  </section>
</body>
</html>"#
        ))
    }
}

impl TryInto<Message> for ForgottenPassword {
    type Error = ApiError;

    fn try_into(self) -> Result<Message, Self::Error> {
        let (link, subject) = self.get_link_subject()?;
        let (html, text) = self.construct_bodies(link)?;

        Ok(Message {
            from: self.email_from,
            to_list: vec![self.email_to],
            subject,
            text_body: Some(text),
            html_body: Some(html),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_forgotten_password_request_get_link_and_subject() {
        let request = ForgottenPassword {
            app_name: "My App".to_owned(),
            base_url: "https://test.com".to_owned(),
            token: "myToken5846".to_owned(),
            email_from: "from@test.com".to_owned(),
            email_to: "to@test.com".to_owned(),
        };

        let expected = (
            "https://test.com/myToken5846".to_owned(),
            "My App - Forgotten password".to_owned(),
        );
        assert_eq!(request.get_link_subject(), Ok(expected));
    }

    #[test]
    fn test_forgotten_password_request_get_link_and_subject_with_invalid_url() {
        let request = ForgottenPassword {
            app_name: "My App".to_owned(),
            base_url: "-test.com".to_owned(),
            token: "myToken5846".to_owned(),
            email_from: "from@test.com".to_owned(),
            email_to: "to@test.com".to_owned(),
        };
        assert!(request.get_link_subject().is_err());
    }

    #[test]
    fn test_forgotten_password_request_try_into_message() {
        let request = ForgottenPassword {
            app_name: "My App".to_owned(),
            base_url: "https://test.com".to_owned(),
            token: "myToken5846".to_owned(),
            email_from: "from@test.com".to_owned(),
            email_to: "to@test.com".to_owned(),
        };
        let msg: Message = request.try_into().unwrap();

        assert_eq!(msg.subject, "My App - Forgotten password".to_owned());
        assert_eq!(msg.from, "from@test.com".to_owned());
        assert_eq!(msg.to_list, vec!["to@test.com".to_owned()]);
        assert!(msg.text_body.is_some());
        assert!(msg.html_body.is_some());
    }
}
