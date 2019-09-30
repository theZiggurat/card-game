use crate::card::{Card, Suit, Face};
use rand::prelude::*;

// the deck holds all cards in the deck
// and all cards already dealt
pub struct Deck {
    pub deck: Vec<Card>,
    pub dealt: Vec<Card>
}

impl Deck {
    
    // returns new deck with 56 cards
    pub fn new() -> Deck {

        let mut new_deck = Deck {
            deck: Vec::with_capacity(56),
            dealt: Vec::with_capacity(56),
        };
        
        // add number cards to deck
        new_deck.deck.extend({
            let mut num_cards = vec![];
            for i in 2..=10 {
                for suit in Suit::all_suits() {
                    num_cards.push(Card::Number(i, suit));
                }
            }
            // rust is an expression based language, so this entire
            // {} block is replaced by whatever is put in the last line
            // without a semi-colon
            num_cards
        }.iter());

        // add face cards to deck
        new_deck.deck.extend({
            let mut face_cards = vec![];
            for face in Face::all_faces() {
                for suit in Suit::all_suits() {
                    face_cards.push(Card::Face(face, suit));
                }
            }
            face_cards
        }.iter());

        // add jokers to deck
        new_deck.deck.extend([Card::Joker; 4].iter());
        // shuffle
        new_deck.deck.shuffle(&mut thread_rng());
        new_deck
    }

    // moves card to dealt vector and returns it
    // reshuffling if the deck is empty
    pub fn draw(&mut self) -> Card {
        if self.deck.is_empty() {
            self.reshuffle();
        }
        let card = self.deck.remove(0);
        self.dealt.push(card);
        card
    }

    // moves all cards back to the deck and shuffles
    pub fn reshuffle(&mut self) {
        println!("\t[Deck empty, re-shuffling]");
        self.deck.extend(self.dealt.drain(..));
        self.deck.shuffle(&mut thread_rng());
    }
}



