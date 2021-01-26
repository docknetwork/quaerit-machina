use oxigraph::model::{BlankNode, NamedOrBlankNode, Term, Triple};
use std::collections::HashMap;

/// Invariant upheld: All blank nodes in graph are random.
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Graph(Vec<Triple>);

impl Graph {
    pub fn new(ts: impl Iterator<Item = Triple>) -> Self {
        let mut ret = Graph(vec![]);
        ret.extend_hygienic(ts);
        ret
    }

    /// Add another graph to this one.
    /// Thes function protects against blank conflations.
    fn extend_hygienic(&mut self, other: impl Iterator<Item = Triple>) {
        self.0.extend(rename_blanks(other));
    }

    pub fn triples(&self) -> &[Triple] {
        &self.0
    }
}

fn rename_blanks(inp: impl Iterator<Item = Triple>) -> impl Iterator<Item = Triple> {
    type Renames = HashMap<BlankNode, BlankNode>;

    fn rename_bn(bn: BlankNode, renames: &mut Renames) -> BlankNode {
        renames.entry(bn).or_default().clone()
    }

    fn rename_named_or_blank(nb: NamedOrBlankNode, renames: &mut Renames) -> NamedOrBlankNode {
        use NamedOrBlankNode::{BlankNode, NamedNode};
        match nb {
            BlankNode(bn) => BlankNode(rename_bn(bn, renames)),
            NamedNode(nn) => NamedNode(nn),
        }
    }

    fn rename_term(tm: Term, renames: &mut Renames) -> Term {
        use Term::{BlankNode, Literal, NamedNode};
        match tm {
            BlankNode(bn) => BlankNode(rename_bn(bn, renames)),
            NamedNode(nn) => NamedNode(nn),
            Literal(lt) => Literal(lt),
        }
    }

    let mut renames = Renames::new();
    inp.map(
        move |Triple {
                  subject,
                  predicate,
                  object,
              }| Triple {
            subject: rename_named_or_blank(subject, &mut renames),
            predicate,
            object: rename_term(object, &mut renames),
        },
    )
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn reify_sanity() {
        // unimplemented!();
        // assert_eq!(
        //     Graph(vec![
        //         [
        //             RdfNode::Blank("b".to_string()),
        //             RdfNode::Blank("a".to_string()),
        //             RdfNode::Blank("b".to_string()),
        //         ],
        //         [
        //             RdfNode::Blank("b".to_string()),
        //             RdfNode::Blank("a".to_string()),
        //             RdfNode::Blank("b".to_string()),
        //         ]
        //     ])
        //     .reify(RdfNode::Iri("https://example.com/a.ttl".to_string())),
        //     Graph(vec![
        //         [
        //             Iri("https://example.com/a.ttl".to_string()),
        //             Iri("https://dock.io/rdf/alpha/dereferencesTo".to_string()),
        //             Blank("b0".to_string())
        //         ],
        //         [
        //             Blank("b0".to_string()),
        //             Iri("http://www.w3.org/2000/01/rdf-schema#member".to_string()),
        //             Blank("b3".to_string())
        //         ],
        //         [
        //             Blank("b3".to_string()),
        //             Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()),
        //             Blank("b1".to_string())
        //         ],
        //         [
        //             Blank("b3".to_string()),
        //             Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()),
        //             Blank("b2".to_string())
        //         ],
        //         [
        //             Blank("b3".to_string()),
        //             Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()),
        //             Blank("b1".to_string())
        //         ],
        //         [
        //             Blank("b0".to_string()),
        //             Iri("http://www.w3.org/2000/01/rdf-schema#member".to_string()),
        //             Blank("b4".to_string())
        //         ],
        //         [
        //             Blank("b4".to_string()),
        //             Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()),
        //             Blank("b1".to_string())
        //         ],
        //         [
        //             Blank("b4".to_string()),
        //             Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()),
        //             Blank("b2".to_string())
        //         ],
        //         [
        //             Blank("b4".to_string()),
        //             Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()),
        //             Blank("b1".to_string())
        //         ]
        //     ])
        // );
    }
}
