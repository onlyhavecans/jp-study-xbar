use serde::Deserialize;
use time::OffsetDateTime;

/// The information requested from study_queue endpoint
#[derive(Debug, Deserialize)]
pub struct StudyQueue {
    pub reviews_available: u32,
    #[serde(with = "time::serde::timestamp")]
    pub next_review_date: OffsetDateTime,
    pub reviews_available_next_hour: u32,
    pub reviews_available_next_day: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bunpro::response::BunproResponse;

    #[test]
    fn parse_study_queue() {
        // from https://bunpro.jp/api/user/{USER_API_KEY}/study_queue
        // https://bunpro.jp/api#study-queue
        let reply = std::fs::read_to_string("test/bunpro/study_queue_response.json").unwrap();

        let parsed: BunproResponse<StudyQueue> = serde_json::from_str(&reply).unwrap();

        // User Struct
        assert_eq!("onlyhavecans", parsed.user_information.username);
        assert_eq!(29, parsed.user_information.grammar_point_count);
        assert_eq!(21, parsed.user_information.ghost_review_count);
        let creation = OffsetDateTime::from_unix_timestamp(1609295500).unwrap();
        assert_eq!(creation, parsed.user_information.creation_date);

        // Study queue reply
        assert_eq!(4, parsed.requested_information.reviews_available);
        let next = OffsetDateTime::from_unix_timestamp(1675389600).unwrap();
        assert_eq!(next, parsed.requested_information.next_review_date);
        assert_eq!(7, parsed.requested_information.reviews_available_next_hour);
        assert_eq!(43, parsed.requested_information.reviews_available_next_day);
    }
}
