use crate::crawl_progress::CrawlProgress;
use crate::curiosity::Curiosity;
use crate::lookup::Lookup;
use crate::rdf_graph::Graph;
use crate::store::Store;
use crate::util::as_named_node;
use oxigraph::model as om;
use oxigraph::sparql::{algebra::Query, EvaluationError, QueryResults};

pub struct Agent<S, L> {
    curiosity: Curiosity,
    knowlege_graph: S,
    progress: CrawlProgress,
    lookup: L,
}

impl<S: Store, L: Lookup> Agent<S, L> {
    pub fn new(curiosity: Curiosity, knowlege_graph: S, lookup: L) -> Self {
        Self {
            curiosity,
            knowlege_graph,
            progress: Default::default(),
            lookup,
        }
    }

    pub fn investigate(&mut self, document: om::NamedNode) {
        self.progress.visit(document.clone());
        self.lookup
            .lookup(&document)
            .map(|cont| self.note_document_contents(document, cont));
    }

    fn curious(&self) -> Result<Vec<om::NamedNode>, EvaluationError> {
        let mut ret = Vec::new();
        self.curiosity.curious(&self.knowlege_graph, |term| {
            as_named_node(term)
                .filter(|nn| self.progress.novel(nn))
                .map(|nn| ret.push(nn.clone()));
        })?;
        Ok(ret)
    }

    fn note_document_contents(&mut self, document: om::NamedNode, contents: Graph) {
        for triple in contents {
            self.knowlege_graph
                .insert(triple.in_graph(document.clone()));
        }
    }

    pub fn crawl(&mut self) -> Result<(), EvaluationError> {
        while self.next()? {}
        Ok(())
    }

    pub fn next(&mut self) -> Result<bool, EvaluationError> {
        let curious = self.curious()?;
        debug_assert!(curious.iter().all(|nn| self.progress.novel(nn)));
        if curious.is_empty() {
            return Ok(false);
        }
        for nn in &curious {
            self.investigate(nn.clone());
        }
        debug_assert!(!curious.iter().any(|nn| self.progress.novel(nn)));
        Ok(true)
    }
}

impl<S: Store, L> Store for Agent<S, L> {
    fn insert(&mut self, quad: om::Quad) {
        Store::insert(&mut self.knowlege_graph, quad)
    }

    fn query(&self, query: Query) -> Result<QueryResults, EvaluationError> {
        Store::query(&self.knowlege_graph, query)
    }
}
