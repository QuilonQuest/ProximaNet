use near_sdk::{env, AccountId, serde_json::json, Balance};
use crate::types::{TokenId, AccountIdHash, EditionNumber, TokenPrice, CollectionId};
use crate::model::{Metadata, Token, Edition, Collection, Bid};
use crate::TransferReason;

// new token
pub(crate) fn log_mint(metadata: Metadata, token_id: TokenId, owner: AccountId) {
    env::log(
        json!({
            "type": "Metadata".to_string(),
            "action": "write",
            "cap_id": format!("tok_{}", token_id),
			"params": {
                "name": metadata.name,
                "collection_id": metadata.collection_id,
                "creator": metadata.creator,
                "description": metadata.description,
                "thumbnail": metadata.thumbnail,
                "main": metadata.main,
                "nft_type": metadata.nft_type,
                "file": metadata.file,
                "external_link": metadata.external_link,
                "royalty": metadata.royalty,
                "editions": metadata.editions,
                "date": metadata.date,
                "tags": metadata.tags,
                "token_id": token_id
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn log_mint_editions(edition: Edition, idx: u64) {
    env::log(
        json!({
            "type": "Edition".to_string(),
            "action": "write",
            "cap_id": format!("ed_{}", idx),
			"params": {
                    "edition_number": edition.edition_number,
                    "edition_owner": edition.edition_owner,
                    "token_id": edition.token_id
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn log_collection(collection: Collection, collection_id: CollectionId) {
    env::log(
        json!({
            "type": "Collection".to_string(),
            "action": "write",
            "cap_id": format!("col_{}", collection_id),
			"params": {
                    "name": collection.name,
                    "description": collection.description,
                    "date": collection.date,
                    "thumbnail": collection.thumbnail,
                    "creator": collection.creator,
                    "minters": collection.minters,
                    "collection_id": collection_id as i32
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn collection_minter_update(collection: Collection, collection_id: CollectionId) {
    env::log(
        json!({
            "type": "Collection".to_string(),
            "action": "update",
            "cap_id": format!("col_{}", collection_id),
			"params": {
                    "name": collection.name,
                    "description": collection.description,
                    "date": collection.date,
                    "thumbnail": collection.thumbnail,
                    "creator": collection.creator,
                    "minters": collection.minters,
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn transfer_edition(edition: Edition, idx: u64, new_owner_id: AccountId) {
    env::log(
        json!({
            "type": "Edition".to_string(),
            "action": "update",
            "cap_id": format!("ed_{}", idx),
			"params": {
                    "edition_number": edition.edition_number,
                    "edition_owner": new_owner_id,
                    "token_id": edition.token_id
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn marketplace_insert(edition: Edition, idx: u64, price: Balance) {
    env::log(
        json!({
            "type": "Market".to_string(),
            "action": "update",
            "cap_id": format!("mp_{}", idx),
			"params": {
                    "edition_number": edition.edition_number,
                    "edition_owner": edition.edition_owner,
                    "token_id": edition.token_id,
                    "is_listed" : true,
                    "price": price.to_string()
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn marketplace_remove(edition: Edition, idx: u64) {
    env::log(
        json!({
            "type": "Market".to_string(),
            "action": "update",
            "cap_id": format!("mp_{}", idx),
			"params": {
                    "edition_number": edition.edition_number,
                    "edition_owner": edition.edition_owner,
                    "token_id": edition.token_id,
                    "is_listed" : false,
                    "price": 0
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn new_offer(bid: Bid, idx: u64, token_id: TokenId, edition_id: u64) {
    env::log(
        json!({
            "type": "Offer".to_string(),
            "action": "insert",
            "cap_id": format!("of_{}_{}_{}", token_id, edition_id, idx),
			"params": {
                    "bidder": bid.bidder,
                    "amount": bid.amount.to_string(),
                    "token_id": token_id,
                    "edition_id": edition_id,
                    "date": bid.date,
                    "executed": bid.executed,
                    "idx": idx,
                    "accepted": false
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn accept_offer(amount: Balance, new_owner: AccountId, idx: u64, token_id: TokenId, edition_id: u64, date: u64) {
    env::log(
        json!({
            "type": "Offer".to_string(),
            "action": "update",
            "cap_id": format!("of_{}_{}_{}", token_id, edition_id, idx),
			"params": {
                    "bidder": new_owner,
                    "amount": amount.to_string(),
                    "token_id": token_id,
                    "edition_id": edition_id,
                    "date": date,
                    "executed": true,
                    "accepted": true
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn execute_offer(bid: Bid, idx: u64, token_id: TokenId, edition_id: u64) {
    env::log(
        json!({
            "type": "Offer".to_string(),
            "action": "update",
            "cap_id": format!("of_{}_{}_{}", token_id, edition_id, idx),
			"params": {
                    "bidder": env::predecessor_account_id(),
                    "amount": bid.amount.to_string(),
                    "token_id": token_id,
                    "edition_id": edition_id,
                    "date": bid.date,
                    "executed": true
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn minter_added(minter: AccountId) {
    env::log(
        json!({
            "type": "Minter".to_string(),
            "action": "insert",
            "cap_id": format!("mtr_{}", minter),
			"params": {
                    "minter": minter,
                    "can_mint": true
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn minter_removed(minter: AccountId) {
    env::log(
        json!({
            "type": "Minter".to_string(),
            "action": "update",
            "cap_id": format!("mtr_{}", minter),
			"params": {
                    "minter": minter,
                    "can_mint": false
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn insert_activity(token_id: TokenId, edition_id: u64, event_name: String, target: String, related: AccountId) {
    env::log(
        json!({
            "type": "Activity".to_string(),
            "action": "insert",
            "cap_id": format!("act_{}_{}", token_id, edition_id),
			"params": {
			    "token_id":token_id,
			    "edition_id": edition_id,
                "event_name": event_name,
                "from": env::predecessor_account_id(),
                "target": target,
                "related" : related,
                "date": env::block_timestamp()
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn burn(token_id: TokenId, edition_id: u64, to_burn_idx: u64, burner: AccountId) {
    env::log(
        json!({
            "type": "Edition".to_string(),
            "action": "update",
            "cap_id": format!("ed_{}", to_burn_idx),
			"params": {
                    "edition_number": edition_id,
                    "edition_owner": "",
                    "token_id": token_id
			}
		})
            .to_string()
            .as_bytes()
    );
    env::log(
        json!({
            "type": "insert".to_string(),
            "action": "update",
            "cap_id": format!("act_{}_{}", token_id, edition_id),
			"params": {
			    "token_id":token_id,
			    "edition_id": edition_id,
                "event_name": "Burn",
                "target": env::predecessor_account_id(),
                "related" : env::predecessor_account_id(),
                "date": env::block_timestamp()
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn near_transfer(to: AccountId, amount: Balance, reason: TransferReason, when: u64){
    env::log(
        json!({
            "type": "NEARTransfer".to_string(),
            "action": "insert",
            "cap_id": format!("ntr_{}", when.to_string()),
			"params": {
                    "to": to,
                    "amount": amount.to_string(),
                    "reason": reason,
                    "date": when.to_string()
			}
		})
            .to_string()
            .as_bytes()
    );
}

pub(crate) fn add_escrow(account: AccountId, escrow: Vec<AccountId>){
    env::log(
        json!({
            "type": "Escrow".to_string(),
            "action": "update",
            "cap_id": format!("escr_{}", account),
			"params": {
                    "account": account,
                    "escrow": escrow
			}
		})
            .to_string()
            .as_bytes()
    );
}
pub(crate) fn edition_allowance(token_id:TokenId, edition_number: u64, idx:u64, allowed: Vec<AccountId>){
    env::log(
        json!({
            "type": "Allowance".to_string(),
            "action": "update",
            "cap_id": format!("allow_{}", idx),
			"params": {
                    "token_id": token_id,
                    "edition_number": edition_number,
                    "allowed": allowed
			}
		})
            .to_string()
            .as_bytes()
    );
}

