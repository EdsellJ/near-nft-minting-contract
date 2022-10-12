use std::collections::HashMap;

use super::*;
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, VMContext};

pub mod lib_tests;
pub mod mint_tests;
pub mod enumeration_tests;
pub mod nft_core_tests;


/* 
    Helper functions for tests
*/

fn get_context(is_view: bool) -> VMContext {
    VMContextBuilder::new()
        .current_account_id("nft_contract".parse().unwrap())
        .signer_account_id("bob_near".parse().unwrap())
        .account_balance(1000000)
        .attached_deposit(near_sdk::ONE_NEAR)
        .is_view(is_view)
        .build()
}

fn get_context_no_attached_deposit(is_view: bool) -> VMContext {
    VMContextBuilder::new()
        .current_account_id("nft_contract".parse().unwrap())
        .signer_account_id("bob_near".parse().unwrap())
        .account_balance(near_sdk::ONE_NEAR)
        .is_view(is_view)
        .build()
}

fn mint_nft_help(
    nft_contract: &mut Contract, 
    context: &VMContext, 
    token_id: &str, 
    royalties: Option<HashMap<AccountId, u32>>, 
    token_type: Option<TokenType>
) {
    nft_contract.nft_mint(
        String::from(token_id), 
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
            token_type
        },
        context.signer_account_id.clone(), 
        royalties
    );
}