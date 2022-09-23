use crate::*;
use near_sdk::ext_contract;

pub trait NonFungibleTokenCore {
    // Approve an account ID to transfer a token on your behalf
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);

    // check if the passwed in account hass access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>
    ) -> bool;

    // revoke a specific account from transferring the token on your behalf
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    // Revoke all accounts from transferring the token on your behalf
    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String
    );
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    // TODO: Write Unit Test
    // Approve an account ID to transfer a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        // Assert at least one yocto for security reasons
        assert_at_least_one_yocto();

        // get the token object from the tokenID
        let mut token = self.tokens_by_id.get(&token_id).expect("No Token");

        // make sure that the person calling the function is the owner of the token
        assert_eq!(
            &env::predecessor_account_id(),
            &token.owner_id,
            "Predecessor must be the token owner"
        );

        // get the next approval ID if we need a new approval
        let approval_id = token.next_approval_id;

        // check if the account has been approved already for this token
        let is_new_approval = token
            .approved_account_ids
            // Insert returns none if the key was not present
            .insert(account_id.clone(), approval_id)
            // if the key was not present, is_none() will return true so it is a new approval
            .is_none();
        
            // if it was a new approval, we need to calculate how much storage is being used to add the account
            let storage_used = if is_new_approval {
                bytes_for_approved_account_id(&account_id)
            // if it was not a new approval, we used no storage
            } else {
                0
            };

            // increment the token's next approval ID by 1
            token.next_approval_id += 1;

            // insert the token back into the tokens_by_id collection
            self.tokens_by_id.insert(&token_id, &token);

            // refund any excess storage attached by the user
            refund_deposit(storage_used);

            // if some message was passed into the function, we initiate a ross contract call
            if let Some(msg) = msg {
                // Defaulting GAS weight to 1, no attached deposit, and no static GAS
                ext_non_fungible_approval_receiver::ext(account_id)
                    .nft_on_approve(token_id, token.owner_id, approval_id, msg)
                    .as_return();
            }
    }

    // TODO: Write Unit Test
    // check if the passed in account hass access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>
    ) -> bool {
        // get the token object
        let token = self.tokens_by_id.get(&token_id).expect("No Token");

        // get the approval number for the passed in accountid
        let approval = token.approved_account_ids.get(&approved_account_id);

        // if there was some approval ID found for the account ID
        if let Some(approval) = approval {
            // if a specific approval_id was passed into the function
            if let Some(approval_id) = approval_id {
                // return if the approval ID passed in matches the acutal approval ID for the account
                approval_id == *approval
            } else {
                true
            }
        } else {
            false
        }
    }

    // TODO: Write Unit Test
    // revoke a specific account from transferring the token on your behalf
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        // assert that the user attached exactly 1 yoctoNEAR
        assert_one_yocto();
        // get the token object using the passed in token_id
        let mut token = self.tokens_by_id.get(&token_id).expect("No token");

        // get the called of the function and assert that they are the owner of the token
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);

        // If the account ID was in the token's approval, we remove it and the if statement logic executes
        if token
            .approved_account_ids
            .remove(&account_id)
            .is_some()
        {
            // refund the funds released by removing the approved_account_id to the caller of the function
            refund_approved_account_ids_iter(predecessor_account_id, [account_id].iter());

            // insert the token back into tokens_by_id collection witht the account_id removed from the approval list
            self.tokens_by_id.insert(&token_id, &token);
        }
    }

    // TODO: Write Unit Test
    // Revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        // assert that the called attached exactly 1 yoctoNEAR
        assert_one_yocto();

        // get the token object
        let mut token = self.tokens_by_id.get(&token_id).expect("No token");

        // get the called and make sure they are the owner of the tokens
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);

        // only revoke if the approved account IDS for the token is not empty
        if !token.approved_account_ids.is_empty() {
            // refund the approved account IDs to the caller of the fuction
            refund_approved_account_ids(predecessor_account_id, &token.approved_account_ids);
            // clear the approved account IDs
            token.approved_account_ids.clear();
            // insert the token back into the tokens_by_id collection with the approved account IDs cleared
            self.tokens_by_id.insert(&token_id, &token);
        }
    }
}