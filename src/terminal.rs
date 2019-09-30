//! # Terminal Module
//! Module responsible for terminal I/O 

use crate::player::Player;
use std::io::Write;
use std::time::Duration;

const GREETING_TOP: &'static str = "\n /------- ----------------------- -------\\ \n";
const GREETING_MID: &'static str = "| * * *   Welcome to Cardgame.rs!   * * * | \n";
const GREETING_BOT: &'static str = " \\------- ----------------------- -------/ \n";

// fancy animated greeting
pub fn display_greeting() {

    const SLEEP_TIME: Duration = Duration::from_millis(15);

    for c in GREETING_TOP.chars() {
        print!("{}", c);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(SLEEP_TIME);
    }

    for c in GREETING_MID.chars() {
        print!("{}", c);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(SLEEP_TIME);
    }

    for c in GREETING_BOT.chars() {
        print!("{}", c);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(SLEEP_TIME);
    }
}

// prompts the user to input the number of players
// returns Some(num) on valid inpit
// and None on invalid input 
// that is why the return type is Optional
pub fn get_player_count() -> Option<u32> {

    let mut buf = String::new();
    print!("-- Number of players: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buf).unwrap();

    match buf.trim().parse::<u32>() {
        // if the input is valid, return it within a Some()
        Ok(num) if num >=2 && num <= 4 => 
            Some(num),
        // if the input is invalid, return None
        _ => { 
            eprintln!("-- Please enter a valid number from 2 to 4\n");
            std::io::stdout().flush().unwrap();
            None
        }
    }
}

// prompts user to input name for n'th player
// inputs cannot be invalid, so the return type is plain String
pub fn get_player_name(id: u32) -> String {
    let mut buf = String::new();
    print!("\t[Player {}] Enter name: ", id+1);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

// display game score held within the player state
pub fn display_score(players: &mut Vec<Player>) {
    players.sort_by(|p1, p2| p1.score.cmp(&p2.score));
    println!("\t-------- Scoreboard --------");
    for player in players.iter().rev() {
        let name = if player.name.len() > 19 {
            let mut n = player.name.clone();
            n.truncate(19);
            n.push_str("..");
            n
        } else {
            player.name.clone()
        };
        println!("\t| {0: <22}{1: <2} |", name, player.score);
    }
    println!("\t----------------------------\n");
}

pub fn display_round(round: i32) {
    println!("  [-+-+-+-+-+-+-+- ROUND {} -+-+-+-+-+-+-+-]\n", round);
}

pub fn query_draw(player: &Player) {
    let mut key_enter = String::new();
    print!("{}'s turn: Press enter to draw card ", player.name);
    std::io::stdout().flush().unwrap();
    // wait until next enter input
    std::io::stdin().read_line(&mut key_enter).unwrap();
}

pub fn display_draw(player: &Player) {
    match player.drawed.unwrap() {
        Card::Joker => print!("\t{} has drawn a penalty card {} -- (-1 points)\n\n",
            player.name, player.drawed.unwrap()),
        _ => print!("\t{} has drawn {}\n\n", player.name, player.drawed.unwrap())
    }
    std::io::stdout().flush().unwrap();
}

pub fn display_round_winner(player: &Player) {
    print!("{} has won the round by drawing {}\n\n", player.name, player.drawed.unwrap());
}

pub fn display_winner(player: &Player) {
    print!("\n\t{} has won the game!\n\n\n", player.name);
}

// breaks up visual clutter in terminal
pub fn break_line(lines: u32) {
    for _ in 0..lines {
        println!("");
    }
    std::io::stdout().flush().unwrap();
}


use crate::card::{Card, Suit, Face};
use std::fmt;

// how the card enum is displayed in console
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Card::Number(i, suit) => write!(f, "[{} of {}s]", to_eng(i), suit),
            Card::Face(face, suit) => write!(f, "[{} of {}s]", face, suit),
            Card::Joker => write!(f, "[Joker]"),
        }
        
    }
}

// how the suit enum is displayed in console
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Spade => write!(f, "Spade"),
            Suit::Heart => write!(f, "Heart"),
            Suit::Diamond => write!(f, "Diamond"),
            Suit::Club => write!(f, "Club"),
        }
    }
}

// how the face enum is displayed in console
impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Face::Jack => write!(f, "Jack"),
            Face::Queen => write!(f, "Queen"),
            Face::King => write!(f, "King"),
            Face::Ace => write!(f, "Ace"),
        }
    }
}

// converts card number to english text
fn to_eng(i: &i32) -> &'static str {
    match i {
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => ""
    }
}

