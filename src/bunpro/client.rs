use super::{error::Error, response::BunproResponse, study_queue::StudyQueue};

/// A client for querying the bunproAPI
/// THis requires a 32 character key used to identify the user
pub struct BunproClient {
    base_url: String,
    api_key: String,
}

impl BunproClient {
    pub fn new(api_key: String) -> Self {
        Self::new_with_url(String::from("https://bunpro.jp/api"), api_key)
    }

    pub fn new_with_url(base_url: String, api_key: String) -> Self {
        BunproClient { base_url, api_key }
    }

    pub fn study_queue(self) -> Result<BunproResponse<StudyQueue>, Error> {
        let url = self.construct_url("study_queue");
        let response = reqwest::blocking::get(url)?.json::<BunproResponse<StudyQueue>>()?;
        Ok(response)
    }

    fn construct_url(self, resource: &str) -> String {
        format!("{}/user/{}/{resource}", self.base_url, self.api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn url_constructor() {
        let b = BunproClient::new_with_url("base_url".into(), "api_key".into());
        let u = b.construct_url("resource");
        assert_eq!("base_url/user/api_key/resource", u);
    }

    #[test]
    fn reqwest_errors_work() {
        let b = BunproClient::new_with_url("base_url".into(), "api_key".into());
        let e = b.study_queue();
        match e {
            Err(Error::ClientError(_)) => {}
            _ => panic!("{e:?}"),
        }
    }

    #[async_std::test]
    async fn study_queue() {
        let body = std::fs::read_to_string("test/bunpro/study_queue_response.json").unwrap();
        let response = ResponseTemplate::new(200).set_body_raw(body, "text/json");

        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/user/key/study_queue"))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let response = BunproClient::new_with_url(mock_server.uri(), "key".into())
            .study_queue()
            .unwrap();

        assert_eq!("onlyhavecans", response.user_information.username);
        assert_eq!(
            7,
            response.requested_information.reviews_available_next_hour
        );
    }
}
