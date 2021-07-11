mod model;
mod types;
mod logger;

use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector, LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};
use crate::types::{TokenId, AccountIdHash, EditionNumber, TokenPrice, CollectionId};
use crate::model::{Metadata, Token, Edition, Collection, Bid};
use std::borrow::Borrow;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use near_sdk::env::sha256;
use near_sdk::serde::{Serialize, Deserialize};

static METADATA_ERROR: &str = "Metadata exceeds character limits.";
static TOKEN_LOCKED: &str = "This edition is burned or locked.";
static PAUSED_ERR: &str = "Maintenance going on. Minting and transfers are temporarily disabled.";
static ONLY_OWNER: &str = "Only contract owner can call this method.";
static ONLY_MINTER: &str = "Only whitelisted artists can call this method.";
static ONLY_TOKEN_OWNER: &str = "Only token owner can call this method.";
static ONLY_COLLECTION_MINTER: &str = "Only collection minter can call this method.";
static ONLY_ESCROW: &str = "You don't have rights to access this account's funds.";
static ACC_NOT_VALID: &str = "Account ID is invalid.";
static DEPOSIT_NOT_ENOUGH: &str = "Deposit not enough to cover metadata storage fee.";
static EVENT_MINT: &str = "Mint";
static EVENT_BURN_TOKEN: &str = "BurnToken";
static EVENT_BURN_EDITION: &str = "BurnEdition";
static EVENT_CREATE_COLLECTION: &str = "CreateCollection";
static EVENT_MINTER_ADD: &str = "MinterAdd";
static EVENT_OFFER: &str = "Offer";
static EVENT_CANCEL_OFFER: &str = "OfferCancel";
static EVENT_ACCEPT_OFFER: &str = "OfferAccept";
static EVENT_TRANSFER: &str = "Transfer";
static EVENT_TRANSFER_BATCH: &str = "TransferBatch";
static EVENT_APPROVAL: &str = "Approval";
static EVENT_OWNERSHIP_TRANSFERRED: &str = "OwnershipTransferred";
static EVENT_MARKET_UPDATE: &str = "MarketUpdate";
static EVENT_MARKET_BATCH_UPDATE: &str = "MarketBatchUpdate";
static EVENT_MARKET_DELETE: &str = "MarketDelete";
static EVENT_MARKET_BUY: &str = "MarketBuy";


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub enum EditionState {
    AVAILABLE,
    LISTED,
    LOCKED,
    BURNED,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub enum TransferReason {
    ROYALTY,
    SALE,
    FEE,
    NEARFOLIO,
}


/// This trait provides the baseline of functions as described at:
/// https://github.com/nearprotocol/NEPs/blob/nep-4/specs/Standards/Tokens/NonFungibleToken.md
pub trait NEP4 {
    // Grant the access to the given `accountId` for the given `tokenId`.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    fn grant_access(&mut self, escrow_account_id: AccountId);

    // Revoke the access to the given `accountId` for the given `tokenId`.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    fn revoke_access(&mut self, escrow_account_id: AccountId);

    // Transfer the given `tokenId` to the given `accountId`. Account `accountId` becomes the new owner.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    fn transfer_from(&mut self, from: AccountId, to: AccountId, token_id: u64, edition_number: u64);

    // Transfer the given `tokenId` to the given `accountId`. Account `accountId` becomes the new owner.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should be the owner of the token. Callers who have
    // escrow access should use transfer_from.
    fn transfer(&mut self, to: AccountId, token_id: TokenId, edition_number: EditionNumber);

    // Returns `true` or `false` based on caller of the function (`predecessor_id) having access to the token
    fn check_access(&self, account_id: AccountId, escrow_id: AccountId) -> bool;

    fn grant_edition_allowance(&mut self, token_id: TokenId, edition_id: u64, account: AccountId);
    fn remove_edition_allowance(&mut self, token_id: TokenId, edition_id: u64, account: AccountId);
    fn check_allowance(&self, token_id: TokenId, edition_id: u64, account: AccountId) -> bool;
}

// Begin implementation
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NonFungibleToken {
    pub owner_id: AccountId,
    pub current_supply: u64,
    pub total_editions: u64,
    pub total_collections: u64,
    pub minters: UnorderedSet<AccountId>,
    pub metadata: LookupMap<TokenId, Metadata>,
    pub tokens: LookupMap<TokenId, Token>,
    pub collections: LookupMap<CollectionId, Collection>,
    pub editions: LookupMap<u64, Edition>,
    pub edition_states: LookupMap<u64, EditionState>,
    pub marketplace: LookupMap<u64, TokenPrice>,
    pub account_gives_access: LookupMap<AccountId, UnorderedSet<AccountId>>,
    pub edition_allowances: LookupMap<u64, UnorderedSet<AccountId>>,
    pub offers: LookupMap<String, Vector<Bid>>,
    // Vec<u8> is sha256 of account, makes it safer and is how fungible token also works
    pub mint_storage_fee: Balance,
    pub edition_storage_fee: Balance,
    pub create_collection_fee: Balance,
    pub trade_fee: Balance,
    pub paused: bool,
    pub fee_receiver: AccountId,
    pub MAX_NAME_LENGTH: u8,
    pub MAX_DESCRIPTION_LENGTH: u8,
    pub IPFS_HASH_LENGTH: u8,
    pub MAX_EDITIONS: u8,
    pub MAX_EXTERNAL_LINK: u8,
}


impl Default for NonFungibleToken {
    fn default() -> Self {
        panic!("NFT should be initialized before usage")
    }
}

#[near_bindgen]
impl NonFungibleToken {
    #[init]
    pub fn new(owner_id: AccountId, fee_receiver: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid.");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            current_supply: 0,
            total_editions: 0,
            total_collections: 0,
            minters: UnorderedSet::new(b"mt".to_vec()),
            metadata: LookupMap::new(b"md".to_vec()),
            tokens: LookupMap::new(b"t".to_vec()),
            collections: LookupMap::new(b"c".to_vec()),
            editions: LookupMap::new(b"e".to_vec()),
            edition_states: LookupMap::new(b"st".to_vec()),
            marketplace: LookupMap::new(b"mp".to_vec()),
            account_gives_access: LookupMap::new(b"esc".to_vec()),
            edition_allowances: LookupMap::new(b"ea".to_vec()),
            offers: LookupMap::new(b"O".to_vec()),
            mint_storage_fee: 300_000_000_000_000_000_000_000,
            edition_storage_fee: 35_000_000_000_000_000_000_000,
            create_collection_fee: 2_000_000_000_000_000_000,
            trade_fee: 13,
            paused: true,
            fee_receiver,
            MAX_NAME_LENGTH: 30,
            MAX_DESCRIPTION_LENGTH: 250,
            IPFS_HASH_LENGTH: 46,
            MAX_EDITIONS: 25,
            MAX_EXTERNAL_LINK: 100,
        }
    }
}

