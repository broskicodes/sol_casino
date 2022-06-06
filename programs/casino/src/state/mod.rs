pub mod implementation;

use crate::constant::MAX_DECKS;
use anchor_lang::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Enums
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum AccountType {
    CasinoV1,
    BlackjackTable,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum TableStakes {
    Low,
    Medium,
    High,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy, EnumIter)]
pub enum Suit {
    Heart,
    Club,
    Diamond,
    Spade,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy, EnumIter)]
pub enum CardType {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

// Structs
pub const RATIO_SIZE: usize = 1 + 1;
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct Ratio {
    pub mult: u8,
    pub div: u8,
}

pub const TABLE_SIZE: usize = 1 + 8 + 8 + (2 * RATIO_SIZE);
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct Table {
    pub stakes: TableStakes,
    pub min_bet: u64,
    pub max_bet: u64,
    pub standard_payout_ratio: Ratio,
    pub high_payout_ratio: Ratio,
}

pub const CARD_SIZE: usize = 1 + 1;
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub face: CardType,
}

pub const DECK_SIZE: usize = 4 + 4 + (MAX_DECKS as usize * 52 * CARD_SIZE);
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub discards: Vec<Card>,
}

// Accounts
pub const CASINO_ACCOUNT_SIZE: usize = 8 + 1 + 32 + 8 + 8;
#[account]
pub struct Casino {
    pub key: AccountType,
    pub authority: Pubkey,
    pub sol_per_chip_rate: u64,
    pub chips_outstanding: u64,
}

pub const BLACKJACK_TABLE_ACCOUNT_SIZE: usize = 8 + 1 + 32 + TABLE_SIZE;
#[account]
pub struct BlackjackTable {
    pub key: AccountType,
    pub casino: Pubkey,
    pub table: Table,
}
