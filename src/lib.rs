//! # BlurHash encoder in portable pure Rust

//! This code implements an encoder for the BlurHash algorithm in C. It can be used to integrate into other language
//! using an FFI interface. Currently the Python integration uses this code.

mod base83;
mod decoder;
mod encoder;
mod utils;

pub use decoder::decode;
pub use decoder::validate;
pub use decoder::DecodeError;

pub use encoder::encode;
pub use encoder::EncodeError;
