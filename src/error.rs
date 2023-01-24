/// Error in Plyr
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This is returned when event_type and provider are mismatched.
    #[error("PlyrError: event_type and provider are mismatched")]
    WrongEventProviderError,
}
