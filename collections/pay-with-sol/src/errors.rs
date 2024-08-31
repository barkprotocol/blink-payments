use znap::prelude::*;

#[derive(ErrorCode)]
pub enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
    #[error(msg = "Invalid receiver public key")]
    InvalidReceiverPublicKey,
    #[error(msg = "Invalid SOL mint public key")]
    InvalidTokenMintPublicKey,
    #[error(msg = "Error obtaining SOL account data")]
    ErrorObtainingTokenAccountData,
    #[error(msg = "Error obtaining SOL metadata")]
    ErrorObtainingTokenMetadata,
    #[error(msg = "Internal server error")]
    InternalServerError,
    #[error(msg = "Unknown server error")]
    UnknownServerError,
    #[error(msg = "Invalid response body")]
    InvalidResponseBody,
    #[error(msg = "No quote was found for SOL at this time")]
    QuoteNotFound,

}