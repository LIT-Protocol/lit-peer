use lit_node_testnet::TestSetupBuilder;

#[tokio::test]
async fn create_cache_state_3_0() {
    let _r = TestSetupBuilder::default()
        .num_staked_and_joined_validators(3)
        .build()
        .await;
}

#[tokio::test]
async fn create_cache_state_5_0() {
    let _r = TestSetupBuilder::default()
        .num_staked_and_joined_validators(5)
        .build()
        .await;
}

#[tokio::test]
async fn create_cache_state_6_0() {
    let _r = TestSetupBuilder::default()
        .num_staked_and_joined_validators(6)
        .build()
        .await;
}

#[tokio::test]
async fn create_cache_state_7_0() {
    let _r = TestSetupBuilder::default()
        .num_staked_and_joined_validators(7)
        .build()
        .await;
}

#[tokio::test]
async fn create_cache_state_5_5() {
    let _r = TestSetupBuilder::default()
        .num_staked_and_joined_validators(5)
        .num_staked_only_validators(5)
        .build()
        .await;
}
