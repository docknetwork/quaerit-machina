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

use crate::rdf_graph::{Graph, Namer};
use rify::{Claim, Entity, Entity::Bound};
use types::RdfNode::{self, Iri};

type Rules = Vec<rify::Rule<String, RdfNode>>;
pub struct Curio {
    if_all: Vec<Claim<Entity<String, RdfNode>>>,
    interesting: Vec<Entity<String, RdfNode>>,
}

impl Curio {
    fn to_rule(self) -> Result<rify::Rule<String, RdfNode>, rify::InvalidRule<String>> {
        let Curio {
            if_all,
            interesting,
        } = self;
        let then = interesting
            .into_iter()
            .map(|s| {
                [
                    s,
                    Bound(Iri("uuid:19d3546f-e33c-4f7d-bac0-64c42fc20a02".to_string())),
                    Bound(Iri("uuid:4e507a3e-0d0a-4c97-9ea8-aa1a51285006".to_string())),
                ]
            })
            .collect();
        rify::Rule::create(if_all, then)
    }
}

pub type Curiosity = Vec<Curio>;

pub struct CuriousAgent {
    logic: Rules,
    curiosity: Rules,
    cg: Graph,
    namer: Namer,
}

impl CuriousAgent {
    pub fn create(logic: Rules, curiosity: Curiosity) -> Result<Self, rify::InvalidRule<String>> {
        let curiosity = curiosity
            .into_iter()
            .map(Curio::to_rule)
            .collect::<Result<_, rify::InvalidRule<String>>>()?;
        Ok(CuriousAgent {
            logic,
            curiosity,
            cg: Graph::default(),
            namer: Namer::default(),
        })
    }

    fn reason(&self) -> Vec<rify::Claim<RdfNode>> {
        unimplemented!()
    }

    fn curious(&self) -> Vec<RdfNode> {
        unimplemented!()
    }

    /// Extend the inner knowlege graph but dont rename any blank nodes
    fn extend_unhygienic(&mut self, other: Graph) {
        self.cg.0.extend(other.0)
    }

    /// Extend the inner knowlege graph, renaming blank nodes to prevent collisions
    fn extend_hygienic(&mut self, other: Graph) {
        let mut other = other;
        self.namer.realloc_names(&mut other);
        self.extend_hygienic(other);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rdf_graph::Graph;
    use crate::ttl::from_ttl;
    use crate::types::RdfNode;
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

        let mut ca = CuriousAgent::create(vec![], vec![]).unwrap();
        crawl(&mut ca, &supergraph);

        // The age claim from C is successfully unwrapped
        unimplemented!();
        // The age claim from D is ignored
        unimplemented!();
    }

    // TODO
    fn crawl(ca: &mut CuriousAgent, l: &impl Lookup) {}

    trait Lookup {
        fn lookup(&self, url: &str) -> Result<Graph, ()>;
    }

    impl Lookup for BTreeMap<&str, Graph> {
        fn lookup(&self, url: &str) -> Result<Graph, ()> {
            let raw = self.get(url).ok_or(())?;
            Ok(raw.clone().reify(RdfNode::Iri(url.to_string())))
        }
    }
}
