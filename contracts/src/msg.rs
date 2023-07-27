use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
    CreateNewGame {
        min_join_duration: u64,
        min_turn_duration: u64,
        entry_fee: u128,
        token_address: Addr,
    },
    JoinGame {
        game_id: u64,
    },
    StartGame {
        game_id: u64,
    },
    DrawNumber {
        game_id: u64,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    TotalGames {},
    PlayerDetails { game_id: u64, player_address: Addr },
    DrawsNumbers { game_id: u64 },
    ActiveGames {},
}
