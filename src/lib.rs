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

#[cfg(test)]
mod test {
    use super::*;
    use rify::Entity::{Bound, Unbound};

    #[test]
    fn delegate_e2e() {
        let dr = delegation_rules();
    }

    fn dock(suffix: &str) -> rify::Entity<&str, types::RdfNode> {
        bound("https://dock.io/rdf/alpha/", suffix)
    }

    fn rdf(suffix: &str) -> rify::Entity<&str, types::RdfNode> {
        bound("http://www.w3.org/1999/02/22-rdf-syntax-ns#", suffix)
    }

    fn bound(pre: &str, suff: &str) -> rify::Entity<&'static str, types::RdfNode> {
        let ret = format!("{}{}", pre, suff);
        assert!([
            "https://dock.io/rdf/alpha/claims",
            "https://dock.io/rdf/alpha/allowsSubject",
            "https://dock.io/rdf/alpha/allowsPredicate",
            "https://dock.io/rdf/alpha/allowsObject",
            "https://dock.io/rdf/alpha/SubjectUnbound",
            "https://dock.io/rdf/alpha/PredicateUnbound",
            "https://dock.io/rdf/alpha/ObjectUnbound",
            "https://dock.io/rdf/alpha/mayClaim",
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject",
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate",
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#object",
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
        ]
        .contains(&ret.as_str()));
        Bound(types::RdfNode::Iri(ret))
    }

    // Delegation Rules
    // Option 1:
    // if   a? rdf:type dock:SubjectUnbound
    // and  b? rdf:type dock:Mentioned
    // then a? dock:allowsSubject b?
    //
    // if   a? rdf:type dock:PredicateUnbound
    // and  b? rdf:type dock:Mentioned
    // then a? dock:allowsPredicate b?
    //
    // if   a? rdf:type dock:ObjectUnbound
    // and  b? rdf:type dock:Mentioned
    // then a? dock:allowsObject b?
    //
    // if   a? b? c?
    // then a? rdf:type dock:Mentioned
    // and  b? rdf:type dock:Mentioned
    // and  c? rdf:type dock:Mentioned

    // Option 2:
    // if   a?   dock:claims           [s? p? o?]
    // and  pol? rdf:type              dock:SubjectUnbound
    // and  pol? rdf:type              dock:PredicateUnbound
    // and  pol? rdf:type              dock:ObjectUnbound
    // and  a?   dock:mayClaim         pol?
    // then s? p? o?
    //
    // if   a?   dock:claims           [s? p? o?]
    // and  pol? rdf:type              dock:SubjectUnbound
    // and  pol? rdf:type              dock:PredicateUnbound
    // and  pol? dock:allowsObject     o?
    // and  a?   dock:mayClaim         pol?
    // then s? p? o?
    //
    // if   a?   dock:claims           [s? p? o?]
    // and  pol? rdf:type              dock:SubjectUnbound
    // and  pol? dock:allowsPredicate  p?
    // and  pol? rdf:type              dock:ObjectUnbound
    // and  a?   dock:mayClaim         pol?
    // then s? p? o?
    //
    // if   a?   dock:claims          [s? p? o?]
    // and  pol? rdf:type             dock:SubjectUnbound
    // and  pol? dock:allowsPredicate p?
    // and  pol? dock:allowsObject    o?
    // and  a?   dock:mayClaim        pol?
    // then s? p? o?
    //
    // if   a?   dock:claims          [s? p? o?]
    // and  pol? dock:allowsSubject   s?
    // and  pol? rdf:type             dock:PredicateUnbound
    // and  pol? rdf:type             dock:ObjectUnbound
    // and  a?   dock:mayClaim        pol?
    // then s? p? o?
    //
    // if   a?   dock:claims           [s? p? o?]
    // and  pol? dock:allowsSubject    s?
    // and  pol? rdf:type              dock:PredicateUnbound
    // and  pol? dock:allowsObject     o?
    // and  a?   dock:mayClaim         pol?
    // then s? p? o?
    //
    // if   a?   dock:claims           [s? p? o?]
    // and  pol? dock:allowsSubject    s?
    // and  pol? dock:allowsPredicate  p?
    // and  pol? rdf:type              dock:ObjectUnbound
    // and  a?   dock:mayClaim         pol?
    // then s? p? o?
    //
    // if   a?   dock:claims           [s? p? o?]
    // and  pol? dock:allowsSubject    s?
    // and  pol? dock:allowsPredicate  p?
    // and  pol? dock:allowsObject     o?
    // and  a?   dock:mayClaim         pol?
    // then s? p? o?

    // Option 3: TODO Try this
    // if   ?a dock:claims [rdf:subject ?s ; rdf:predicate ?p ; rdf:object ?o]
    // and  ?a dock:mayClaim [
    //     dock:subjects   { [ rdfs:member ?s ] } OR { dock:Anything } ;
    //     dock:predicates { [ rdfs:member ?p ] } OR { dock:Anything } ;
    //     dock:objects    { [ rdfs:member ?o ] } OR { dock:Anything } ;
    // ]
    // then [?s ?p ?o]

    fn delegation_rules() -> Vec<[Vec<[rify::Entity<&'static str, types::RdfNode>; 3]>; 2]> {
        let mut ret = vec![];
        let [ss, os, ps] = [
            [
                [Unbound("pol"), dock("allowsSubject"), Unbound("s")],
                [Unbound("pol"), rdf("type"), dock("SubjectUnbound")],
            ],
            [
                [Unbound("pol"), dock("allowsPredicate"), Unbound("p")],
                [Unbound("pol"), rdf("type"), dock("PredicateUnbound")],
            ],
            [
                [Unbound("pol"), dock("allowsObject"), Unbound("o")],
                [Unbound("pol"), rdf("type"), dock("ObjectUnbound")],
            ],
        ];
        for s in &ss {
            for p in &ps {
                for o in &os {
                    ret.push([
                        vec![
                            // ?a dock:claims [rdf:subject ?s ; rdf:predicate ?p ; rdf:object ?o] .
                            [Unbound("a"), dock("claims"), Unbound("c")],
                            [Unbound("c"), rdf("subject"), Unbound("s")],
                            [Unbound("c"), rdf("predicate"), Unbound("p")],
                            [Unbound("c"), rdf("object"), Unbound("o")],
                            // ?a dock:mayClaim [
                            //     { dock:allowsSubject ?s } OR { rdf:type dock:SubjectUnbound } ;
                            //     { dock:allowsPredicate ?s } OR { rdf:type dock:PredicateUnbound } ;
                            //     { dock:allowsObject ?s } OR { rdf:type dock:ObjectUnbound } ;
                            // ] .
                            [Unbound("a"), dock("mayClaim"), Unbound("pol")],
                            s.clone(),
                            p.clone(),
                            o.clone(),
                        ],
                        vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
                    ]);
                }
            }
        }
        ret
    }
}
