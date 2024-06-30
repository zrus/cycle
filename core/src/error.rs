use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Generate key failed")]
  GenerateKeyFailed,
  #[error("Unable to parse to PEM")]
  UnableToParseToPem,
  #[error("Unable to read PEM")]
  UnableToReadPem,
  #[error("Unable to read der")]
  UnableToReadDer,
  #[error("Encrypt failed")]
  EncryptFailed,
  #[error("Decrypt failed")]
  DecryptFailed,
  #[error("Invalid message length")]
  InvalidMessageLength,
  #[error("Decode key failed")]
  DecodeKeyFailed,
  #[error("Unable to parse encrypted")]
  UnableToParseEncrypted,
  #[error("Unable to parse cipher text")]
  UnableToParseCipher,
}
