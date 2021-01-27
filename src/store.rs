//! Oxigraph doesn't seem to implement a Store trait for some reason.
//! This module defines a custom Store trait to fill that gap.

use oxigraph::model as om;
use oxigraph::sparql::{algebra::Query, EvaluationError, QueryResults};

pub trait Store {
    fn insert(&mut self, quad: om::Quad);
    fn query(&self, query: Query) -> Result<QueryResults, EvaluationError>;
}

impl Store for oxigraph::store::MemoryStore {
    fn insert(&mut self, quad: om::Quad) {
        oxigraph::store::MemoryStore::insert(self, quad)
    }

    fn query(&self, query: Query) -> Result<QueryResults, EvaluationError> {
        oxigraph::store::MemoryStore::query(self, query)
    }
}
