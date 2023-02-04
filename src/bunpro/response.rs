use serde::Deserialize;
use time::OffsetDateTime;

/// The base response from all Bunpro.jp requests
///
///  These include the user_information as well as the requested information.
#[derive(Debug, Deserialize)]
pub struct BunproResponse<T> {
    pub user_information: UserInformation,
    pub requested_information: T,
}

/// The information on the user returned as part of all valid replies
#[derive(Debug, Deserialize)]
pub struct UserInformation {
    pub username: String,
    pub grammar_point_count: u32,
    pub ghost_review_count: u32,
    #[serde(with = "time::serde::timestamp")]
    pub creation_date: OffsetDateTime,
}
