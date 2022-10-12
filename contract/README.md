# Welcome To Educoin's Near Minting Smart Contract!
=================================

# Available Functions
* nft_mint
* nft_metadata
* nft_transfer
* nft_token
* nft_approve
* nft_is_approved
* nft_revoke
* nft_revoke_all
* nft_total_supply
* nft_tokens
* nft_supply_for_owner
* nft_tokens_for_owner

# Functions

## nft_mint
### Description
This function is the power horse of this contract. This function will mint a token and store it on the blockchain.
### Arguments
```shell
{
  token_id: String,
  metadata: {
    title: Optional<String>,
    description: Optional<String>,
    media: Optional<String>,
    media_hash: Optional<String>,
    copies: Optional<Number>,
    issued_at: Optional<Number>,
    expires_at: Optional<Number>,
    starts_at: Optional<Number>,
    updated_at: Optional<Number>,
    extra: Optional<String>,
    reference: Optional<String>,
    reference_hash: Optional<Base64VecU8>,
    token_type: Optional<"Content" | "Badge">
  },
  receiver_id: String,
  perpetual_royalties: Optional<{AccountId: String, Amount: Number}>
}

```
### Usage
```console
$ near call $NFT_CONTRACT nft_mint '{"token_id": "TestToken", "receiver_id": "reciever.testnet", "metadata": { "title": "Welcome" } }' --accountId myaccount.testnet --deposit 0.1 
```
## nft_metadata
### Description
Returns the Metadata for the contract
### Arguments
None
### Usage
```console
$ near call $NFT_CONTRACT nft_metadata
```
## nft_transfer
### Description
Transfer an nft from your account to another
### Arguments
```
{
  receiver_id: String,
  token_id: String,
  approval_id: Optional<Number>,
  memo: Optional<String>
}
```
### Usage
```console
$ near call $NFT_CONTRACT nft_transfer '{ "receiver_id": "another.testnet", "token_id": "tokenid" }' --accountId myAccount.testnet --deposit 0.1
```
## nft_token
### Description
Get token information for a given token id
### Arguments
```
{
  token_id: String
}
```
### Usage
```console
$ near view $NFT_CONTRACT nft_token '{"token_id": "an_exsiting_id"}'
```
## nft_approve
### Description
Let an account id transfer your tokens on your behalf
### Arguments
```
{
  token_id: String,
  account_id: String,
  msg: Optional<String>
}
```
### Usage
```console
$ near call $NFT_CONTRACT nft_approve '{"token_id": "an_exsiting_id", "account_id": "an_account.testnet"}' --accountId myAccount.testnet --deposit 0.1
```
## nft_is_approved
### Description
Check to see if a passed in account has access to approve the token id
### Arguments
```
{
  token_id: String,
  approved_account_id: String,
  approval_id: Optional<Number>
}
```
### Usage
```console
$ near call $NFT_CONTRACT nft_is_approved '{"token_id": "an_exsiting_id", "approved_account_id": "hello.testnet"}' --accountId myAccount.testnet --deposit 0.1
```
## nft_revoke
### Description
Remove a specific account from transferring the token on your behalf
### Arguments
```
{
  token_id: String,
  account_id: String
}
```
### Usage
```console
$ near call $NFT_CONTRACT nft_revoke '{ "token_id": "anToken", "account_id": "anaccount.testnet" }' --accountId myaccount.testnet --deposit 0.1
```
## nft_revoke_all
### Description
Revoke all accounts from transferring the token on your behalf
### Arguments
```
{
  token_id: String
}
```
### Usage
```console
$ near call $NFT_CONTRACT nft_revoke_all '{ "token_id": "anToken" }' --accountId myaccount.testnet --deposit 0.1
```
## nft_total_supply
### Description
Get the number of NFTS on the contract
### Arguments
None
### Usage
```console
$ near view $NFT_CONTRACT nft_total_supply
```
## nft_tokens
### Description
Query for nft tokens ono the contract regardless of the owner
### Arguments
```
{
  from_index: Optional<Number>,
  limit: Optional<Number>
}
```
### Usage
```console
$ near view $NFT_CONTRACT nft_tokens
```
## nft_supply_for_owner
### Description
Get the total supply of NFTs for a given owner
### Arguments
```
{
  account_id: String
}
```
### Usage
```console
$ near view $NFT_CONTRACT nft_supply_for_owner
```
## nft_tokens_for_owner
### Description
Query for all the tokens for an owner
### Arguments
```
{
  account_id: String,
  from_index: Optional<Number>,
  limit: Optional<Number>
}
```
### Usage
```console
$ near view $NFT_CONTRACT nft_tokens_for_owner '{"account_id": "myaccount.testnet"}'
```