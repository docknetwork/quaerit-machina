extern crate alloc;
extern crate core;

mod ipfs_resolver;
mod parse;
mod resolve;
mod rm_to_om;

use crate::ipfs_resolver::create_with_tokio;
use resolve::Resolve;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(required = true)]
    iri: Vec<String>,
}

#[tokio::main]
async fn main() {
    let mut resolver = create_with_tokio().await.unwrap();
    for iri in Args::from_args().iri {
        let doc = resolver.lookup(&iri).await.unwrap();
        let graph = parse::into_rdf(&doc, "text/turtle; charset=utf-8").unwrap();
        dbg!(graph);
    }
}
