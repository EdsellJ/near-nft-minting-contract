use super::*;

    
#[test]
#[should_panic]
fn should_panic_contract_not_initialized() {
    let context = get_context(false);
    testing_env!(context);

    Contract::default(); // This does not init
}

#[test]
fn should_create_contract_with_new_method() {
    let context = get_context(false);
    testing_env!(context.clone());

    let nft_contract = Contract::new(context.current_account_id.clone(), NFTContractMetadata { 
        spec: "nft-1.0.0".to_string(),
        name: "Educoin_Near_Minting_Serivce".to_string(),
        symbol: "GOT".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None
    });

    // This should not panic
    nft_contract.nft_total_supply();
}