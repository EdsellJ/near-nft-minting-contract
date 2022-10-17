use super::*;

#[test]
fn should_mint_nft() {
    let context = get_context(false);
    testing_env!(context.clone());

    let mut nft_contract = Contract::new_default_meta(context.current_account_id);
    nft_contract.nft_mint(
        String::from("token_id"), 
        TokenMetadata { 
            title: None, 
            description: None, 
            media: None, 
            media_hash: None, 
            copies: None, 
            issued_at: None, 
            expires_at: None, 
            starts_at: None, 
            updated_at: None, 
            extra: None, 
            reference: None, 
            reference_hash: None,
            token_type: None
        },
        context.signer_account_id.clone(), 
        None
    );
    let nft_count = nft_contract.nft_token(String::from("token_id"));
    // check to see that an nft was created and returned
    assert!(nft_count.is_some());

    let nft = nft_count.unwrap();
    
    assert_eq!(nft.token_id, String::from("token_d"));
    assert_eq!(nft.owner_id, context.signer_account_id);
    assert!(nft.metadata.token_type.is_some());
    assert!(nft.metadata.issued_at.is_some());

}

#[test]
#[should_panic]
fn should_panic_token_already_exists() {
    let context = get_context(false);
    testing_env!(context.clone());

    let mut nft_contract = Contract::new_default_meta(context.current_account_id.clone());

    mint_nft_help(&mut nft_contract, &context, "token_id", None, None);

    // mint nft again, same id
    mint_nft_help(&mut nft_contract, &context, "token_id", None, None);
}

#[test]
#[should_panic]
fn should_panic_no_attached_deposit() {
    let context = get_context_no_attached_deposit(false);
    testing_env!(context.clone());
    let mut nft_contract = Contract::new_default_meta(context.current_account_id.clone());

    // This will fail
    mint_nft_help(&mut nft_contract, &context, "token_id", None, None);
}

#[test]
#[should_panic]
fn should_panic_attached_seven_royalties_accounts() {
    let context = get_context(false);
    testing_env!(context.clone());
    let mut nft_contract = Contract::new_default_meta(context.current_account_id.clone());

    // Create Hashmap with 8 entries
    let royalties: HashMap<AccountId, u32> = [
        ("account1".parse().unwrap(), 2),
        ("account2".parse().unwrap(), 2),
        ("account3".parse().unwrap(), 2),
        ("account4".parse().unwrap(), 2),
        ("account5".parse().unwrap(), 2),
        ("account6".parse().unwrap(), 2),
        ("account7".parse().unwrap(), 2),
        ("account8".parse().unwrap(), 2),
    ].iter().cloned().collect::<HashMap<_,_>>();
            
    mint_nft_help(&mut nft_contract, &context, "token_id", Some(royalties), None);
}

#[test]
fn should_add_all_royalty_accounts() {
    let context = get_context(false);
    testing_env!(context.clone());
    let mut nft_contract = Contract::new_default_meta(context.current_account_id.clone());

    // Create Hashmap with 8 entries
    let royalties: HashMap<AccountId, u32> = [
        ("account1".parse().unwrap(), 2),
        ("account2".parse().unwrap(), 2),
    ].iter().cloned().collect::<HashMap<_,_>>();
    
    mint_nft_help(&mut nft_contract, &context, "token_id", Some(royalties), None);

    assert_eq!(nft_contract.nft_supply_for_owner(context.signer_account_id.clone()), U128(1));

    let token = nft_contract.nft_token("token_id".to_string());
    assert!(token.is_some());
    let token = token.unwrap();

    assert_eq!(token.royalty.get(&"account1".parse().unwrap()).unwrap(), &2);
    assert_eq!(token.royalty.get(&"account2".parse().unwrap()).unwrap(), &2);

}