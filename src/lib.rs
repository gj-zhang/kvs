#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;

mod kv;
mod error;

pub use kv::KvStore;
pub use error::{KvsError, Result};
