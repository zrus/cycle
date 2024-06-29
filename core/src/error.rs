use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("generate key failed")]
  GenerateKeyFailed,
  #[error("encrypt failed")]
  EncryptFailed,
  #[error("decrypt failed")]
  DecryptFailed,
  #[error("invalid message length")]
  InvalidMessageLength,
}
