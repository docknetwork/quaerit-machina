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
}

impl IntoIterator for Graph {
    type IntoIter = <Vec<Triple> as IntoIterator>::IntoIter;
    type Item = Triple;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.0.into_iter()
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
