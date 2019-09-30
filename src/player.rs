use crate::card::Card;

// simple struct holding player info
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub drawed: Option<Card>
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            score: 0,
            drawed: None
        }
    }
}