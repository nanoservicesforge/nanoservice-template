//! The entry point for compiling the TCP handling into another server.
//! This might be a good choice over other servers like the Actix or Axum if you are just calling the
//! nanoservice internally.
pub mod routing;
