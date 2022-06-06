use crate::{constant::*, error::CasinoError, state::*};
use anchor_lang::prelude::*;

// Traits
// trait Table {
//     fn
// }

// Implementations
impl Ratio {
    pub fn new(mult: u8, div: u8) -> Result<Self> {
        if div == 0 {
            return Err(error!(CasinoError::InvalidRatioDiv));
        }

        Ok(Self { mult, div })
    }
}

impl Table {
    pub fn new(stakes: TableStakes) -> Result<Self> {
        let (min_bet, max_bet, standard_payout_ratio) = match stakes {
            TableStakes::Low => (LOW_STAKES_MIN, LOW_STAKES_MAX, Ratio::new(3, 2)?),
            TableStakes::Medium => (MID_STAKES_MIN, MID_STAKES_MAX, Ratio::new(2, 1)?),
            TableStakes::High => (HIGH_STAKES_MIN, HIGH_STAKES_MAX, Ratio::new(5, 2)?),
        };

        let high_payout_ratio = Ratio::new(
            standard_payout_ratio
                .mult
                .checked_mul(3)
                .ok_or(CasinoError::NumericalOverflow)?,
            standard_payout_ratio.div,
        )?;

        Ok(Self {
            stakes,
            min_bet,
            max_bet,
            standard_payout_ratio,
            high_payout_ratio,
        })
    }
}

impl Deck {
    pub fn new(num_decks: u8) -> Result<Self> {
        if num_decks > MAX_DECKS {
            return Err(error!(CasinoError::TooManyDecks));
        }

        let mut cards: Vec<Card> = vec![];
        let discards: Vec<Card> = vec![];

        for _ in 0..num_decks {
            for suit in Suit::iter() {
                for face in CardType::iter() {
                    cards.push(Card { suit, face });
                }
            }
        }

        Ok(Self { cards, discards })
    }
}
