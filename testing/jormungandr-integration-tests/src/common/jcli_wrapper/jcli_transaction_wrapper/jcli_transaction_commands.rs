#![allow(dead_code)]

use std::path::Path;
use std::process::Command;

use crate::common::configuration;

use chain_impl_mockchain::fee::LinearFee;

#[derive(Default, Debug)]
pub struct TransactionCommands {}

impl TransactionCommands {
    pub fn new() -> TransactionCommands {
        TransactionCommands {}
    }

    pub fn get_new_transaction_command(&self, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("new")
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_add_input_command(
        &self,
        tx_id: &str,
        tx_index: u8,
        amount: &str,
        staging_file: &Path,
    ) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("add-input")
            .arg(&tx_id)
            .arg(tx_index.to_string())
            .arg(&amount)
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_add_account_command(
        &self,
        account_addr: &str,
        amount: &str,
        staging_file: &Path,
    ) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("add-account")
            .arg(account_addr.to_string())
            .arg(amount)
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_add_certificate_command(&self, certificate: &str, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("add-certificate")
            .arg(certificate.to_string())
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_add_output_command(&self, addr: &str, amount: &str, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("add-output")
            .arg(&addr)
            .arg(amount)
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_finalize_command(&self, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("finalize")
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_finalize_with_fee_command<P: AsRef<Path>>(
        &self,
        address: &str,
        linear_fees: &LinearFee,
        staging_file: P,
    ) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("finalize")
            .arg(address)
            .arg("--fee-certificate")
            .arg(linear_fees.certificate.to_string())
            .arg("--fee-coefficient")
            .arg(linear_fees.coefficient.to_string())
            .arg("--fee-constant")
            .arg(linear_fees.constant.to_string())
            .arg("--staging")
            .arg(staging_file.as_ref());
        command
    }

    pub fn get_make_witness_command(
        &self,
        block0_hash: &str,
        tx_id: &str,
        addr_type: &str,
        spending_account_counter: Option<u32>,
        witness_file: &Path,
        witness_key: &Path,
    ) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());

        let spending_counter = match spending_account_counter {
            Some(value) => value,
            None => 0,
        };

        command
            .arg("transaction")
            .arg("make-witness")
            .arg("--genesis-block-hash")
            .arg(block0_hash)
            .arg("--type")
            .arg(&addr_type)
            .arg(&tx_id)
            .arg(witness_file)
            .arg("--account-spending-counter")
            .arg(spending_counter.to_string())
            .arg(witness_key);
        command
    }

    pub fn get_add_witness_command(&self, witness_file: &Path, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("add-witness")
            .arg(witness_file)
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_seal_command(&self, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("seal")
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_auth_command(&self, signing_key: &Path, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("auth")
            .arg("--staging")
            .arg(staging_file)
            .arg("--key")
            .arg(&signing_key);
        command
    }

    pub fn get_transaction_message_to_command(&self, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("to-message")
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_transaction_id_command(&self, staging_file: &Path) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("data-for-witness")
            .arg("--staging")
            .arg(staging_file);
        command
    }

    pub fn get_transaction_info_command<P: AsRef<Path>>(
        &self,
        format: &str,
        staging_file: P,
    ) -> Command {
        let mut command = Command::new(configuration::get_jcli_app());
        command
            .arg("transaction")
            .arg("info")
            .arg("--format")
            .arg(format)
            .arg("--staging")
            .arg(staging_file.as_ref());
        command
    }
}
