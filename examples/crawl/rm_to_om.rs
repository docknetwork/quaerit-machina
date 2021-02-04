use core::fmt;
use oxigraph::model as om;
use rio_api::model as rm;
use tap::pipe::Pipe as _;

#[derive(Debug)]
pub enum ConvertErr {
    BadIri(om::IriParseError),
    BadBlank(om::BlankNodeIdParseError),
    BadTag(om::LanguageTagParseError),
}

impl fmt::Display for ConvertErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ConvertErr {}

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
