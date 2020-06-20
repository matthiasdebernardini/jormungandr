use super::{Explorer, LayersConfig, Log, Mempool, Policy, Rest, TopicsOfInterest};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2p {
    /// The public address to which other peers may connect to
    pub public_address: poldercast::Address,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_id: Option<poldercast::Id>,

    /// the rendezvous points for the peer to connect to in order to initiate
    /// the p2p discovery from.
    pub trusted_peers: Vec<TrustedPeer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<poldercast::Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_inbound_connections: Option<u32>,

    pub allow_private_addresses: bool,

    pub topics_of_interest: Option<TopicsOfInterest>,

    pub policy: Option<Policy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<LayersConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedPeer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub address: poldercast::Address,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<PathBuf>,
    pub rest: Rest,
    pub p2p: P2p,
    pub log: Option<Log>,
    pub explorer: Explorer,
    pub mempool: Option<Mempool>,
    pub bootstrap_from_trusted_peers: Option<bool>,
    pub skip_bootstrap: Option<bool>,
}
