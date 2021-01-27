//! Tracks the progress of a crawl

use alloc::collections::BTreeSet;
use oxigraph::model as om;

#[derive(Default, Debug)]
pub struct CrawlProgress {
    visited: BTreeSet<String>,
    errors: BTreeSet<String>,
}

impl CrawlProgress {
    pub fn novel(&self, iri: &om::NamedNode) -> bool {
        let st = iri.as_str();
        !(self.visited.contains(st) || self.errors.contains(st))
    }

    pub fn visit(&mut self, iri: om::NamedNode) {
        self.visited.insert(iri.into_string());
    }

    pub fn error(&mut self, iri: om::NamedNode) {
        self.errors.insert(iri.into_string());
    }
}
