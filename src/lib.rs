mod translate;
mod types;

use rify::Claim;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
