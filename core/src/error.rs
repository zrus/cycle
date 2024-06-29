use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("generate key failed")]
  GenerateKeyFailed,
}
