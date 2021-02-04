//! The Resolve trait specifies the ability to find some document addressed by iri

use async_trait::async_trait;
use core::fmt::{Debug, Display};

/// stringly typed error
#[derive(Debug)]
pub struct ResolveError(String);

impl<T: Display> From<T> for ResolveError {
    fn from(other: T) -> ResolveError {
        ResolveError(format!("{}", other))
    }
}

#[async_trait]
pub trait Resolve {
    async fn lookup(&mut self, iri: &str) -> Result<Vec<u8>, ResolveError>;
}