#[near_bindgen]
impl NEP4 for NonFungibleToken {
    fn grant_access(&mut self, escrow_account_id: AccountId) {
        let mut acc = self.account_gives_access.get(&env::predecessor_account_id()).unwrap_or(UnorderedSet::new(env::sha256(env::predecessor_account_id().as_bytes()).to_vec()));
        assert_eq!(acc.contains(&escrow_account_id), false, "{}", "ALREADY GRANTED ACCESS");
        acc.insert(&escrow_account_id);
        self.account_gives_access.insert(&env::predecessor_account_id(), &acc);
        logger::add_escrow(env::predecessor_account_id(), acc.to_vec());
    }
    fn revoke_access(&mut self, escrow_account_id: AccountId) {
        let mut acc = self.account_gives_access.get(&env::predecessor_account_id()).unwrap_or(UnorderedSet::new(env::sha256(env::predecessor_account_id().as_bytes()).to_vec()));
        acc.remove(&escrow_account_id);
        self.account_gives_access.insert(&env::predecessor_account_id(), &acc);
        logger::add_escrow(env::predecessor_account_id(), acc.to_vec());
    }


    #[payable]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, token_id: u64, edition_number: u64) {
        let index = self.tokens.get(&token_id).unwrap().edition_index + edition_number;
        assert_eq!(self.is_paused(), false, "{}", PAUSED_ERR);
        assert_eq!(self.check_access(from.clone(), env::predecessor_account_id()) ||
                       self._is_allowed(index, env::predecessor_account_id()),
                   true, "{}", ONLY_ESCROW);
        self._internal_transfer(from, to, token_id, edition_number, index);
    }

    #[payable]
    fn transfer(&mut self, to: AccountId, token_id: TokenId, edition_number: EditionNumber) {
        assert_eq!(self.is_paused(), false, "{}", PAUSED_ERR);
       // self.check_valid_account(to.clone());
        self.only_token_owner(token_id, edition_number);
        let index = self.tokens.get(&token_id).unwrap().edition_index;
        let mut edition = self.editions.get(&u64::from(edition_number + index)).unwrap();
        let state = self.edition_states.get(&(&edition_number + index)).unwrap();
        // ensure token is available
        match state {
            EditionState::LOCKED => {
                env::panic(TOKEN_LOCKED.as_bytes());
            }
            EditionState::LISTED => {
                self.marketplace.remove(&(edition_number + index));
                //self.events.push(&Event::new_event(EVENT_MARKET_DELETE.to_string(), env::predecessor_account_id(),
                //                                   env::predecessor_account_id(), env::predecessor_account_id(), token_id, edition_number, 0));
            }
            _ => {}
        }
        assert_eq!(edition.edition_owner == env::predecessor_account_id() && edition.edition_number == edition_number, true, "{}", ONLY_TOKEN_OWNER);
        edition.edition_owner = to.clone();
        self.editions.insert(&u64::from(edition_number + index), &edition);
        self._clear_allowance(u64::from(edition_number + index));
        logger::transfer_edition(edition, u64::from(edition_number + index), to);
    }
    fn check_access(&self, account_id: AccountId, escrow_id: AccountId) -> bool {
        let acc = self.account_gives_access.get(&account_id).unwrap_or(UnorderedSet::new(account_id.as_bytes().to_vec()));
        //  assert_eq!(acc.contains(&env::predecessor_account_id()), true, "{}", ONLY_ESCROW);
        acc.contains(&escrow_id)
    }
    fn grant_edition_allowance(&mut self, token_id: TokenId, edition_id: u64, account: AccountId) {
        self.only_token_owner(token_id, edition_id);
        let idx = self.tokens.get(&token_id).unwrap().edition_index + edition_id;
        let mut allowances = self.edition_allowances.get(&idx).unwrap();
        assert_eq!(allowances.contains(&account), false, "ALREADY GRANTED ALLOWANCE");
        allowances.insert(&account);
        self.edition_allowances.insert(&idx, &allowances);
        logger::edition_allowance(token_id, edition_id, idx, allowances.as_vector().to_vec())
    }
    fn remove_edition_allowance(&mut self, token_id: TokenId, edition_id: u64, account: AccountId) {
        self.only_token_owner(token_id, edition_id);
        let idx = self.tokens.get(&token_id).unwrap().edition_index + edition_id;
        let mut allowances = self.edition_allowances.get(&idx).unwrap();
        assert_eq!(allowances.contains(&account), true, "ALREADY GRANTED ALLOWANCE");
        allowances.remove(&account);
        self.edition_allowances.insert(&idx, &allowances);
        logger::edition_allowance(token_id, edition_id, idx, allowances.as_vector().to_vec())
    }
    fn check_allowance(&self, token_id: TokenId, edition_id: u64, account: AccountId) -> bool {
        let idx = self.tokens.get(&token_id).unwrap().edition_index + edition_id;
        let allowances = self.edition_allowances.get(&idx).unwrap();
        allowances.contains(&account)
    }
}


