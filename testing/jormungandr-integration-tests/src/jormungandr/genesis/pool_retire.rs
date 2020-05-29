use crate::common::{
    jcli_wrapper, jormungandr::ConfigurationBuilder, process_utils, startup,
    transaction_utils::TransactionHash,
};
use chain_core::property::FromStr;
use jormungandr_lib::crypto::hash::Hash;

#[test]
pub fn test_pool_retire() {
    let mut owner = startup::create_new_account_address();
    let other_owner = startup::create_new_account_address();

    let (jormungandr, stake_pools) = startup::start_stake_pool(
        &[owner.clone(), other_owner.clone()],
        &[],
        ConfigurationBuilder::new()
            .with_explorer()
            .with_slot_duration(1)
            .with_slots_per_epoch(5),
    )
    .unwrap();

    startup::sleep_till_next_epoch(10, &jormungandr.config);

    let stake_pool = stake_pools.iter().cloned().next().unwrap();
    let stake_pool_id = Hash::from_str(&stake_pool.id().to_string()).unwrap();
    let mut explorer_stake_pool = jormungandr
        .explorer()
        .get_stake_pool(stake_pool_id)
        .expect("cannot get stake pool from explorer");

    assert!(explorer_stake_pool.retirement.is_none());

    let transaction = owner
        .issue_pool_retire_cert(
            &jormungandr.genesis_block_hash(),
            &jormungandr.fees(),
            &stake_pool.clone().into(),
        )
        .unwrap()
        .encode();

    jcli_wrapper::assert_transaction_in_block(&transaction, &jormungandr);
    startup::sleep_till_epoch(10, 10, &jormungandr.config);

    explorer_stake_pool = jormungandr
        .explorer()
        .get_stake_pool(stake_pool_id)
        .expect("cannot get stake pool from explorer");

    assert!(explorer_stake_pool.retirement.is_some());
    jormungandr.assert_no_errors_in_log();
}

#[test]
pub fn test_retire_the_only_one_pool() {
    let mut owner = startup::create_new_account_address();

    let (jormungandr, stake_pools) = startup::start_stake_pool(
        &[owner.clone()],
        &[],
        ConfigurationBuilder::new().with_explorer(),
    )
    .unwrap();

    process_utils::sleep(5);

    let stake_pool = stake_pools.iter().cloned().next().unwrap();
    let stake_pool_id = Hash::from_str(&stake_pool.id().to_string()).unwrap();

    jormungandr
        .explorer()
        .get_stake_pool(stake_pool_id)
        .expect("cannot get stake pool from explorer");

    let transaction = owner
        .issue_pool_retire_cert(
            &jormungandr.genesis_block_hash(),
            &jormungandr.fees(),
            &stake_pool.clone().into(),
        )
        .unwrap()
        .encode();

    jcli_wrapper::assert_transaction_in_block(&transaction, &jormungandr);
    startup::sleep_till_next_epoch(1, &jormungandr.block0_configuration());

    jormungandr
        .explorer()
        .get_stake_pool(stake_pool_id)
        .expect_err("Explorer should return error when querying retired stake pool");

    assert!(jormungandr
        .logger
        .raw_log_contains_any_of(&["Distribution of rewards will not overflow"])
        .unwrap());
}
