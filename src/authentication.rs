use rocket::{http::Status, Request, request::{FromRequest, Outcome}};

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "Basic YW5pbWE6aXNnb29k"
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}