/// Methods not in the strict scope of the NFT spec (NEP4)
#[near_bindgen]
impl NonFungibleToken {
    pub fn add_minter(&mut self, minter: AccountId) {
        self.only_owner();
        self.minters.insert(&minter);

        logger::minter_added(minter);
        // self.events.push(&Event::new_event(EVENT_MINTER_ADD.to_string(), env::predecessor_account_id(),
        //                                   env::current_account_id().to_string(), minter, 0, 0, 0));
    }
    pub fn add_collection_minter(&mut self, collection_id: CollectionId, person: AccountId) {
        let mut target = self.collections.get(&collection_id).unwrap();
        assert_eq!(target.creator == env::predecessor_account_id(), true, "{}", ONLY_COLLECTION_MINTER);
        assert_eq!(target.minters.contains(&person), false, "{}", "USER ALREADY AUTHORIZED");
        target.minters.push(person);
        // self.minters.insert(&person);
        self.collections.insert(&collection_id, &target);
        logger::collection_minter_update(target.clone(), collection_id.clone());
    }
    pub fn remove_collection_minter(&mut self, collection_id: CollectionId, person: AccountId) {
        let mut target = self.collections.get(&collection_id).unwrap();
        assert_eq!(target.creator == env::predecessor_account_id(), true, "{}", ONLY_COLLECTION_MINTER);
        assert_eq!(target.minters.contains(&person) == true, true, "{}", "USER NOT AUTHORIZED");
        let idx = target.minters.iter().position(|r| r.eq(&person)).unwrap();
        target.minters.remove(idx);
        logger::collection_minter_update(target.clone(), collection_id.clone());
    }
    pub fn remove_minter(&mut self, minter: AccountId) {
        self.only_owner();
        assert_eq!(self.minters.contains(&minter), true, "{}", ACC_NOT_VALID);
        self.minters.remove(&minter);
        logger::minter_removed(minter);
    }


