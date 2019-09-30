#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Card {
    Number(i32, Suit),
    Face(Face, Suit),
    Joker
}

impl Card {

    // card value will be determined by card face and suit
    // since there are 4 suits, the face will be multiplied by 4
    // so that each face/suit combo will be a unique integer
    pub fn value(&self) -> i32 {
        match self {
            Card::Number(i, suit) => i * 4 + suit.value(),
            Card::Face(face, suit) => face.value() * 4 + suit.value(),
            Card::Joker => -1
        }
    }
}

use std::cmp::Ordering;

// uses Card::value() to determine ordering
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

// must be implemented to satisfy rust compiler
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club
}

impl Suit {
    // four suits in order of rank
    fn value(&self) -> i32 {
        match self {
            Suit::Spade => 3,
            Suit::Heart => 2,
            Suit::Diamond => 1,
            Suit::Club => 0
        }
    }

    pub fn all_suits() -> Vec<Suit> {
        vec![
            Suit::Spade,
            Suit::Heart,
            Suit::Diamond,
            Suit::Club
        ]
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Face {
    Jack,
    Queen,
    King,
    Ace
}

impl Face {
    // num value ends at 10, so face value will start at 11
    fn value(&self) -> i32 {
        match self {
            Face::Jack => 11,
            Face::Queen => 12,
            Face::King => 13,
            Face::Ace => 14
        }
    }

    pub fn all_faces() -> Vec<Face> {
        vec![
            Face::Jack,
            Face::Queen,
            Face::King,
            Face::Ace
        ]
    }
}