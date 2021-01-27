//! Tracks the progress of a crawl

use alloc::collections::BTreeSet;
use oxigraph::model as om;

#[derive(Default, Debug)]
pub struct CrawlProgress {
    visited: BTreeSet<String>,
}

impl CrawlProgress {
    pub fn novel(&self, iri: &om::NamedNode) -> bool {
        !self.visited.contains(iri.as_str())
    }

    pub fn visit(&mut self, iri: om::NamedNode) {
        self.visited.insert(iri.into_string());
    }
}
