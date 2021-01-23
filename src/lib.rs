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
use alloc::collections::BTreeSet;
use oxigraph::model::NamedNode;
use oxigraph::model::NamedOrBlankNode;
use oxigraph::model::Term;
use oxigraph::sparql::algebra::Query;
use oxigraph::sparql::EvaluationError;
use oxigraph::sparql::QueryResults;
use oxigraph::store::memory::MemoryStore;

pub struct CuriousAgent {
    /// queries in this list must all be select statements
    curiosity: Vec<Query>,
    cg: MemoryStore,
    lookedup: BTreeSet<String>,
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

    pub fn curious(&self) -> Result<Vec<Term>, EvaluationError> {
        assert!(self.curiosity.iter().all(is_select));
        let mut ret = Vec::new();
        for cur in &self.curiosity {
            let q = self.cg.query(cur.clone())?;
            match q {
                QueryResults::Solutions(solutions) => {
                    for s in solutions {
                        for (_name, term) in s?.iter() {
                            ret.push(term.clone());
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

    pub fn add_document_lookup(&mut self, iri: &str, mut content: Graph) {
        content.reify(NamedOrBlankNode::NamedNode(NamedNode::new(iri).unwrap()));
        for triple in content.triples() {
            self.cg.insert(triple.clone().in_graph(None));
        }
    }

    pub fn crawl(&mut self, l: &impl Lookup) {
        while !self.next(l) {}
    }

    pub fn next(&mut self, l: &impl Lookup) -> bool {
        let curious = self.curious().unwrap();
        let curious = curious
            .iter()
            .filter_map(as_named_node)
            .map(|n| n.as_str().to_string());
        if curious.clone().all(|nn| self.lookedup.contains(&nn)) {
            true
        } else {
            for c in curious {
                self.try_lookup(&c, &l);
            }
            false
        }
    }

    pub fn try_lookup(&mut self, url: &str, l: impl Lookup) {
        if !self.lookedup.contains(url) {
            if let Some(graph) = l.lookup(url) {
                self.add_document_lookup(url, graph);
            }
            self.lookedup.insert(url.to_string());
        }
    }
}

fn show(cg: &MemoryStore) -> String {
    use std::io::Cursor;
    let mut ret = Vec::new();
    cg.dump_dataset(Cursor::new(&mut ret), oxigraph::io::DatasetFormat::NQuads)
        .unwrap();
    String::from_utf8(ret).unwrap()
}

pub trait Lookup {
    fn lookup(&self, url: &str) -> Option<Graph>;
}

impl Lookup for BTreeMap<&str, Graph> {
    fn lookup(&self, url: &str) -> Option<Graph> {
        self.get(url).cloned()
    }
}

impl<T: Lookup> Lookup for &T {
    fn lookup(&self, url: &str) -> Option<Graph> {
        (*self).lookup(url)
    }
}

fn as_named_node(term: &Term) -> Option<&NamedNode> {
    match term {
        Term::NamedNode(nn) => Some(nn),
        Term::BlankNode(_) | Term::Literal(_) => None,
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

    #[test]
    fn delegate_e2e() {
        // A is trusted
        let known = from_ttl(
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
            ",
        );
        let supergraph: &[(&str, &str)] = &[
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
        ];
        let supergraph: BTreeMap<&str, Graph> =
            supergraph.iter().map(|(a, b)| (*a, from_ttl(b))).collect();

        let mut ca = CuriousAgent::create(vec![
            "SELECT ?s WHERE { ?s ?p ?o }".parse().unwrap(),
            "SELECT ?p WHERE { ?s ?p ?o }".parse().unwrap(),
            "SELECT ?o WHERE { ?s ?p ?o }".parse().unwrap(),
        ])
        .unwrap();
        ca.try_lookup("did:a", &supergraph);

        dbg!(ca.cg.len());
        println!("{}", show(&ca.cg));
        while !ca.next(&supergraph) {
            dbg!(ca.cg.len());
        }
        dbg!(ca.cg.len());

        assert_ne!(ca.cg.len(), 0);

        // The age claim from C is successfully unwrapped
        unimplemented!();
        // The age claim from D is ignored
        unimplemented!();
    }
}
