mod agent;
mod convert;
mod crawl_progress;
mod curiosity;
#[cfg(test)]
mod delegation_rules;
mod lookup;
mod rdf_graph;
mod store;
mod translate;
mod ttl;
mod types;
mod util;

extern crate alloc;
extern crate core;

pub use agent::Agent;
pub use curiosity::Curiosity;
pub use lookup::{Lookup, LookupError};
pub use rdf_graph::Graph;
pub use store::Store;

#[cfg(test)]
mod test {
    use super::*;
    use crate::agent::Agent;
    use crate::curiosity::Curiosity;
    use crate::rdf_graph::Graph;
    use crate::store::Store;
    use crate::ttl::from_ttl;
    use crate::util::as_named_node;
    use alloc::collections::BTreeMap;
    use futures_lite::future::block_on;
    use oxigraph::model as om;
    use oxigraph::sparql::QuerySolutionIter;
    use oxigraph::sparql::{algebra::Query, QueryResults};
    use oxigraph::store::MemoryStore;
    use tap::prelude::*;

    #[test]
    fn simple_crawl() {
        block_on(async {
            let mut ca = default_agent();
            ca.investigate(named_node("did:a")).await.unwrap();
            ca.crawl().await.unwrap();
            assert_eq!(
                list_graphs(&ca)
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
        })
    }

    fn default_agent() -> Agent<MemoryStore, BTreeMap<&'static str, Graph>> {
        let curio = Curiosity::create(curious_about_everything()).unwrap();
        let memst = MemoryStore::default();
        let sup = supergraph();
        Agent::new(curio, memst, sup)
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

    fn _known() -> Graph {
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

    fn list_graphs(store: &impl Store) -> impl Iterator<Item = om::Term> {
        fn as_solutions(q: QueryResults) -> Option<QuerySolutionIter> {
            match q {
                QueryResults::Solutions(solutions) => Some(solutions),
                QueryResults::Boolean(_) | QueryResults::Graph(_) => None,
            }
        }

        store
            .query(
                "SELECT DISTINCT ?g WHERE { GRAPH ?g { ?s ?p ?o } }"
                    .parse()
                    .unwrap(),
            )
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

    fn named_node(iri: &str) -> om::NamedNode {
        om::NamedNode::new(iri).unwrap()
    }
}
