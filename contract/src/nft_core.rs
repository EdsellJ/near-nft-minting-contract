use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult};
use std::collections::HashMap;
const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(25_000_000_000_000);

pub trait NonFungibleTokenCore {
    // Transfers an NFT to a receiver ID
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>
    );

    // Transfers an NFT to a receiver and calls a function ont the receiver ID's contract
    // Returns 'true' if the token was transferred from the sender's account
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;

    // Get info about the NFT token passed in
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
}

#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    // Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
    // Returns 'true' if the token should be returned back to the sender
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String
    ) -> Promise;
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    /*
        Resolves the promise of the cross contract call to the receiver contract
        this is troed on THIS contract and is meant to analyze what happened in the cross contract call when
        nft_on_transfer was called
        as part of the nft_transfer_call method
    */
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>
    ) -> bool;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    #[payable]
    // Transfers an NFT to a receiver ID
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>
    ) {
        // assert that the user attached exacly 1 yoctoNEAR, this is for security and so that the user will be
        // redirected to the NEAR wallet
        assert_one_yocto();
        // Get the sender sending the token
        let sender_id = env::predecessor_account_id();

        // Call the internal transfer method
        let previous_token = self.internal_transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            approval_id,
            memo
        );

        // refund the owner for releasing the storage used up by the approved account IDs
        refund_approved_account_ids(
            previous_token.owner_id.clone(), 
            &previous_token.approved_account_ids,
        );
    }

    // Transfers an NFT to a receiver and calls a function on the receiver ID's contract
    // Returns 'true' if the token was transferred from the sender's account
    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        // assert that the user attached exactly 1 yocto for security
        assert_one_yocto();

        // get the sender ID
        let sender_id = env::predecessor_account_id();

        // transfer the token and get the previous token object
        let previous_token = self.internal_transfer(
            &sender_id, 
            &receiver_id, 
            &token_id, 
            approval_id,
            memo.clone()
        );

        // default the authorized_id to none
        let mut authorized_id = None;
        // if the sender isn't the owner of the token, we set the authorized ID equal to the sender
        if sender_id != previous_token.owner_id {
            authorized_id = Some(sender_id.to_string());
        }

        // init receiver's call and the callback
        // defaulting gas weight to 1, no attached deposit, and static GAS equal to the GAS for nft on transfer
        ext_non_fungible_token_receiver::ext(receiver_id.clone())
            .with_static_gas(GAS_FOR_NFT_ON_TRANSFER)
            .nft_on_transfer(
                sender_id, 
                previous_token.owner_id.clone(), 
                token_id.clone(), 
                msg
            )
            // resolve the promise and call nft_resolve_transfer
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
                    .nft_resolve_transfer(
                        authorized_id,
                        previous_token.owner_id,
                        receiver_id, 
                        token_id,
                        previous_token.approved_account_ids,
                        memo,
                    )
            ).into()
    }

    // Get info about the NFT token passed in
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        // if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            // get the metadata for that token
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();

            // return the JsonToken
            Some(JsonToken {
                token_id,
                owner_id: token.owner_id,
                metadata,
                approved_account_ids: token.approved_account_ids,
                royalty: token.royalty
            })
        } else { // If there wasn't a tokenID in the tokens_by_id collection
            None
        }
    }
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {

    #[private]
    fn nft_resolve_transfer(
        &mut self,
        // Authorized id for logging the transfer event
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>
    ) -> bool {
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            // the nft_on_transfer should return whether we should return the token to its owner or not
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                if !return_token {
                    refund_approved_account_ids(owner_id, &approved_account_ids);
                    return true;
                }
            }
        }

        // get the token object if there is some token obejct
        let mut token = if let Some(token) = self.tokens_by_id.get(&token_id) {
            if token.owner_id != receiver_id {
                // refund the owner for rleasing the storage used up by the approved account IDs
                refund_approved_account_ids(owner_id, &approved_account_ids);
                // the token is not owned by the receiver anymore
                return  true;
            }
            token
        } else {
            // refund the owner for releasing the storage used up by the approved account IDs
            refund_approved_account_ids(owner_id, &approved_account_ids);
            return true;
        };

        // remove the token from the reciever
        self.internal_remove_token_from_owner(&receiver_id, &token_id);

        // add the token to the original owner
        self.internal_add_token_to_owner(&owner_id, &token_id);

        // change the token struct's owner to be the original owner
        token.owner_id = owner_id.clone();

        // refund the receiver any approved account ids that they may have set on the token
        refund_approved_account_ids(receiver_id.clone(), &token.approved_account_ids);
        // reset the approved account ids to what they were before the transfer
        token.approved_account_ids = approved_account_ids;

        // insert the token back into the tokens_by_id collection
        self.tokens_by_id.insert(&token_id, &token);

        // Log that the NFT was reverted back to the original owner
        let nft_transfer_log: EventLog = EventLog {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                authorized_id,
                old_owner_id: receiver_id.to_string(),
                new_owner_id: owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo
            }])
        };

        // log
        env::log_str(&nft_transfer_log.to_string());

        false
    }
}