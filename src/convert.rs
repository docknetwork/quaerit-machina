use oxigraph::model::{BlankNode, NamedOrBlankNode, Term};

pub trait AsBlank {
    fn as_blank(&self) -> Option<&BlankNode>;
    fn as_blank_mut(&mut self) -> Option<&mut BlankNode>;
}

impl AsBlank for Term {
    fn as_blank(&self) -> Option<&BlankNode> {
        match self {
            Term::BlankNode(b) => Some(b),
            _ => None,
        }
    }

    fn as_blank_mut(&mut self) -> Option<&mut BlankNode> {
        match self {
            Term::BlankNode(b) => Some(b),
            _ => None,
        }
    }
}

impl AsBlank for NamedOrBlankNode {
    fn as_blank(&self) -> Option<&BlankNode> {
        match self {
            NamedOrBlankNode::BlankNode(b) => Some(b),
            _ => None,
        }
    }

    fn as_blank_mut(&mut self) -> Option<&mut BlankNode> {
        match self {
            NamedOrBlankNode::BlankNode(b) => Some(b),
            _ => None,
        }
    }
}

/// rio_api_model to oxigraph model
pub mod rm_to_om {
    use oxigraph::model as om;
    use rio_api::model as rm;
    use tap::pipe::Pipe as _;

    #[derive(Debug)]
    pub enum ConvertErr {
        BadIri(om::IriParseError),
        BadBlank(om::BlankNodeIdParseError),
        BadTag(om::LanguageTagParseError),
    }

    pub fn triple(tr: rm::Triple) -> Result<om::Triple, ConvertErr> {
        let rm::Triple {
            subject,
            predicate,
            object,
        } = tr;
        om::Triple {
            subject: named_or_blank_node(subject)?,
            predicate: named_node(predicate)?,
            object: term(object)?,
        }
        .pipe(Ok)
    }

    fn term(tm: rm::Term) -> Result<om::Term, ConvertErr> {
        match tm {
            rm::Term::NamedNode(nn) => om::Term::NamedNode(named_node(nn)?),
            rm::Term::BlankNode(bn) => om::Term::BlankNode(blank_node(bn)?),
            rm::Term::Literal(lt) => om::Term::Literal(literal(lt)?),
        }
        .pipe(Ok)
    }

    fn literal(lt: rm::Literal) -> Result<om::Literal, ConvertErr> {
        match lt {
            rm::Literal::LanguageTaggedString { value, language } => {
                om::Literal::new_language_tagged_literal(value, language)?
            }
            rm::Literal::Simple { value } => om::Literal::new_simple_literal(value),
            rm::Literal::Typed { value, datatype } => {
                om::Literal::new_typed_literal(value, named_node(datatype)?)
            }
        }
        .pipe(Ok)
    }

    fn named_or_blank_node(nb: rm::NamedOrBlankNode) -> Result<om::NamedOrBlankNode, ConvertErr> {
        match nb {
            rm::NamedOrBlankNode::NamedNode(nn) => om::NamedOrBlankNode::NamedNode(named_node(nn)?),
            rm::NamedOrBlankNode::BlankNode(bn) => om::NamedOrBlankNode::BlankNode(blank_node(bn)?),
        }
        .pipe(Ok)
    }

    fn named_node(nb: rm::NamedNode) -> Result<om::NamedNode, om::IriParseError> {
        om::NamedNode::new(nb.iri)
    }

    fn blank_node(nb: rm::BlankNode) -> Result<om::BlankNode, om::BlankNodeIdParseError> {
        om::BlankNode::new(nb.id)
    }

    impl From<om::IriParseError> for ConvertErr {
        fn from(other: om::IriParseError) -> Self {
            ConvertErr::BadIri(other)
        }
    }

    impl From<om::BlankNodeIdParseError> for ConvertErr {
        fn from(other: om::BlankNodeIdParseError) -> Self {
            ConvertErr::BadBlank(other)
        }
    }

    impl From<om::LanguageTagParseError> for ConvertErr {
        fn from(other: om::LanguageTagParseError) -> Self {
            ConvertErr::BadTag(other)
        }
    }
}

// use crate::types::RdfNode;
// use rio_api::model::BlankNode;
// use rio_api::model::Literal;
// use rio_api::model::NamedNode;
// use rio_api::model::NamedOrBlankNode;
// use rio_api::model::Term;
// use rio_api::model::Triple;

// pub fn from_rio_triple(t: Triple<'_>) -> [RdfNode; 3] {
//     [
//         from_nob(t.subject),
//         RdfNode::Iri(t.predicate.iri.to_string()),
//         from_term(t.object),
//     ]
// }

// fn from_nob(n: NamedOrBlankNode) -> RdfNode {
//     match n {
//         NamedOrBlankNode::NamedNode(NamedNode { iri }) => RdfNode::Iri(iri.to_string()),
//         NamedOrBlankNode::BlankNode(BlankNode { id }) => RdfNode::Blank(id.to_string()),
//     }
// }

// fn from_term(n: Term) -> RdfNode {
//     match n {
//         Term::NamedNode(NamedNode { iri }) => RdfNode::Iri(iri.to_string()),
//         Term::BlankNode(BlankNode { id }) => RdfNode::Blank(id.to_string()),
//         Term::Literal(Literal::Simple { value }) => RdfNode::Literal {
//             value: value.to_string(),
//             datatype: "http://www.w3.org/2001/XMLSchema#string".to_string(),
//             language: None,
//         },
//         Term::Literal(Literal::LanguageTaggedString { value, language }) => RdfNode::Literal {
//             value: value.to_string(),
//             datatype: "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_string(),
//             language: Some(language.to_string()),
//         },
//         Term::Literal(Literal::Typed { value, datatype }) => RdfNode::Literal {
//             value: value.to_string(),
//             datatype: datatype.iri.to_string(),
//             language: None,
//         },
//     }
// }
