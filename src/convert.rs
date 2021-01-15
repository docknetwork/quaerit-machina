use crate::types::RdfNode;
use rio_api::model::BlankNode;
use rio_api::model::Literal;
use rio_api::model::NamedNode;
use rio_api::model::NamedOrBlankNode;
use rio_api::model::Term;
use rio_api::model::Triple;

pub fn from_rio_triple(t: Triple<'_>) -> [RdfNode; 3] {
    [
        from_nob(t.subject),
        RdfNode::Iri(t.predicate.iri.to_string()),
        from_term(t.object),
    ]
}

fn from_nob(n: NamedOrBlankNode) -> RdfNode {
    match n {
        NamedOrBlankNode::NamedNode(NamedNode { iri }) => RdfNode::Iri(iri.to_string()),
        NamedOrBlankNode::BlankNode(BlankNode { id }) => RdfNode::Blank(id.to_string()),
    }
}

fn from_term(n: Term) -> RdfNode {
    match n {
        Term::NamedNode(NamedNode { iri }) => RdfNode::Iri(iri.to_string()),
        Term::BlankNode(BlankNode { id }) => RdfNode::Blank(id.to_string()),
        Term::Literal(Literal::Simple { value }) => RdfNode::Literal {
            value: value.to_string(),
            datatype: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            language: None,
        },
        Term::Literal(Literal::LanguageTaggedString { value, language }) => RdfNode::Literal {
            value: value.to_string(),
            datatype: "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_string(),
            language: Some(language.to_string()),
        },
        Term::Literal(Literal::Typed { value, datatype }) => RdfNode::Literal {
            value: value.to_string(),
            datatype: datatype.iri.to_string(),
            language: None,
        },
    }
}
