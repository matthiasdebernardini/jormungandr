use self::{
    client::GraphQLClient,
    data::{
        ExplorerLastBlock, ExplorerStakePool, ExplorerTransaction, GraphQLQuery, GraphQLResponse,
    },
};
use jormungandr_lib::crypto::hash::Hash;
use std::convert::TryFrom;

mod client;
mod data;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExplorerError {
    #[error("graph client error")]
    ClientError(#[from] client::GraphQLClientError),
    #[error("json serializiation error")]
    SerializationError(#[from] serde_json::Error),
}

pub struct Explorer {
    client: GraphQLClient,
}

impl Explorer {
    pub fn new<S: Into<String>>(address: S) -> Explorer {
        Explorer {
            client: GraphQLClient::new(address),
        }
    }

    pub fn get_last_block(&self) -> Result<ExplorerLastBlock, ExplorerError> {
        let query = ExplorerLastBlock::query();
        let text = self.run_query(query)?;
        let response: GraphQLResponse =
            serde_json::from_str(&text).map_err(ExplorerError::SerializationError)?;
        ExplorerLastBlock::try_from(response).map_err(|e| e.into())
    }

    pub fn get_transaction(&self, hash: Hash) -> Result<ExplorerTransaction, ExplorerError> {
        let query = ExplorerTransaction::query_by_id(hash);
        let text = self.run_query(query)?;
        let response: GraphQLResponse =
            serde_json::from_str(&text).map_err(ExplorerError::SerializationError)?;
        ExplorerTransaction::try_from(response).map_err(|e| e.into())
    }

    pub fn get_stake_pool(&self, id: Hash) -> Result<ExplorerStakePool, ExplorerError> {
        let query = ExplorerStakePool::query_by_id(id);
        let text = self.run_query(query)?;
        let response: GraphQLResponse =
            serde_json::from_str(&text).map_err(|e| ExplorerError::SerializationError(e))?;
        ExplorerStakePool::try_from(response).map_err(|e| e.into())
    }

    fn run_query(&self, query: GraphQLQuery) -> Result<String, client::GraphQLClientError> {
        let request_response = self.client.run(query)?;
        let text = request_response
            .text()
            .map_err(|e| client::GraphQLClientError::ReqwestError(e))?;
        println!("GraphQLResponse: {:#?}", text);
        Ok(text)
    }
}
