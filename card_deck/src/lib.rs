pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

pub enum Rank {
    Ace,
    Number(u8),
    King,
    Queen,
    Jack,
}

impl Suit {
    pub fn random() -> Suit {
    }

    pub fn translate(value: u8) -> Suit {
    }
}

impl Rank {
    pub fn random() -> Rank {
    }

    pub fn translate(value: u8) -> Rank {
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        return true;
    }
    false
}