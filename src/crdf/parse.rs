use crate::resolve::ResolveError;
use crate::rm_to_om::triple;
use oxigraph::model::Triple;
use quaerit_machina::Graph;
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleParser;
use std::io::Cursor;
use tap::prelude::*;

pub fn into_rdf(body: &[u8], content_type: &str) -> Result<Graph, ResolveError> {
    fn parse<P>(p: P) -> Result<Graph, ResolveError>
    where
        P: TriplesParser,
        <P as TriplesParser>::Error: 'static,
    {
        p.into_iter(|t| -> Result<_, ResolveError> { Ok(triple(t)?) })
            .collect::<Result<Vec<Triple>, ResolveError>>()?
            .into_iter()
            .pipe(Graph::new)
            .pipe(Ok)
    }

    match content_type {
        "text/turtle; charset=utf-8" => parse(TurtleParser::new(Cursor::new(body), None)),
        _ => Err("unknown content-type".into()),
    }
}
