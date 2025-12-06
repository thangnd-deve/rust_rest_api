use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct CreateUser {
    #[validate(
        required(message = "Username is required"),
        length(
            min = 1,
            max = 30,
            message = "Username must be between 1 and 30 characters"
        )
    )]
    pub username: Option<String>,

    #[validate(
        required(message = "Email is required"),
        email(message = "Invalid email format")
    )]
    pub email: Option<String>,

    #[validate(
        required(message = "Password is required"),
        length(min = 8, message = "Password must be at least 8 characters long")
    )]
    pub password: Option<String>,
}
