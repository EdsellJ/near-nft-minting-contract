#[cfg(all(test, not(target_arch="wasm32")))]
pub mod enumeration_tests;
pub mod lib_tests;


fn get_context(is_view: bool) -> VMContext {
    VMContextBuilder::new()
        .current_account_id("nft_contract".parse().unwrap())
        .signer_account_id("bob_near".parse().unwrap())
        .account_balance(1000000)
        .attached_deposit(near_sdk::ONE_NEAR)
        .is_view(is_view)
        .build()
}