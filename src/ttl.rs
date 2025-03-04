// deserialize an rdf turtle document into a rdf Graph

#[cfg(test)]
/// Panics if input is invalid ttl.
pub fn from_ttl(src: &str) -> crate::rdf_graph::Graph {
    use crate::convert::rm_to_om::triple;
    use crate::rdf_graph::Graph;

    use rio_api::parser::TriplesParser;
    use rio_turtle::{TurtleError, TurtleParser};
    use std::io::Cursor;

    let triples = TurtleParser::new(Cursor::new(src), None)
        .into_iter(|t| -> Result<_, TurtleError> { Ok(triple(t).unwrap()) })
        .map(|a| a.unwrap());
    Graph::new(triples)
}

#[cfg(test)]
mod test {
    // use super::*;
    // use crate::types::RdfNode::{Blank, Iri, Literal};

    // #[test]
    // fn sanity() {
    //     let ttl = r#"
    //         @base <http://example.org/> .
    //         @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    //         @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
    //         @prefix foaf: <http://xmlns.com/foaf/0.1/> .
    //         @prefix rel: <http://www.perceive.net/schemas/relationship/> .

    //         <#green-goblin>
    //             rel:enemyOf <#spiderman> ;
    //             a foaf:Person ;    # ipn the context of the Marvel universe
    //             foaf:name "Green Goblin" .
    //         <#spiderman>
    //             rel:enemyOf <#green-goblin> ;
    //             a foaf:Person ;
    //             foaf:name "Spiderman", "Человек-паук"@ru .
    //         [] rel:enemyOf [] .
    //     "#;
    //     assert_eq!(
    //         from_ttl(ttl),
    //         Graph(vec![
    //             [
    //                 Iri("http://example.org/#green-goblin".to_string()),
    //                 Iri("http://www.perceive.net/schemas/relationship/enemyOf".to_string()),
    //                 Iri("http://example.org/#spiderman".to_string())
    //             ],
    //             [
    //                 Iri("http://example.org/#green-goblin".to_string()),
    //                 Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string()),
    //                 Iri("http://xmlns.com/foaf/0.1/Person".to_string())
    //             ],
    //             [
    //                 Iri("http://example.org/#green-goblin".to_string()),
    //                 Iri("http://xmlns.com/foaf/0.1/name".to_string()),
    //                 Literal {
    //                     value: "Green Goblin".to_string(),
    //                     datatype: "http://www.w3.org/2001/XMLSchema#string".to_string(),
    //                     language: None
    //                 }
    //             ],
    //             [
    //                 Iri("http://example.org/#spiderman".to_string()),
    //                 Iri("http://www.perceive.net/schemas/relationship/enemyOf".to_string()),
    //                 Iri("http://example.org/#green-goblin".to_string())
    //             ],
    //             [
    //                 Iri("http://example.org/#spiderman".to_string()),
    //                 Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string()),
    //                 Iri("http://xmlns.com/foaf/0.1/Person".to_string())
    //             ],
    //             [
    //                 Iri("http://example.org/#spiderman".to_string()),
    //                 Iri("http://xmlns.com/foaf/0.1/name".to_string()),
    //                 Literal {
    //                     value: "Spiderman".to_string(),
    //                     datatype: "http://www.w3.org/2001/XMLSchema#string".to_string(),
    //                     language: None
    //                 }
    //             ],
    //             [
    //                 Iri("http://example.org/#spiderman".to_string()),
    //                 Iri("http://xmlns.com/foaf/0.1/name".to_string()),
    //                 Literal {
    //                     value: "Человек-паук".to_string(),
    //                     datatype: "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"
    //                         .to_string(),
    //                     language: Some("ru".to_string())
    //                 }
    //             ],
    //             [
    //                 Blank("b0".to_string()),
    //                 Iri("http://www.perceive.net/schemas/relationship/enemyOf".to_string()),
    //                 Blank("b1".to_string())
    //             ],
    //         ])
    //     );
    // }
}
