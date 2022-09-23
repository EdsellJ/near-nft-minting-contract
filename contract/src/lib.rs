use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;

mod approval;
mod enumeration;
mod metadata;
mod mint;
mod nft_core;
mod internal;
mod royalty;
mod events;

pub type TokenId = String;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

// Helper structure for keys of the persistne collections
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Contract owner
    pub owner_id: AccountId,

    // keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    // keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    // keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    // Keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /*
        init function (can only be called once).
        This will init the contract with default metadata so the user
        doesnt have to manually type metadata
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata { 
                spec: "nft-1.0.0".to_string(),
                name: "Educoin_Near_Minting_Serivce".to_string(),
                symbol: "GOT".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None
            }
        )
    }

    /*
        init function (can only be called once)
        this inits the contract with metadata that was passed in and the owner_id
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        // Create a variable of type self with all the fields init
        let this = Self {
            // Storage keys are simply the prefizes used for the collections. 
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap()
            ),
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata)
            )
        };
        this
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(all(test, not(target_arch="wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .current_account_id("nft_contract".parse().unwrap())
            .signer_account_id("bob_near".parse().unwrap())
            .account_balance(1000000)
            .attached_deposit(near_sdk::ONE_NEAR)
            .is_view(is_view)
            .build()
    }

    fn mint_nft_help(nft_contract: &mut Contract, context: &VMContext, token_id: &str) {
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
                reference_hash: None 
            },
            context.signer_account_id.clone(), 
            None
        );
    }

    #[test]
    #[should_panic]
    fn should_panic_contract_not_initialized() {
        let context = get_context(false);
        testing_env!(context);

        Contract::default(); // This does not init
    }

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
                reference_hash: None 
            },
            context.signer_account_id.clone(), 
            None
        );
        let nft_count = nft_contract.nft_token(String::from("token_id"));
        // check to see that an nft was created and returned
        assert!(nft_count.is_some());

        let nft = nft_count.unwrap();
        assert_eq!(nft.token_id, String::from("token_id"));
        assert_eq!(nft.owner_id, context.signer_account_id);
    
    }

    #[test]
    #[should_panic]
    fn should_panic_token_already_exists() {
        let context = get_context(false);
        testing_env!(context.clone());

        let mut nft_contract = Contract::new_default_meta(context.current_account_id.clone());

        mint_nft_help(&mut nft_contract, &context, "token_id");

        // mint nft again, same id
        mint_nft_help(&mut nft_contract, &context, "token_id");
    }



}