    #[payable]
    pub fn mint_token(&mut self, mut metadata: Metadata) {
        assert!(env::attached_deposit() >= (self.mint_storage_fee + (self.edition_storage_fee * metadata.editions as u128)), "{} {}", DEPOSIT_NOT_ENOUGH, (self.mint_storage_fee + (self.edition_storage_fee * metadata.editions as u128)));

        self.only_whitelisted();
        self._validate_token(metadata.clone());
        let new_token_id: TokenId = self.current_supply;
        let new_edition_index = self.total_editions + 1;
        metadata.creator = env::predecessor_account_id();
        metadata.date = env::block_timestamp().to_string();
        // check collection permission if metadata contains
        let mut col = self.collections.get(&metadata.collection_id).unwrap();
        // check if sender is authorized to mint in that collection
        if metadata.collection_id > 0 {
            assert!(col.minters.contains(&(env::predecessor_account_id() as AccountId)), "{}", ONLY_COLLECTION_MINTER);
        }
        // if collection exists
        // get token_id for new token
        // create new token
        let mut new_token = Token {
            edition_index: self.total_editions,
            editions: metadata.editions,
            metadata: new_token_id,
        };
        // update balance
        // insert new token metadata
        // insert metadata to archive
        // insert editions
        // insert balances
        self.tokens.insert(&new_token_id, &new_token);
        self.metadata.insert(&new_token_id, &metadata);
        // update user balance
        self.generate_editions(new_token_id.clone(), metadata.clone(), env::predecessor_account_id(), new_edition_index);
        // save states.
        self.current_supply += 1;
        self.total_editions += metadata.editions as u64;
        logger::log_mint(metadata, new_token_id, env::predecessor_account_id());
    }
    fn _validate_token(&self, meta: Metadata) {
        assert_eq!(meta.editions <= self.MAX_EDITIONS as u64, true, "{}: {}", METADATA_ERROR, "Max Edition Number is 20.");
        assert_eq!(meta.description.len() <= self.MAX_DESCRIPTION_LENGTH as usize, true, "{}: {}", METADATA_ERROR, "Description must be under 250 characters long.");
        assert_eq!(meta.name.len() < self.MAX_NAME_LENGTH as usize, true, "{}: {}", METADATA_ERROR, "Name must be under 50 characters long.");
        assert_eq!(meta.external_link.len() <= self.MAX_EXTERNAL_LINK as usize, true, "{}: {}", METADATA_ERROR, "External link must be under 100 characters long. Please use a url shortener or ipfs.");
        assert_eq!(meta.tags.len() <= 3, true, "{}: {}", METADATA_ERROR, "Only 3 tags allowed.");
        //assert_eq!(meta.thumbnail.len() == self.IPFS_HASH_LENGTH as usize, true, "{}: {}", METADATA_ERROR, "IPFS Hash must be 46 bytes long");
        //assert_eq!(meta.main.len() == self.IPFS_HASH_LENGTH as usize, true, "{}: {}", METADATA_ERROR, "IPFS Hash must be 46 bytes long");
    }
    fn _validate_collection(&self, meta: Collection) {
        assert_eq!(meta.name.len() <= self.MAX_NAME_LENGTH as usize, true, "{}: {}", METADATA_ERROR, "Name must be under 50 characters long.");
        assert_eq!(meta.description.len() <= self.MAX_DESCRIPTION_LENGTH as usize, true, "{}: {}", METADATA_ERROR, "Description must be under 250 characters long.");
        assert_eq!(meta.thumbnail.len() == self.IPFS_HASH_LENGTH as usize, true, "{}: {}", METADATA_ERROR, "IPFS Hash must be 46 bytes long");
    }
    fn generate_editions(&mut self, new_token_id: TokenId, metadata: Metadata, pred: AccountId, current_edition: u64) {
        // generate each unique edition
        for i in 0..metadata.editions {
            self.editions.insert(&u64::from(&current_edition + i), &Edition {
                edition_owner: pred.clone(),
                edition_number: i + 1,
                token_id: new_token_id,
            });
            self.edition_states.insert(&u64::from(&current_edition + i), &EditionState::AVAILABLE);
            // account_to_editions.insert(&u64::from(&current_edition + i));
            let new_allowance: UnorderedSet<AccountId> = UnorderedSet::new(self.prefix(&current_edition.to_string()));
            self.edition_allowances.insert(&u64::from(&current_edition + i), &new_allowance);
            logger::log_mint_editions(Edition {
                edition_owner: pred.clone(),
                edition_number: i + 1,
                token_id: new_token_id,
            }, &current_edition + i);
        }
        // self.account_to_editions.insert(&env::predecessor_account_id(), &account_to_editions);
    }
    fn prefix(&self, account_id: &AccountId) -> Vec<u8> {
        format!("o{}", account_id).into_bytes()
    }
    //
    // fn owned_editions_prefix(&self, account_id: &AccountId) -> Vec<u8> {
    //     format!("oe{}", account_id).into_bytes()
    // }

    // burns single, owned edition of a token. not every token! be careful using it. you will lose ownership of edition and edition will be lost forever.

    pub fn burn_edition(&mut self, token_id: TokenId, edition_id: EditionNumber) {
        self.only_token_owner(token_id, edition_id);

        let to_burn_idx = edition_id + self.tokens.get(&token_id).unwrap().edition_index;
        let state = self.edition_states.get(&to_burn_idx).unwrap();
        match state {
            EditionState::LOCKED => {
                env::panic(TOKEN_LOCKED.as_bytes());
            }
            EditionState::LISTED => {
                self.marketplace.remove(&to_burn_idx);
            }
            _ => {}
        }

        //  let mut owned = self.account_to_editions.get(&env::predecessor_account_id()).unwrap();
        //   owned.remove(&to_burn_idx);
        //   self.account_to_editions.insert(&env::predecessor_account_id(), &owned);

        self.editions.remove(&to_burn_idx);
        self.edition_states.insert(&to_burn_idx, &EditionState::BURNED);
        self._clear_allowance(to_burn_idx);
        logger::burn(token_id, edition_id, to_burn_idx, env::predecessor_account_id())
    }

