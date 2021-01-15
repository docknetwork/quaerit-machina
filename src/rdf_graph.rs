use crate::types::RdfNode;
use alloc::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone)]
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
}

#[derive(Default)]
struct Namer {
    count: usize,
}

impl Namer {
    fn next(&mut self) -> String {
        let ret = format!("b{}", self.count);
        self.count += 1;
        ret
    }

    fn realloc_names(&mut self, g: &mut Graph) {
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

    #[test]
    fn sanity() {
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
}
