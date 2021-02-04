extern crate core;

mod rm_to_om;

use async_trait::async_trait;
use core::fmt::Debug;
use oxigraph::io::DatasetFormat;
use oxigraph::model as om;
use oxigraph::model::NamedNode;
use oxigraph::MemoryStore;
use quaerit_machina::LookupError;
use quaerit_machina::{Agent, Curiosity, Graph, Lookup};
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleParser;
use rm_to_om::triple;
use std::collections::BTreeMap;
use std::io::Cursor;
use tap::prelude::*;

type Berr = Box<dyn std::error::Error>;

const START_URL: &str = "http://www.w3.org/2000/01/rdf-schema";

#[tokio::main]
async fn main() {
    let start_url = std::env::args()
        .skip(1)
        .next()
        .unwrap_or(START_URL.to_string());
    let store = MemoryStore::new();
    let mut agent = Agent::new(curiosity(), store.clone(), CachedHttp::default());
    agent
        .investigate(NamedNode::new(start_url).unwrap())
        .await
        .unwrap();
    while !agent.next().await.unwrap() {
        dbg!(store.len());
    }
    println!("{}", show(&store));
    dbg!(store.len(), "done");
}

async fn http_lookup(url: &Url) -> Result<Graph, Berr> {
    let resp = reqwest::get(url.as_str()).await?;
    if !resp.status().is_success() {
        return Err("unsucsessful GET".into());
    }
    let content_type = resp
        .headers()
        .get(CONTENT_TYPE)
        .ok_or("no content-type header in response")?
        .to_str()?
        .to_string();
    let bytes = resp.bytes().await?;
    into_rdf(&bytes, &content_type).map_err(Into::into)
}

fn into_rdf(body: &[u8], content_type: &str) -> Result<Graph, Berr> {
    fn parse<P>(p: P) -> Result<Graph, Berr>
    where
        P: TriplesParser,
        <P as TriplesParser>::Error: 'static,
    {
        p.into_iter(|t| -> Result<_, Berr> { Ok(triple(t)?) })
            .collect::<Result<Vec<om::Triple>, Berr>>()?
            .into_iter()
            .pipe(Graph::new)
            .pipe(Ok)
    }

    match content_type {
        "text/turtle; charset=utf-8" => parse(TurtleParser::new(Cursor::new(body), None)),
        _ => Err("unknown content-type".into()),
    }
}

#[derive(Default)]
struct CachedHttp {
    cache: BTreeMap<Url, Graph>,
}

#[async_trait]
impl Lookup for CachedHttp {
    async fn lookup(&mut self, iri: &NamedNode) -> Result<Graph, LookupError> {
        let url: Url = iri.as_str().parse().map_err(debg)?;
        if let Some(g) = self.cache.get(&url) {
            return Ok(g.clone());
        }
        let ret = http_lookup(&url).await.map_err(debg)?;
        self.cache.insert(url, ret.clone());
        Ok(ret)
    }
}

fn curiosity() -> Curiosity {
    Curiosity::create(
        [
            "SELECT DISTINCT ?s WHERE { GRAPH ?g { ?s ?p ?o } }",
            "SELECT DISTINCT ?p WHERE { GRAPH ?g { ?s ?p ?o } }",
            "SELECT DISTINCT ?o WHERE { GRAPH ?g { ?s ?p ?o } }",
            "SELECT DISTINCT ?g WHERE { GRAPH ?g { ?s ?p ?o } }",
        ]
        .iter()
        .map(|a| a.parse().unwrap())
        .collect(),
    )
    .unwrap()
}

fn show(store: &MemoryStore) -> String {
    let mut writer = std::io::Cursor::new(Vec::<u8>::new());
    store
        .dump_dataset(&mut writer, DatasetFormat::NQuads)
        .unwrap();
    String::from_utf8(writer.into_inner()).unwrap()
}

fn debg(t: impl Debug) -> String {
    format!("{:?}", t)
}
