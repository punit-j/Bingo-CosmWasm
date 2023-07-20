use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BingoGame {
    players: Vec<Player>,
    board: [[Option<u32>; 5]; 5], // 5x5 Bingo board
    status: GameStatus,
    entry_fee: u32,
    min_join_duration: u64,
    min_turn_duration: u64,
    winner: Option<Addr>, // Address of the winning player
    pot: u32,             // Total pot accumulated of entry fees till min_join_duration
}

// Player information
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Player {
    address: Addr, // Address of the player's account
    bingo: bool,   // Whether the player achieved a Bingo
}

// Game status enum
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
enum GameStatus {
    Waiting,
    Ongoing,
    Finished,
}

// Board cell status enum
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
enum CellStatus {
    Marked(u32),
    Unmarked,
}

pub const GAMES: Map<u64, BingoGame> = Map::new("bingo");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const TOTAL_GAMES: Item<u64> = Item::new("totalgames");
