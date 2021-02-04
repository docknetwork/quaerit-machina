//! The Lookup trait specifies the ability to find some rdf graph addressed by Iri

use crate::rdf_graph::Graph;
use alloc::collections::BTreeMap;
use async_trait::async_trait;
use core::fmt::{Debug, Display};
use oxigraph::model as om;
use tap::prelude::*;

// TODO: remove this stringly typed error
#[derive(Debug)]
pub struct LookupError(String);

impl<T: Display> From<T> for LookupError {
    fn from(other: T) -> LookupError {
        LookupError(format!("{}", other))
    }
}

#[async_trait]
pub trait Lookup {
    async fn lookup(&mut self, iri: &om::NamedNode) -> Result<Graph, LookupError>;
}

#[async_trait]
impl Lookup for BTreeMap<&str, Graph> {
    async fn lookup(&mut self, iri: &om::NamedNode) -> Result<Graph, LookupError> {
        self.get(iri.as_str())
            .map(Clone::clone)
            .ok_or("unregistered graph")?
            .pipe(Ok)
    }
}
