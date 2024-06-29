mod error;
mod keypair;

pub use error::Error;
pub use keypair::KeyPair;

pub type Result<T, E = Error> = core::result::Result<T, E>;
