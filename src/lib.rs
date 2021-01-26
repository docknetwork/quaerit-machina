mod convert;
#[cfg(test)]
mod delegation_rules;
mod rdf_graph;
mod translate;
mod ttl;
mod types;
mod util;

extern crate alloc;
extern crate core;

use crate::rdf_graph::Graph;
use alloc::collections::BTreeMap;
use oxigraph::model as om;
use oxigraph::sparql::{algebra::Query, EvaluationError, QueryResults};
use oxigraph::store::MemoryStore;
use std::collections::HashSet;

pub struct CuriousAgent {
    /// queries in this list must all be select statements
    curiosity: Vec<Query>,
    cg: MemoryStore,
    lookedup: HashSet<om::GraphName>,
}

impl CuriousAgent {
    pub fn create(curiosity: Vec<Query>) -> Result<Self, ()> {
        if !curiosity.iter().all(is_select) {
            return Err(());
        }
        Ok(Self {
            curiosity,
            cg: MemoryStore::new(),
            lookedup: Default::default(),
        })
    }

    pub fn curious(&self) -> Result<Vec<om::NamedNode>, EvaluationError> {
        let mut ret = Vec::new();
        for cur in &self.curiosity {
            let q = self.cg.query(cur.clone())?;
            match q {
                QueryResults::Solutions(solutions) => {
                    for s in solutions {
                        for nn in s?
                            .iter()
                            .filter_map(|(_name, term)| as_named_node(term))
                            .filter(|nn| self.is_novel(nn))
                        {
                            ret.push(nn.clone());
                        }
                    }
                }
                QueryResults::Boolean(_) | QueryResults::Graph(_) => {
                    panic!("Expected SELECT statements only.");
                }
            }
        }
        Ok(ret)
    }

    fn lookup(&mut self, nn: om::NamedNode, l: &impl Lookup) {
        if let Some(graph) = l.lookup(nn.as_str()) {
            for quad in graph.triples() {
                self.cg.insert(quad.clone().in_graph(nn.clone()));
            }
        }
        self.lookedup.insert(nn.clone().into());
    }

    pub fn crawl(&mut self, l: &impl Lookup) -> Result<(), EvaluationError> {
        while self.next(l)? {}
        Ok(())
    }

    pub fn next(&mut self, l: &impl Lookup) -> Result<bool, EvaluationError> {
        let curious = self.curious()?;
        debug_assert!(curious.iter().all(|nn| self.is_novel(nn)));
        if curious.is_empty() {
            return Ok(false);
        }
        for nn in &curious {
            self.lookup(nn.clone(), l);
        }
        debug_assert!(!curious.iter().any(|nn| self.is_novel(nn)));
        Ok(true)
    }

    fn is_novel(&self, nn: &om::NamedNode) -> bool {
        !self.lookedup.contains(&nn.clone().into())
    }
}

pub trait Lookup {
    fn lookup(&self, url: &str) -> Option<&Graph>;
}

impl Lookup for BTreeMap<&str, Graph> {
    fn lookup(&self, url: &str) -> Option<&Graph> {
        self.get(url)
    }
}

fn as_named_node(term: &om::Term) -> Option<&om::NamedNode> {
    match term {
        om::Term::NamedNode(nn) => Some(nn),
        om::Term::BlankNode(_) | om::Term::Literal(_) => None,
    }
}

