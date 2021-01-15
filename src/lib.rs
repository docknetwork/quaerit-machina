mod convert;
#[cfg(test)]
mod delegation_rules;
mod rdf_graph;
mod translate;
mod ttl;
mod types;

extern crate alloc;
extern crate core;

use alloc::collections::{BTreeMap, BTreeSet};
use rify::Claim;

pub struct CuriousAgent {
    /// the implication relationships used thus far, these relationships tell us which knowlege to
    /// invalidate when information sources change
    _implications: BTreeMap<Fact, Claim<Id>>,
    /// Everthing the agent has ever been curious about, excluding unresolved curiosities.
    _curious: BTreeSet<Id>,
    /// Things the agent is curious about, but for which its curiosity has not been satiated.
    _unresolved_curious: BTreeSet<Id>,
    _knowlege: rify::reasoner::TripleStore,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fact {
    Derived(Claim<Id>),
    Known,
}

/// An index to some rdf entity
type Id = usize;

#[cfg(test)]
mod test {
    use super::*;
    use crate::rdf_graph::Graph;
    use crate::ttl::from_ttl;
    use crate::types::RdfNode;

    #[test]
    fn delegate_e2e() {
        // A is trusted
        // A claims that B mayClaim anything
        // B claims that C mayClaim age claims
        // C claims an age claim
        // D claims an age claim

        // The age claim from C is successfully unwrapped
        // The age claim from D is ignored

        let supergraph: &[(&str, &str)] = &[
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
                <did:b> dock:mayclaim [
                    dock:allowedSubjects dock:ANYTHING ;
                    dock:allowedPredicates dock:ANYTHING ;
                    dock:allowedObjects dock:ANYTHING ;
                ] .
                ",
            ),
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
                @prefix ex: <https://example.com/> .
                <did:c> dock:mayclaim [
                    dock:allowedSubjects dock:ANYTHING ;
                    dock:allowedPredicates ( ex:born ) ;
                    dock:allowedObjects dock:ANYTHING ;
                ] .
                ",
            ),
        ];
        let supergraph: BTreeMap<&str, Graph> =
            supergraph.iter().map(|(a, b)| (*a, from_ttl(b))).collect();
        unimplemented!("{:?}", supergraph);
    }

    fn reify_claim(claimer: RdfNode, triple: Claim<RdfNode>) -> [Claim<RdfNode>; 4] {
        unimplemented!()
    }
}
