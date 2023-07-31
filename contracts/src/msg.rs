use cosmwasm_std::Addr;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
}

#[cw_serde]
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

#[cw_serde]
pub enum QueryMsg {
    TotalGames {},
    PlayerDetails { game_id: u64, player_address: Addr },
    DrawsNumbers { game_id: u64 },
    ActiveGames {},
}
