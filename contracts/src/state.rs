use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BingoGame {
    // id -> Addr -> Bingo
    pub players: Vec<Option<Addr>>,
    pub number_draws: Vec<Option<u64>> ,
    pub status: GameStatus,
    pub entry_fee: Option<u32>,
    pub min_join_duration: u64,
    pub min_turn_duration: u64,
    pub winner: Option<Addr>, // Address of the winning player
    pub pot: u32,             // Total pot accumulated of entry fees till min_join_duration
    pub current_chance: i64,
}
 

// Player information
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Player {
    pub board: [[Option<u64>; 5]; 5], // 5x5 Bingo board
    pub bingo: bool, // Whether the player achieved a Bingo
}

// Game status enum
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum GameStatus {
    NotStarted,
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
// Game_id -> PLayer_Address -> Player_details
pub const PLAYERS: Map<(u64, Addr), Option<Player>> = Map::new("Players");
