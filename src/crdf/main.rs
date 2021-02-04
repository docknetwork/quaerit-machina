//! In persuit of a quicker release, the current plan is to only support turtle documents stored on
//! ipfs. That obvates this program.

extern crate alloc;
extern crate core;

mod ipfs_client_resolver;
mod parse;
mod resolve;
mod rm_to_om;

use ipfs_api::IpfsClient;
use resolve::Resolve;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(required = true)]
    iri: Vec<String>,
}

#[tokio::main]
async fn main() {
    let mut resolver = IpfsClient::default();
    for iri in Args::from_args().iri {
        let doc = resolver.lookup(&iri).await.unwrap();
        let graph = parse::into_rdf(&doc, "text/turtle; charset=utf-8").unwrap();
        dbg!(graph);
    }
}
