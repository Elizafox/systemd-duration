#[derive(Debug, thiserror::Error)]
/// The systemd-duration error type.
pub enum Error {
    #[error("Duration overflowed")]
    DurationOverflow,

    #[error(transparent)]
    ParserError(#[from] nom::error::Error<String>),
}