    #[payable]
    pub fn create_collection(&mut self, mut collection: Collection) {
        assert!(env::attached_deposit() >= self.create_collection_fee, "{}", DEPOSIT_NOT_ENOUGH);
        self._validate_collection(collection.clone());
        self.only_whitelisted();
        let new_collection_id = self.total_collections + 1;
        collection.creator = env::predecessor_account_id();
        collection.minters.push(env::predecessor_account_id());
        collection.date = env::block_timestamp().to_string();
        self.collections.insert(&new_collection_id, &collection);

        //self.events.push(&Event::new_event(EVENT_CREATE_COLLECTION.to_string(), env::predecessor_account_id(),
        //                                 env::current_account_id().to_string(), env::predecessor_account_id(), new_collection_id, new_collection_id, 0));
        self.total_collections += 1;

        logger::log_collection(collection, new_collection_id);
    }

    pub fn set_price(&mut self, token_id: TokenId, edition_id: EditionNumber, price_as_yoctonear: String) {
        // check if its owner
        self.only_token_owner(token_id, edition_id);
        let price = u128::from_str(&price_as_yoctonear).unwrap();
        self._set_price(token_id, edition_id, price);
    }

    pub fn batch_set_price(&mut self, token_id: TokenId, edition_ids: Vec<EditionNumber>, price_as_yoctonear: String) {
        assert_eq!(edition_ids.len() > 0, true, "EDITIONS CANNOT BE EMPTY");
        let price = u128::from_str(&price_as_yoctonear).unwrap();
        for i in 0..edition_ids.len() {
            self._set_price(token_id, edition_ids[i], price);
        }
    }

    fn _set_price(&mut self, token_id: TokenId, edition_id: EditionNumber, price: u128) {
        // add token to marketplace
        let token = self.tokens.get(&token_id).unwrap();
        let index = token.edition_index;
        let edition = self.editions.get(&(u64::from(edition_id as u64 + index as u64))).unwrap();
        assert_eq!(edition.edition_owner == env::predecessor_account_id(), true, "{}", ONLY_TOKEN_OWNER);
        self.marketplace.insert(&(edition_id as u64 + index as u64), &price);
        self.edition_states.insert(&(edition_id as u64 + index as u64), &EditionState::LISTED);

        logger::marketplace_insert(edition, index + edition_id, price);
        logger::insert_activity(token_id, edition_id, EVENT_MARKET_UPDATE.to_string(), price.to_string(), env::predecessor_account_id());
    }

    pub fn get_price(&self, token_id: TokenId, edition_id: EditionNumber) -> TokenPrice {
        let index = self.tokens.get(&token_id).unwrap().edition_index;
        self.marketplace.get(&(edition_id as u64 + index as u64)).unwrap()
    }

    pub fn cancel_sale(&mut self, token_id: TokenId, edition_id: u64) {
        self.only_token_owner(token_id, edition_id);
        // remove token from marketplace
        let index = self.tokens.get(&token_id).unwrap().edition_index + edition_id;
        let edition = self.editions.get(&index).unwrap();
        assert_eq!(edition.edition_owner == env::predecessor_account_id(), true, "{}", ONLY_TOKEN_OWNER);
        self.marketplace.remove(&edition_id);
        logger::marketplace_remove(edition, index);
        // self.events.push(&Event::new_event(EVENT_MARKET_DELETE.to_string(), env::predecessor_account_id(),
        //                                   env::current_account_id().to_string(), env::predecessor_account_id(), token_id, edition_id, 0));
    }

    #[payable]
    pub fn buy(&mut self, token_id: TokenId, edition_id: u64) {
        // check price & deposit & check if token available
        let token = self.tokens.get(&token_id).unwrap();
        let idx = token.edition_index;
        let edition_index = idx + edition_id;
        let listed = self.marketplace.get(&edition_index).unwrap();
        /// return money if deposit not enough
        assert_eq!(env::attached_deposit() >= listed, true, "{}", "DEPOSIT NOT ENOUGH");
        let mut target = self.editions.get(&edition_index).unwrap();
        let old_owner = target.edition_owner.clone();
        assert_eq!(env::predecessor_account_id() != old_owner.clone(), true, "{}", "CANNOT BUY YOUR OWN TOKEN");

        // send money to their owners, calculate royalties
        self._internal_transfer(old_owner.clone(), env::predecessor_account_id(), token_id, edition_id, edition_index.clone());
        logger::insert_activity(token_id, edition_id, EVENT_MARKET_BUY.to_string(), env::attached_deposit().to_string(), old_owner.clone());
        logger::marketplace_remove(target.clone(), edition_index);
        let nearfolio_fee: u128 = env::attached_deposit().div(self.trade_fee);
        let rest = env::attached_deposit() - nearfolio_fee;
        let mut sellers: u128 = 0;
        Promise::new(self.fee_receiver.clone()).transfer(nearfolio_fee);
        logger::near_transfer(self.fee_receiver.clone(), nearfolio_fee, TransferReason::FEE, env::block_timestamp());
        let md = self.metadata.get(&token.metadata).unwrap();
        let mut royalty_fee = 0;
        if md.creator != target.edition_owner {
            if md.royalty == 1 {
                Promise::new(md.creator.clone()).transfer(rest);
                logger::near_transfer(md.creator.clone(), rest.clone(), TransferReason::ROYALTY, env::block_timestamp());
                //   env::log(format!("Sent royalties. {} $NEAR to {}", rest, md.creator.clone()).as_bytes());
            } else if md.royalty > 1 {
                royalty_fee = rest.div((u128::from(md.royalty)));
                sellers = rest.sub(royalty_fee);
                if royalty_fee > 0 {
                    Promise::new(md.creator.clone()).transfer(royalty_fee);
                    logger::near_transfer(md.creator, royalty_fee, TransferReason::ROYALTY, env::block_timestamp());
                }
            } else {
                sellers = rest
            }
        } else {
            sellers = rest
        }
        if sellers > 0 {
            Promise::new(old_owner.clone()).transfer(sellers.clone());
            logger::near_transfer(old_owner.clone(), sellers, TransferReason::SALE, env::block_timestamp());
        }
    }

