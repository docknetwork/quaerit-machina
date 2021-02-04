extern crate core;

use crate::resolve::{Resolve, ResolveError};
use async_trait::async_trait;
use core::str::FromStr;
use futures::stream::TryStreamExt;
use ipfs::Ipfs;
use ipfs::IpfsOptions;
use ipfs::IpfsPath;
use ipfs::IpfsTypes;
use ipfs::Keypair;
use ipfs::Multiaddr;
use ipfs::PeerId;

/// spawn an ipfs node with somthing like default settings.
///
/// Panics:
///
/// Panics if not run from within a tokio runtime.
pub async fn create_with_tokio() -> Result<Ipfs<ipfs::Types>, ipfs::Error> {
    let opts = IpfsOptions {
        ipfs_path: std::env::temp_dir(),
        keypair: Keypair::generate_ed25519(),
        mdns: true,
        bootstrap: [
            "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
            "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
            "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
            "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
            "/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
            "/ip4/104.131.131.82/udp/4001/quic/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
        ]
        .iter()
        .cloned()
        .map(|addr| {
            (
                addr.parse::<Multiaddr>().unwrap(),
                addr.rsplit('/').next().unwrap().parse::<PeerId>().unwrap(),
            )
        })
        .collect(),
        kad_protocol: Some("/ipfs/kad/1.0.0".to_string()),
        listening_addrs: [
            "/ip4/0.0.0.0/tcp/0",
            "/ip4/0.0.0.0/udp/0/quic",
            "/ip6/::1/tcp/0",
            "/ip6/::1/udp/0/quic",
            "/p2p-circuit",
        ]
        .iter()
        .cloned()
        .map(|addr| addr.parse().unwrap())
        .collect(),
        span: None,
    };
    let (resolver, resolver_driver) = ipfs::UninitializedIpfs::<ipfs::Types>::new(opts)
        .start()
        .await?;
    tokio::spawn(resolver_driver);
    Ok(resolver)
}

#[async_trait]
impl<T: IpfsTypes> Resolve for Ipfs<T> {
    async fn lookup(&mut self, iri: &str) -> Result<Vec<u8>, ResolveError> {
        let path = IpfsPath::from_str(iri.strip_prefix("ipfs://ipfs/").ok_or("bad prefix")?)?;
        let stream = self.cat_unixfs(path, None).await?;
        stream.try_concat().await.map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn resolve() {
        let mut ic = create_with_tokio().await.unwrap();
        let iri = "ipfs://ipfs/QmcmYSJHVD9Cqo24LSjJjd8cAVrqdG9FouxXMVVjKzuZKX";
        let doc = timeout(tokio::time::Duration::from_secs(10), ic.lookup(iri))
            .await
            .expect("timeout while retrieving document")
            .unwrap();
        assert_eq!(doc, include_bytes!("rdf-schema"));
    }
}

// The following are links to the same document, QmVk is a directory containing
// the file rdf-schema.ttl .
// - ipfs://ipfs/QmVkiRsmcuJ22vBHrw1r5CCQVYsrfwbhoBnmaXZ3UEztBa/rdf-schema.ttl
// - ipfs://ipfs/QmcmYSJHVD9Cqo24LSjJjd8cAVrqdG9FouxXMVVjKzuZKX
