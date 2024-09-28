//! # EnvType crate
//!
//! env-type crate is a library that provides an enum to represent the environment type.
//! It is used to determine the environment type of the application.
//! The environment type is used to determine the configuration of the application.
//!
//! The crate also provides a trait to get the environment key from the environment variable.
//! The environment key is used to get the environment type from the environment variable.
//!
//! ## Features
//!
//! env-type crate has the following features:
//!
//! - `shuttle`: This feature is used to get the environment type from the secret store.
//! - `all`: This feature is used to enable all features.
pub mod types;

#[cfg(feature = "shuttle")]
pub mod secret_store;
