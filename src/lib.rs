#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;

mod kv;
mod error;
mod engine;

pub use kv::KvStore;
pub use error::{KvsError, Result};
