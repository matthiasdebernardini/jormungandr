#![allow(dead_code)]

use super::TestConfig;
use chain_core::mempack;
use chain_impl_mockchain::{block::Block, fee::LinearFee, fragment::Fragment};
use jormungandr_lib::interfaces::{Block0Configuration, NodeConfig, UTxOInfo};
use jormungandr_testing_utils::wallet::Wallet;

use serde::Serialize;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct JormungandrParams<Conf = NodeConfig> {
    node_config: Conf,
    node_config_path: PathBuf,
    genesis_block_path: PathBuf,
    genesis_block_hash: String,
    secret_model_paths: Vec<PathBuf>,
    block0_configuration: Block0Configuration,
    rewards_history: bool,
    log_file_path: PathBuf,
}

impl<Conf: TestConfig> JormungandrParams<Conf> {
    pub(crate) fn new<Secs>(
        node_config: Conf,
        node_config_path: impl Into<PathBuf>,
        genesis_block_path: impl Into<PathBuf>,
        genesis_block_hash: impl Into<String>,
        secret_model_paths: Secs,
        block0_configuration: Block0Configuration,
        rewards_history: bool,
        log_file_path: impl Into<PathBuf>,
    ) -> Self
    where
        Secs: IntoIterator,
        Secs::Item: Into<PathBuf>,
    {
        JormungandrParams {
            node_config,
            node_config_path: node_config_path.into(),
            genesis_block_path: genesis_block_path.into(),
            genesis_block_hash: genesis_block_hash.into(),
            secret_model_paths: secret_model_paths
                .into_iter()
                .map(|item| item.into())
                .collect(),
            block0_configuration,
            rewards_history,
            log_file_path: log_file_path.into(),
        }
    }

    pub fn block0_configuration(&self) -> &Block0Configuration {
        &self.block0_configuration
    }

    pub fn block0_configuration_mut(&mut self) -> &mut Block0Configuration {
        &mut self.block0_configuration
    }

    pub fn genesis_block_path(&self) -> &Path {
        &self.genesis_block_path
    }

    pub fn genesis_block_hash(&self) -> &str {
        &self.genesis_block_hash
    }

    pub fn node_config_path(&self) -> &Path {
        &self.node_config_path
    }

    pub fn rewards_history(&self) -> bool {
        self.rewards_history
    }

    pub fn log_file_path(&self) -> &Path {
        &self.log_file_path
    }

    pub fn secret_model_paths(&self) -> impl Iterator<Item = &Path> {
        self.secret_model_paths.iter().map(|b| b.as_path())
    }

    pub fn rest_uri(&self) -> String {
        format!("http://{}/api", self.node_config.rest_socket_addr())
    }

    pub fn node_config(&self) -> &Conf {
        &self.node_config
    }

    pub fn node_config_mut(&mut self) -> &mut Conf {
        &mut self.node_config
    }

    fn regenerate_ports(&mut self) {
        self.node_config.set_rest_socket_addr(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            super::get_available_port(),
        ));
        self.node_config.set_p2p_public_address(
            format!("/ip4/127.0.0.1/tcp/{}", super::get_available_port())
                .parse()
                .unwrap(),
        );
    }

    pub fn fees(&self) -> LinearFee {
        self.block0_configuration
            .blockchain_configuration
            .linear_fees
    }

    pub fn get_p2p_listen_port(&self) -> u16 {
        let address = self.node_config.p2p_listen_address().to_string();
        let tokens: Vec<&str> = address.split('/').collect();
        assert_eq!(
            tokens.get(3),
            Some(&"tcp"),
            "expected a tcp part in p2p.public_address"
        );
        let port_str = tokens
            .get(4)
            .expect("cannot extract port from p2p.public_address");
        port_str.parse().unwrap()
    }

    pub fn block0_utxo(&self) -> Vec<UTxOInfo> {
        let block0_bytes = std::fs::read(self.genesis_block_path()).expect(&format!(
            "Failed to load block 0 binary file '{}'",
            self.genesis_block_path().display()
        ));
        mempack::read_from_raw::<Block>(&block0_bytes)
            .expect(&format!(
                "Failed to parse block in block 0 file '{}'",
                self.genesis_block_path().display()
            ))
            .contents
            .iter()
            .filter_map(|fragment| match fragment {
                Fragment::Transaction(transaction) => Some((transaction, fragment.hash())),
                _ => None,
            })
            .map(|(transaction, fragment_id)| {
                transaction
                    .as_slice()
                    .outputs()
                    .iter()
                    .enumerate()
                    .map(move |(idx, output)| {
                        UTxOInfo::new(
                            fragment_id.into(),
                            idx as u8,
                            output.address.clone().into(),
                            output.value.into(),
                        )
                    })
            })
            .flatten()
            .collect()
    }

    pub fn block0_utxo_for_address(&self, wallet: &Wallet) -> UTxOInfo {
        let utxo = self
            .block0_utxo()
            .into_iter()
            .find(|utxo| *utxo.address() == wallet.address())
            .expect(&format!(
                "No UTxO found in block 0 for address '{:?}'",
                wallet
            ));
        println!(
            "Utxo found for address {}: {:?}",
            wallet.address().to_string(),
            &utxo
        );
        utxo
    }
}

impl<Conf: TestConfig + Serialize> JormungandrParams<Conf> {
    pub fn write_node_config(&self) {
        let mut output_file = File::create(&self.node_config_path).unwrap();
        serde_yaml::to_writer(&mut output_file, &self.node_config)
            .expect("cannot serialize node config");
    }

    pub fn refresh_instance_params(&mut self) {
        self.regenerate_ports();
        self.write_node_config();
    }
}
