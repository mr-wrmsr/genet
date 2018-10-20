extern crate fnv;
extern crate libc;
extern crate parking_lot;

#[macro_use]
extern crate lazy_static;

pub mod attr;
pub mod cast;
pub mod context;
pub mod decoder;
pub mod env;
pub mod error;
pub mod fixed;
pub mod layer;
pub mod metadata;
pub mod reader;
pub mod result;
pub mod slice;
pub mod token;
pub mod variant;
pub mod writer;

mod string;
mod vec;
