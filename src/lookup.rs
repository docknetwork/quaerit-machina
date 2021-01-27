//! The Lookup trait specifies the ability to find some rdf graph addressed by Iri

use crate::rdf_graph::Graph;
use alloc::collections::BTreeMap;
use async_trait::async_trait;
use oxigraph::model as om;

mod a {
    use super::*;

    pub trait Lookup {
        type Error;
        fn lookup(&self, iri: &om::NamedNode) -> Result<Graph, Self::Error>;
    }

    impl Lookup for BTreeMap<&str, Graph> {
        type Error = ();
        fn lookup(&self, iri: &om::NamedNode) -> Result<Graph, Self::Error> {
            self.get(iri.as_str()).map(Clone::clone).ok_or(())
        }
    }
}

#[async_trait]
pub trait Lookup {
    type Error;
    async fn lookup(&self, iri: &om::NamedNode) -> Result<Graph, Self::Error>;
}

#[async_trait]
impl Lookup for BTreeMap<&str, Graph> {
    type Error = ();
    async fn lookup(&self, iri: &om::NamedNode) -> Result<Graph, Self::Error> {
        self.get(iri.as_str()).map(Clone::clone).ok_or(())
    }
}