    #[payable]
    pub fn offer(&mut self, token_id: TokenId, edition_id: EditionNumber) {
        assert_eq!(!self.paused, true, "{}", PAUSED_ERR);
        let token = self.tokens.get(&token_id).unwrap();
        let edition = self.editions.get(&(token.edition_index + edition_id as u64)).unwrap();
        assert_eq!(env::attached_deposit() > self.mint_storage_fee, true, "{}", "NOTHING DEPOSITED");
        assert_eq!(edition.edition_owner != env::predecessor_account_id(), true, "YOU CANNOT BID ON YOUR OWN TOKEN");
        let tok_x_edition: String = self.gen_token_x_edition(token_id, edition_id);
        let bid: Bid = Bid {
            bidder: env::predecessor_account_id(),
            amount: env::attached_deposit(),
            date: env::block_timestamp().to_string(),
            executed: false,
        };
        let mut current_offers = self.offers.get(&tok_x_edition).unwrap_or(Vector::new(sha256(tok_x_edition.as_bytes()).to_vec()));
        current_offers.push(&bid);


        logger::new_offer(bid.clone(), current_offers.len() - 1, token_id.clone(), edition_id.clone());
        self.offers.insert(&tok_x_edition, &current_offers);

        logger::insert_activity(token_id, edition_id, EVENT_OFFER.to_string(), bid.amount.to_string(), edition.edition_owner);
    }

    pub fn accept_offer(&mut self, token_id: TokenId, edition_id: EditionNumber, idx: u64) {
        /// accept, /remove other offers/, transfer money, transfer nft
        let tokxedition = self.gen_token_x_edition(token_id, edition_id);
        let token = self.tokens.get(&token_id).unwrap();
        let edition_idx = token.edition_index + edition_id as u64;
        let mut edition = self.editions.get(&edition_idx).unwrap();
        assert_eq!(edition.edition_owner == env::predecessor_account_id(), true, "{}", ONLY_TOKEN_OWNER);
        let old_owner = edition.edition_owner.clone();
        let mut offers = self.offers.get(&tokxedition).unwrap();
        let mut to_be_accepted = offers.get(idx).unwrap();
        assert_eq!(to_be_accepted.executed == false, true, "{}", "OFFER IS CANCELLED OR ACCEPTED.");
        self._internal_transfer(env::predecessor_account_id(), to_be_accepted.bidder.clone(), token_id, edition_id, edition_idx.clone());

        self.edition_states.insert(&(edition_idx as u64), &EditionState::AVAILABLE);
        // send money to their owners
        let nearfolio_fee: u128 = to_be_accepted.amount.div(self.trade_fee);
        let rest = to_be_accepted.amount - nearfolio_fee;
        let mut sellers: u128 = 0;
        Promise::new(self.fee_receiver.clone()).transfer(nearfolio_fee);
        logger::near_transfer(self.fee_receiver.clone(), nearfolio_fee.clone(), TransferReason::FEE, env::block_timestamp());
        let md = self.metadata.get(&token.metadata).unwrap();
        let mut royalty_fee = 0;
        if md.creator != edition.edition_owner {
            if md.royalty == 1 {
                Promise::new(md.creator.clone()).transfer(rest);
                logger::near_transfer(md.creator, rest.clone(), TransferReason::ROYALTY, env::block_timestamp());
                // env::log(format!("Sent royalties. {} $NEAR to {}", rest, md.creator.clone()).as_bytes());
            } else if md.royalty > 1 {
                royalty_fee = rest.div((u128::from(md.royalty)));
                sellers = rest.sub(royalty_fee);
                if royalty_fee > 0 {
                    Promise::new(md.creator.clone()).transfer(royalty_fee);
                    logger::near_transfer(md.creator, royalty_fee, TransferReason::ROYALTY, env::block_timestamp());
                    // env::log(format!("Sent royalties. {} $NEAR to {}", royalty_fee, md.creator.clone()).as_bytes());
                }
            } else {
                sellers = rest
            }
        } else {
            sellers = rest
        }
        if sellers > 0 {
            Promise::new(old_owner.clone()).transfer(sellers.clone());
            logger::near_transfer(old_owner.clone(), sellers.clone(), TransferReason::SALE, env::block_timestamp());
        }
        logger::marketplace_remove(edition.clone(), edition_idx.clone());
        logger::accept_offer(to_be_accepted.amount.clone(), env::predecessor_account_id(), idx.clone(), token_id.clone(), edition_id.clone(), env::block_timestamp());
        logger::transfer_edition(edition.clone(), edition_idx.clone(), to_be_accepted.bidder.clone());
        logger::insert_activity(token_id, edition_id, EVENT_ACCEPT_OFFER.to_string(), to_be_accepted.amount.to_string(), to_be_accepted.bidder.clone());
        to_be_accepted.executed = true;
        to_be_accepted.bidder = "".to_string();
        to_be_accepted.amount = 0;
        offers.replace(idx, &to_be_accepted);
        self.offers.insert(&tokxedition, &offers);
    }

