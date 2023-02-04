#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error happened when making the network request
    #[error("Network error: {}", _0)]
    ClientError(#[from] reqwest::Error),
    /// Bunpro returned an error from the request
    #[error("Bunpro error: {}", _0)]
    BunproError(String),
}
