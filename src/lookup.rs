//! The Lookup trait specifies the ability to find some rdf graph addressed by Iri

use crate::rdf_graph::Graph;
use alloc::collections::BTreeMap;
use oxigraph::model as om;

pub trait Lookup {
    fn lookup(&self, iri: &om::NamedNode) -> Option<Graph>;
}

impl Lookup for BTreeMap<&str, Graph> {
    fn lookup(&self, iri: &om::NamedNode) -> Option<Graph> {
        self.get(iri.as_str()).map(Clone::clone)
    }
}
