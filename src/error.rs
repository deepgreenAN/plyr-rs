/// Error in Plyr
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("PlyrError: event_type and provider are mismatched")]
    WrongEventProviderError,
}
