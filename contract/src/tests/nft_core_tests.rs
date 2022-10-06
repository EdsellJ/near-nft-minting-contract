use super::*;

#[test]
#[should_panic]
fn should_panic_transferring_a_badge_tokne() {
    let context = get_context(false);
    testing_env!(context.clone());

    let mut nft_contract = Contract::new_default_meta(context.current_account_id.clone());

    // Mint a badge nft
    mint_nft_help(&mut nft_contract, &context, "token_id", None, Some(TokenType::Badge));

    // attempt to transfer it
    nft_contract.nft_transfer(
        "receiver.testnet".parse().unwrap(), 
        "token_id".into(), 
        None, 
        None
    )
}