fn is_select(q: &Query) -> bool {
    match q {
        Query::Select { .. } => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rdf_graph::Graph;
    use crate::ttl::from_ttl;
    use alloc::collections::BTreeMap;
    use oxigraph::sparql::QuerySolutionIter;
    use tap::prelude::*;

    #[test]
    fn simple_crawl() {
        let mut ca = CuriousAgent::create(curious_about_everything()).unwrap();
        ca.lookup(om::NamedNode::new("did:a").unwrap(), &supergraph());
        ca.crawl(&supergraph()).unwrap();
        assert_eq!(
            list_graphs(&ca.cg)
                .map(|term| as_named_node(&term).unwrap().clone().into_string())
                .pipe(sorted),
            [
                "did:b",
                "did:c:claims",
                "did:a",
                "did:a:claims",
                "did:b:claims",
                "did:c"
            ]
            .iter()
            .cloned()
            .map(str::to_string)
            .pipe(sorted)
        );
    }

    fn curious_about_everything() -> Vec<Query> {
        [
            "SELECT DISTINCT ?s WHERE { GRAPH ?g { ?s ?p ?o } }",
            "SELECT DISTINCT ?p WHERE { GRAPH ?g { ?s ?p ?o } }",
            "SELECT DISTINCT ?o WHERE { GRAPH ?g { ?s ?p ?o } }",
            "SELECT DISTINCT ?g WHERE { GRAPH ?g { ?s ?p ?o } }",
        ]
        .iter()
        .map(|a| a.parse().unwrap())
        .collect()
    }

    fn supergraph() -> BTreeMap<&'static str, Graph> {
        [
            // A claims that B mayClaim anything
            (
                "did:a",
                "
                @prefix dock: <https://dock.io/rdf/alpha/> .
                <did:a> dock:attestDocumentContent <did:a:claims> .
                ",
            ),
            (
                "did:a:claims",
                "
                @prefix dock: <https://dock.io/rdf/alpha/> .
                <did:b> dock:mayclaim <uuid:d653df41-fb26-46b2-9edf-35a73836f7e0> .
                ",
            ),
            // B claims that C mayClaim age claims
            (
                "did:b",
                "
                @prefix dock: <https://dock.io/rdf/alpha/> .
                <did:b> dock:attestDocumentContent <did:b:claims> .
                ",
            ),
            (
                "did:b:claims",
                "
                @prefix dock: <https://dock.io/rdf/alpha/> .
                <did:c> dock:mayclaim <uuid:ec3ae823-2e51-48ab-bdbf-bc41037eeead> .
                ",
            ),
            // C claims an age claim
            (
                "did:c",
                "
                @prefix dock: <https://dock.io/rdf/alpha/> .
                <did:c> dock:attestDocumentContent <did:c:claims> .
                ",
            ),
            (
                "did:c:claims",
                "
                @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
                @prefix schema: <http://schema.org/> .
                <did:c> schema:birthDate \"2002-09-24Z\"^^xsd:date .
                ",
            ),
            // D claims an age claim
            (
                "did:d",
                "
                @prefix dock: <https://dock.io/rdf/alpha/> .
                <did:c> dock:attestDocumentContent <did:d:claims> .
                ",
            ),
            (
                "did:d:claims",
                "
                @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
                @prefix schema: <http://schema.org/> .
                <did:d> schema:birthDate \"2002-09-24Z\"^^xsd:date .
                ",
            ),
        ]
        .iter()
        .map(|(a, b)| (*a, from_ttl(b)))
        .collect()
    }

    fn known() -> Graph {
        "
        @prefix dock: <https://dock.io/rdf/alpha/> .
        @prefix schema: <http://schema.org/> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        # Unrestricted delegation
        <uuid:d653df41-fb26-46b2-9edf-35a73836f7e0> dock:allowedSubjects dock:ANYTHING .
        <uuid:d653df41-fb26-46b2-9edf-35a73836f7e0> dock:allowedPredicates dock:ANYTHING .
        <uuid:d653df41-fb26-46b2-9edf-35a73836f7e0> dock:allowedObjects dock:ANYTHING .
        # Delegation that grants only the authority to claim birthdate
        <uuid:ec3ae823-2e51-48ab-bdbf-bc41037eeead> dock:allowedSubjects dock:ANYTHING .
        <uuid:ec3ae823-2e51-48ab-bdbf-bc41037eeead> dock:allowedPredicates
            [ rdfs:member schema:birthDate ] .
        <uuid:ec3ae823-2e51-48ab-bdbf-bc41037eeead> dock:allowedObjects dock:ANYTHING .
        # A is trusted with unrestricted delegation
        <did:a> dock:mayClaim <uuid:d653df41-fb26-46b2-9edf-35a73836f7e0> .
        "
        .pipe(from_ttl)
    }

    fn list_graphs(store: &MemoryStore) -> impl Iterator<Item = om::Term> {
        fn as_solutions(q: QueryResults) -> Option<QuerySolutionIter> {
            match q {
                QueryResults::Solutions(solutions) => Some(solutions),
                QueryResults::Boolean(_) | QueryResults::Graph(_) => None,
            }
        }

        store
            .query("SELECT DISTINCT ?g WHERE { GRAPH ?g { ?s ?p ?o } }")
            .unwrap()
            .pipe(as_solutions)
            .unwrap()
            .map(|a| a.unwrap())
            .flat_map(|solution| {
                solution
                    .iter()
                    .map(|(_k, v)| v)
                    .cloned()
                    .collect::<Vec<_>>()
            })
    }

    fn sorted<T: Ord>(inp: impl IntoIterator<Item = T>) -> Vec<T> {
        let mut ret = inp.into_iter().collect::<Vec<T>>();
        ret.sort();
        ret
    }
}
