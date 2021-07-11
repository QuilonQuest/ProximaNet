#[path = "./types.rs"]
mod types;

use types::{TokenId, TokenPrice, CollectionId, AccountIdHash, Allow, Fee, EditionNumber};
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId, near_bindgen, Balance};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub description: String,
    pub date: String,
    pub thumbnail: String,
    pub creator: AccountId,
    pub minters: Vec<AccountId>,
}


#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub bidder: AccountId,
    pub amount: Balance,
    pub date: String,
    pub executed: bool
}


#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
pub struct Token {
    pub edition_index: u64,
    pub editions: EditionNumber,
    pub metadata: TokenId,
}



#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
pub struct Edition {
    pub edition_number: EditionNumber,
    pub edition_owner: AccountId,
    pub token_id: TokenId
}



#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub collection_id: CollectionId,
    pub creator: String,
    pub description: String,
    pub thumbnail: String,
    pub main: String,
    pub nft_type: String,
    pub file: String,
    pub external_link: String,
    pub royalty: u32,
    pub editions: EditionNumber,
    pub date: String,
    pub tags: Vec<String>
}
