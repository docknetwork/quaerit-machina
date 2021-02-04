extern crate core;

use crate::resolve::{Resolve, ResolveError};
use async_trait::async_trait;
use futures::TryStreamExt;
use ipfs_api::IpfsClient;

#[async_trait]
impl Resolve for IpfsClient {
    async fn lookup(&mut self, iri: &str) -> Result<Vec<u8>, ResolveError> {
        self.cat(iri.strip_prefix("ipfs://ipfs/").ok_or("bad prefix")?)
            .map_ok(|a| a.to_vec())
            .try_concat()
            .await
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn resolve() {
        let payload = include_bytes!("rdf-schema");
        let mut ic = IpfsClient::default();
        ic.add(std::io::Cursor::new(payload)).await.unwrap();
        let iri = "ipfs://ipfs/QmcmYSJHVD9Cqo24LSjJjd8cAVrqdG9FouxXMVVjKzuZKX";
        let doc = timeout(tokio::time::Duration::from_secs(5), ic.lookup(iri))
            .await
            .expect("timeout while retrieving document")
            .unwrap();
        assert_eq!(doc, include_bytes!("rdf-schema"));
    }
}
