use std::collections::HashMap;

use crate::*;

// defines the pyaout type we'll be returning as a part of the royalty standards
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,                       // Required
    pub name: String,                       // Required
    pub symbol: String,                     // Required
    pub icon: Option<String>,               // Data URL
    pub base_uri: Option<String>, 
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<String>,
    pub copies: Option<u64>,
    pub issued_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub starts_at: Option<u64>,
    pub updated_at: Option<u64>,
    pub extra: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
    pub token_type: Option<TokenType>
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    // Owner of the token
    pub owner_id: AccountId,

    // List of approved account IDS that have access to transfer the token.
    pub approved_account_ids: HashMap<AccountId, u64>,

    // the next approval ID to give out
    pub next_approval_id: u64,

    pub royalty: HashMap<AccountId, u32>
}

// The Json token is what will be returned from view calls
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    // Token ID
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
    // List of approved account IDs that have access to transfer the token
    pub approved_account_ids: HashMap<AccountId, u64>,

    pub royalty: HashMap<AccountId, u32>
}

pub trait NonFungibleTokenMetadata {
    // View call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenType {
    Content,
    Badge
}