    pub fn cancel_offer(&mut self, token_id: TokenId, edition_id: EditionNumber, idx: u64) {
        let tokxedition = self.gen_token_x_edition(token_id, edition_id);
        let mut offer = self.offers.get(&tokxedition).unwrap();
        let mut to_be_cancelled = offer.get(idx).unwrap();
        assert_eq!(to_be_cancelled.executed == false, true, "{}", "OFFER IS CANCELLED OR ACCEPTED.");
        assert_eq!(to_be_cancelled.bidder == env::predecessor_account_id(), true, "{}", "ONLY OFFER OWNER CAN CANCEL");

        let mut cut_storage_fee = 0;
        if to_be_cancelled.amount > self.edition_storage_fee {
            cut_storage_fee = to_be_cancelled.amount - self.edition_storage_fee;
            Promise::new(env::predecessor_account_id()).transfer(cut_storage_fee);
        }
        offer.replace(idx, &to_be_cancelled);
        self.offers.insert(&tokxedition, &offer);

        self.offers.insert(&tokxedition, &offer);
        logger::execute_offer(to_be_cancelled.clone(), idx, token_id.clone(), edition_id.clone());
        logger::insert_activity(token_id, edition_id, EVENT_CANCEL_OFFER.to_string(), to_be_cancelled.amount.to_string(), to_be_cancelled.bidder.clone());
        to_be_cancelled.bidder = String::from("::");
        to_be_cancelled.executed = true;
        offer.replace(idx, &to_be_cancelled);
    }

    pub fn gen_token_x_edition(&self, token_id: TokenId, edition_id: EditionNumber) -> String {
        token_id.to_string() + &*"::".to_string() + &*edition_id.to_string()
    }

    pub fn get_allowances(&self, token_id: TokenId, edition_id: EditionNumber) -> Vec<AccountId> {
        self.edition_allowances.get(&(self.tokens.get(&token_id).unwrap().edition_index + edition_id)).unwrap().as_vector().to_vec()
    }
    /// VIEWS FOR INDEXER

    pub fn get_offers(&self, token_id: TokenId, edition_id: EditionNumber) -> Vec<Bid> {
        let tokxedition = self.gen_token_x_edition(token_id, edition_id);
        let list = self.offers.get(&tokxedition).unwrap();
        let mut result = Vec::new();
        for i in 0..list.len() {
            result.push(list.get(i).unwrap())
        };
        result
    }

    pub fn get_token(&self, token_id: TokenId) -> Token {
        self.tokens.get(&token_id).unwrap()
    }

    pub fn get_edition(&self, token_id: TokenId, edition_id: EditionNumber) -> Edition {
        let index = self.tokens.get(&token_id).unwrap();
        self.editions.get(&u64::from(index.edition_index + edition_id as u64)).unwrap()
    }

