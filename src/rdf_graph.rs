use crate::convert::AsBlank;
use crate::types::Term;
use crate::util::prefix::{dock, rdf, rdfs};
use oxigraph::model::BlankNode;
use oxigraph::model::NamedOrBlankNode;
use oxigraph::model::Triple;
use std::collections::HashMap;

/// Invariant upheld: All blank nodes in graph are random.
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Graph(Vec<Triple>);

impl Graph {
    pub fn new(ts: impl Iterator<Item = Triple>) -> Self {
        let mut ret = Graph(ts.collect());
        let mut rename = HashMap::<BlankNode, BlankNode>::new();
        ret.foreach_blank_mut(|b| {
            if let Some(n) = rename.get(&b) {
                *b = n.clone();
            } else {
                let n = BlankNode::default();
                rename.insert(b.clone(), n.clone());
                *b = n;
            }
        });
        ret
    }

    pub fn extend(&mut self, other: Self) {
        for sb in self.blanks() {
            for ob in other.blanks() {
                assert_ne!(sb, ob);
            }
        }
        self.0.extend(other.0);
    }

    pub fn triples(&self) -> &[Triple] {
        &self.0
    }

    /// make explict that this graph was dereferenced from src
    /// the output will state that 'src' points to a graph containing the input graph
    /// the original input graph will be expressed as a container of rdf:statement
    pub fn reify(&mut self, src: NamedOrBlankNode) {
        let mut ret = Vec::with_capacity(self.0.len() * 4 + 1);
        let collection = BlankNode::default();
        ret.push(Triple::new(src, dock("dereferencesTo"), collection.clone()));
        for Triple {
            subject,
            predicate,
            object,
        } in self.0.drain(..)
        {
            let c = BlankNode::default();
            ret.push(Triple::new(collection.clone(), rdfs("member"), c.clone()));
            ret.push(Triple::new(c.clone(), rdf("subject"), subject));
            ret.push(Triple::new(c.clone(), rdf("predicate"), predicate));
            ret.push(Triple::new(c, rdf("object"), object));
        }
        debug_assert_eq!(ret.capacity(), ret.len());
        self.0 = ret
    }

    fn blanks(&self) -> impl Iterator<Item = &BlankNode> {
        self.0
            .iter()
            .map(|Triple { subject, .. }| subject.as_blank())
            .chain(self.0.iter().map(|Triple { object, .. }| object.as_blank()))
            .filter_map(|a| a)
    }

    fn foreach_blank_mut(&mut self, mut f: impl FnMut(&mut BlankNode)) {
        for Triple {
            subject,
            predicate: _,
            object,
        } in self.0.iter_mut()
        {
            subject.as_blank_mut().map(&mut f);
            object.as_blank_mut().map(&mut f);
        }
    }
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
