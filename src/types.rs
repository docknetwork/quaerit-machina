pub type Iri = String;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RdfNode {
    Blank(String),
    Iri(Iri),
    Literal {
        value: String,
        datatype: Iri,
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,
    },
}

