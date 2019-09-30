mod player;
mod terminal;
mod card;
mod deck;

use crate::{
    deck::Deck, 
    card::Card,
    player::Player,
    terminal::*
};

fn main() {

    display_greeting();
    break_line(2);

    let player_count = loop {
        if let Some(player_count) = get_player_count() {
            break player_count;
        }
    };
    break_line(1);

    // collect players
    let mut players = Vec::with_capacity(player_count as usize);
    for i in 0..player_count {
        let name = get_player_name(i);
        players.push(Player::new(name));
    }
    break_line(2);

    // build deck
    let mut deck = Deck::new();

    let mut round = 1;
    let mut running = true;
    // main game loop
    while running {

        display_round(round);

        // move players so rust's aliasing rules aren't broken
        let mut players_moved = std::mem::replace(&mut players, Vec::new());
        
        // let all players draw their card
        for player in players_moved.iter_mut() {
            query_draw(&player);
            let card = deck.draw();
            player.drawed = Some(card);
            display_draw(&player);
        }

        // get round winner
        players_moved.sort_by(|p1, p2| 
            p2.drawed.unwrap().cmp(&p1.drawed.unwrap()));
        display_round_winner(&players_moved[0]);
        players_moved[0].score += 2;
        
        // penalize players that drew Joker
        players_moved.iter_mut()
            .filter(|p| p.drawed == Some(Card::Joker))
            .for_each(|p| p.score = (p.score -1).max(0));

        // move players back
        players = players_moved;

        // check if there is winner
        if let Some(player) = players.iter().find(|p| p.score >= 21) {
            // make sure there are no players trailing by 1 point
            if let None = players.iter().find(|p| p.score == player.score -1) {
                display_winner(player);
                running = false;
            }
        }

        // display players in order of score on scoreboard
        players.sort_by(|p1, p2| p1.score.cmp(&p2.score));
        display_score(&mut players);

        round += 1;
    }
}


