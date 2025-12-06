use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
struct create_user {
    #[validate(length(min = 1, max = 30))]
    username: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 8))]
    password: String,
}

fn validate_new_user(data: &create_user) -> Result<(), validator::ValidationErrors> {
    data.validate()
}