    pub fn get_collection(&self, collection_id: CollectionId) -> Collection {
        self.collections.get(&collection_id).unwrap()
    }
    pub fn get_metadata(&self, token_id: TokenId) -> Metadata {
        self.metadata.get(&token_id).unwrap()
    }
    pub fn owner_of(&self, token_id: TokenId, edition_id: EditionNumber) -> AccountId {
        let index = self.tokens.get(&token_id).unwrap().edition_index + edition_id;
        self.editions.get(&(index)).unwrap().edition_owner
    }
    // admin stuff
    pub fn generate_genesis_collection(&mut self, thumbnail: String) {
        self.only_owner();
        assert_eq!(self.collections.get(&(0 as u64)).is_none(), true, "GENESIS COLLECTION ALREADY CREATED");
        self.collections.insert(&(0 as u64), &Collection {
            name: "Nearfolio".to_string(),
            date: env::block_timestamp().to_string(),
            thumbnail: thumbnail.clone(),
            creator: "nearfolio.near".to_string(),
            minters: Vec::new(),
            description: "Nearfolio default collection.".to_string(),
        });
        self.paused = false;
        logger::log_collection(Collection {
            name: "Nearfolio".to_string(),
            date: env::block_timestamp().to_string(),
            thumbnail,
            creator: "nearfolio.near".to_string(),
            minters: Vec::new(),
            description: "Nearfolio default collection.".to_string(),
        }, 0);
    }
    pub fn pause(&mut self) {
        self.only_owner();
        self.paused = true;
    }
    pub fn unpause(&mut self) {
        self.only_owner();
        self.paused = false
    }
    pub fn is_paused(&self) -> bool {
        self.paused.clone()
    }
    pub fn is_escrow(&self, account_id: AccountId, escrow: AccountId) -> bool {
        self.account_gives_access.get(&account_id).unwrap().contains(&escrow)
    }
    pub fn get_escrows(&self, account_id: AccountId) -> Vec<AccountId> {
        self.account_gives_access.get(&account_id).unwrap().to_vec()
    }
    /* pub fn owned_editions(&self, account: AccountId) -> Vec<EditionNumber> {
        self.account_to_editions.get(&account).unwrap().as_vector().to_vec()
    } */
    pub fn edition_by_index(&self, index: u64) -> Edition {
        self.editions.get(&index).unwrap()
    }
    /// helper function determining contract ownership and artist permissions
    fn only_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "{}", ONLY_OWNER);
    }
    fn only_whitelisted(&self) {
        assert!(self.minters.contains(&env::predecessor_account_id()), "{}", ONLY_MINTER)
    }
    fn only_token_owner(&self, token_id: TokenId, edition_id: EditionNumber) {
        let token = self.tokens.get(&token_id).unwrap();
        let edition = self.editions.get(&u64::from(edition_id as u64 + token.edition_index)).unwrap();
        assert_eq!(edition.edition_owner, env::predecessor_account_id(), "{}", ONLY_TOKEN_OWNER)
    }
    fn check_valid_account(&self, account: AccountId) {
        let acc_hash = env::sha256(account.as_bytes());
        assert!(env::is_valid_account_id(&acc_hash), "{}", ACC_NOT_VALID);
    }
    fn _is_allowed(&self, idx: u64, account: AccountId) -> bool {
        let allowances = self.edition_allowances.get(&idx).unwrap();
        allowances.contains(&account)
    }
    fn _clear_allowance(&mut self, edition_idx: u64) {
        let mut allowances = self.edition_allowances.get(&edition_idx).unwrap();
        allowances.clear();
        self.edition_allowances.insert(&edition_idx, &allowances);
    }
    fn _internal_transfer(&mut self, from: AccountId, to: AccountId, token_id: u64, edition_number: u64, edition_idx: u64) {
        //self.check_valid_account(to.clone());
        let mut edition = self.editions.get(&edition_idx).unwrap();
        assert_eq!(self.is_paused(), false, "{}", PAUSED_ERR);
        assert_eq!(edition.edition_owner == from && edition.edition_number == edition_number, true, "{} {}", ONLY_TOKEN_OWNER, "ERROR2".to_string());
        // ensure token is available
        let state = self.edition_states.get(&edition_idx).unwrap();
        match state {
            EditionState::BURNED => {
                env::panic(TOKEN_LOCKED.as_bytes());
            }
            EditionState::LOCKED => {
                env::panic(TOKEN_LOCKED.as_bytes());
            }
            EditionState::LISTED => {
                self.marketplace.remove(&edition_idx);
            }
            _ => {}
        }

        edition.edition_owner = to.clone();

        self.editions.insert(&edition_idx, &edition);
        self.edition_states.insert(&edition_idx, &EditionState::AVAILABLE);
        self._clear_allowance(edition_idx.clone());
        logger::transfer_edition(edition, edition_idx, env::predecessor_account_id());
        logger::insert_activity(token_id, edition_number, "Transfer".to_string(), to, from)
    }
    pub fn owner(&self) -> AccountId {
        self.owner_id.clone()
    }
    pub fn is_minter(&self, account: AccountId) -> bool {
        self.minters.contains(&account).clone()
    }
    pub fn mint_fee(&self) -> Balance {
        self.mint_storage_fee.clone()
    }
    pub fn edition_fee(&self) -> Balance {
        self.edition_storage_fee.clone()
    }
    pub fn set_mint_fee(&mut self, fee: String) {
        self.only_owner();
        self.mint_storage_fee = u128::from_str(&fee).unwrap();
    }
    pub fn set_edition_fee(&mut self, fee: String) {
        self.only_owner();
        self.edition_storage_fee = u128::from_str(&fee).unwrap();
    }
    pub fn set_max_edition(&mut self, value: u8) {
        self.only_owner();
        self.MAX_EDITIONS = value;
    }
    pub fn get_states(&self) -> Vec<EditionState> {
        vec![EditionState::AVAILABLE, EditionState::LISTED, EditionState::LOCKED, EditionState::BURNED]
    }
    pub fn state_of(&self, token_id: TokenId, edition_id: EditionNumber) -> EditionState {
        self.edition_states.get(&(self.tokens.get(&token_id).unwrap().edition_index + edition_id)).unwrap()
    }
    pub fn fee_receiver(&self) -> AccountId {
        self.fee_receiver.clone()
    }
    pub fn all_minters(&self) -> Vec<AccountId> {
        self.minters.as_vector().to_vec()
    }
    pub fn set_trade_fee(&mut self, fee: u128) {
        self.only_owner();
        self.trade_fee = fee;
    }
}
