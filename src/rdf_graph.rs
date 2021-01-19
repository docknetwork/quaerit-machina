use crate::types::RdfNode;
use crate::util::prefix::{dock, rdf, rdfs};
use alloc::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Graph(pub Vec<[RdfNode; 3]>);

impl Graph {
    pub fn merge(mut self, mut other: Self) -> Self {
        let mut namer = Namer::default();
        namer.realloc_names(&mut self);
        namer.realloc_names(&mut other);
        self.0.extend(other.0);
        self
    }

    pub fn reassign_blanks(&mut self) {
        Namer::default().realloc_names(self)
    }

    fn nodes_mut(&mut self) -> impl Iterator<Item = &mut RdfNode> {
        self.0.iter_mut().flatten()
    }

    fn blanks_mut(&mut self) -> impl Iterator<Item = &mut String> {
        self.nodes_mut().filter_map(|node| match node {
            RdfNode::Blank(s) => Some(s),
            _ => None,
        })
    }

    /// make explict that this graph was dereferenced from src
    /// the output will state that 'src' points to a graph containing the input graph
    /// the input graph expressed as an edge list of named tuples
    pub fn reify(self, src: RdfNode) -> Self {
        let mut slef = self;
        let mut namer = Namer::default();
        let collection = RdfNode::Blank(namer.next());
        namer.realloc_names(&mut slef);

        let mut ret = Graph(Vec::with_capacity(slef.0.len() * 4 + 1));
        ret.0
            .push([src, dock("dereferencesTo"), collection.clone()]);

        for [s, p, o] in slef.0.drain(..) {
            let c = RdfNode::Blank(namer.next());
            ret.0.push([collection.clone(), rdfs("member"), c.clone()]);
            ret.0.push([c.clone(), rdf("subject"), s]);
            ret.0.push([c.clone(), rdf("predicate"), p]);
            ret.0.push([c, rdf("object"), o]);
        }

        debug_assert_eq!(ret.0.capacity(), ret.0.len());
        ret
    }
}

#[derive(Default)]
pub struct Namer {
    count: usize,
}

impl Namer {
    pub fn next(&mut self) -> String {
        let ret = format!("b{}", self.count);
        self.count += 1;
        ret
    }

    pub fn realloc_names(&mut self, g: &mut Graph) {
        let mut renames: BTreeMap<String, String> = BTreeMap::new();
        for name in g.blanks_mut() {
            *name = renames
                .entry(name.to_string())
                .or_insert_with(|| self.next())
                .clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::RdfNode::{Blank, Iri};

    #[test]
    fn merge_sanity() {
        assert_eq!(
            Graph(vec![[
                RdfNode::Blank("b".to_string()),
                RdfNode::Blank("a".to_string()),
                RdfNode::Blank("b".to_string()),
            ]])
            .merge(Graph(vec![[
                RdfNode::Blank("b".to_string()),
                RdfNode::Blank("a".to_string()),
                RdfNode::Blank("b".to_string()),
            ]])),
            Graph(vec![
                [
                    RdfNode::Blank("b0".to_string()),
                    RdfNode::Blank("b1".to_string()),
                    RdfNode::Blank("b0".to_string()),
                ],
                [
                    RdfNode::Blank("b2".to_string()),
                    RdfNode::Blank("b3".to_string()),
                    RdfNode::Blank("b2".to_string()),
                ]
            ])
        );
    }

    #[test]
    fn reify_sanity() {
        assert_eq!(
            Graph(vec![
                [
                    RdfNode::Blank("b".to_string()),
                    RdfNode::Blank("a".to_string()),
                    RdfNode::Blank("b".to_string()),
                ],
                [
                    RdfNode::Blank("b".to_string()),
                    RdfNode::Blank("a".to_string()),
                    RdfNode::Blank("b".to_string()),
                ]
            ])
            .reify(RdfNode::Iri("https://example.com/a.ttl".to_string())),
            Graph(vec![
                [
                    Iri("https://example.com/a.ttl".to_string()),
                    Iri("https://dock.io/rdf/alpha/dereferencesTo".to_string()),
                    Blank("b0".to_string())
                ],
                [
                    Blank("b0".to_string()),
                    Iri("http://www.w3.org/2000/01/rdf-schema#member".to_string()),
                    Blank("b3".to_string())
                ],
                [
                    Blank("b3".to_string()),
                    Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()),
                    Blank("b1".to_string())
                ],
                [
                    Blank("b3".to_string()),
                    Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()),
                    Blank("b2".to_string())
                ],
                [
                    Blank("b3".to_string()),
                    Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()),
                    Blank("b1".to_string())
                ],
                [
                    Blank("b0".to_string()),
                    Iri("http://www.w3.org/2000/01/rdf-schema#member".to_string()),
                    Blank("b4".to_string())
                ],
                [
                    Blank("b4".to_string()),
                    Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()),
                    Blank("b1".to_string())
                ],
                [
                    Blank("b4".to_string()),
                    Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()),
                    Blank("b2".to_string())
                ],
                [
                    Blank("b4".to_string()),
                    Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()),
                    Blank("b1".to_string())
                ]
            ])
        );
    }
}
