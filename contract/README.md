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

near call $NFT_CONTRACT nft_mint 

```
## nft_metadata

### Description

### Arguments
### Usage

## nft_transfer

### Description

### Arguments
### Usage

## nft_token

### Description

### Arguments
### Usage

## nft_approve

### Description

### Arguments
### Usage

## nft_is_approved

### Description

### Arguments
### Usage

## nft_revoke

### Description

### Arguments
### Usage

## nft_revoke_all

### Description

### Arguments
### Usage

## nft_total_supply

### Description

### Arguments
### Usage

## nft_tokens

### Description

### Arguments
### Usage

## nft_supply_for_owner

### Description

### Arguments
### Usage

## nft_tokens_for_owner

### Description

### Arguments
### Usage