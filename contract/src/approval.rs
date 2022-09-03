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
    // Approve an account ID to transfer a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        todo!()
    }

    // check if the passwed in account hass access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>
    ) -> bool {
        todo!()
    }

    // revoke a specific account from transferring the token on your behalf
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        todo!()
    }

    // Revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        todo!()
    }
}