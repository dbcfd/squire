pub fn router() -> Router {
    Router::new().route("/v1/favorite/city", post(create_city))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
    #[validate(length(min = 3, max = 16), regex = "USERNAME_REGEX")]
    username: String,
    #[validate(length(min = 8, max = 32))]
    password: String,
}
