use rand::Rng;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{BingoGame, GameStatus, Player, ADMIN, GAMES, PLAYERS, TOTAL_GAMES};

use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let contract_admin: StdResult<Addr> = Ok(msg.admin);
    ADMIN.save(deps.storage, &contract_admin?)?;
    TOTAL_GAMES.save(deps.storage, &0)?;
    Ok(Response::new())
}

#[allow(dead_code)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    use ExecuteMsg::*;

    match msg {
        CreateNewGame {
            min_join_duration,
            min_turn_duration,
            entry_fee,
        } => execute::create_new_game(deps, info, min_join_duration, min_turn_duration, entry_fee),
        JoinGame { game_id } => execute::join_bingo_game(deps, info, game_id),
        StartGame { game_id } => execute::start_game(deps, info, game_id),
        DrawNumber { game_id } => execute::draw_number(deps, info, env, game_id),
    }
}

mod execute {
    use super::*;
    use cosmwasm_std::StdError;

    pub fn create_new_game(
        deps: DepsMut,
        info: MessageInfo,
        min_join_duration: u64,
        min_turn_duration: u64,
        entry_fee: u32,
    ) -> StdResult<Response> {
        let contract_admin = ADMIN.load(deps.storage)?;
        if contract_admin != info.sender {
            return Err(StdError::generic_err(
                "Unauthorised access: Only Admin can start a new game",
            ));
        }
        let mut total_games_count = TOTAL_GAMES.load(deps.storage)?;
        total_games_count += 1;
        let game_details = BingoGame {
            players: vec![],
            number_draws: Vec::new(),
            status: GameStatus::NotStarted,
            entry_fee: Some(entry_fee),
            min_join_duration,
            min_turn_duration,
            winner: None,
            pot: 0,
            current_chance: -999,
        };
        GAMES.save(deps.storage, total_games_count, &game_details)?;
        TOTAL_GAMES.save(deps.storage, &total_games_count)?;

        Ok(Response::new())
    }

    pub fn join_bingo_game(deps: DepsMut, info: MessageInfo, game_id: u64) -> StdResult<Response> {
        let bingo_game = GAMES.load(deps.storage, game_id)?;
        let contract_admin = ADMIN.load(deps.storage)?;
        let players = PLAYERS.may_load(deps.storage, (game_id, info.sender.to_owned()))?;
        if contract_admin == info.sender {
            return Err(StdError::generic_err(
                "Bingo Game: Admin cannot join the game",
            ));
        }
        if bingo_game.status != GameStatus::Waiting {
            return Err(StdError::generic_err(
                "Bingo Game: Game is already started or finished",
            ));
        }

        if players.is_some() {
            return Err(StdError::generic_err(
                "Join Game: Player already joined the game",
            ));
        }

        if bingo_game.players.contains(&Some(info.sender.to_owned())) {
            return Err(StdError::generic_err(
                "Bingo Game: Player already joined this game",
            ));
        }
        GAMES.update(deps.storage, game_id, |game| {
            if let Some(mut game) = game {
                game.players.push(Some(info.sender.to_owned()));
                game.current_chance = 0;
                Ok(game)
            } else {
                Err(StdError::generic_err(
                    "Bingo Game: Player not added in the Game",
                ))
            }
        })?;
        PLAYERS.save(
            deps.storage,
            (game_id, info.sender.to_owned()),
            &(Player {
                board: get_2d_array_with_random_number(),
                bingo: false,
            }),
        )?;

        Ok(Response::new())
    }

    pub fn start_game(deps: DepsMut, info: MessageInfo, game_id: u64) -> StdResult<Response> {
        let mut bingo_game = GAMES.load(deps.storage, game_id)?;
        let contract_admin = ADMIN.load(deps.storage)?;
        if contract_admin != info.sender {
            return Err(StdError::generic_err(
                "Bingo Game: Only Admin can start the game",
            ));
        }
        if bingo_game.status != GameStatus::NotStarted {
            return Err(StdError::generic_err(
                "Bingo Game: Game is already started or finished",
            ));
        }
        bingo_game.status = GameStatus::Waiting;
        GAMES.update(deps.storage, game_id, |game| {
            if let Some(mut game) = game {
                game.status = GameStatus::Waiting;
                Ok(game)
            } else {
                Err(StdError::generic_err("Bingo Game: Game not started"))
            }
        })?;
        Ok(Response::new())
    }
    //TODO: cw20 integration and claim_bingo() and integration tests
    // Players draws number till some player claims their bingo as success
    pub fn draw_number(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        game_id: u64,
    ) -> StdResult<Response> {
        let bingo_game = GAMES.load(deps.storage, game_id)?;
        if bingo_game.min_join_duration > env.block.time.seconds() {
            return Err(StdError::generic_err(format!(
                "Game will start after {} seconds",
                bingo_game.min_join_duration - env.block.time.seconds()
            )));
        }
        let mut current_turn_player = bingo_game.current_chance;
        let total_players_in_game = bingo_game.players.len() as i64;

        if &Some(info.sender) != &bingo_game.players[current_turn_player as usize] {
            return Err(StdError::generic_err(format!(
                "This chance is for player: {:?}",
                &bingo_game.players[current_turn_player as usize]
                    .clone()
                    .unwrap()
                    .as_str()
            )));
        };
        // circular array
        current_turn_player = (current_turn_player + 1) % total_players_in_game;
        let number = generate_random_number();
        GAMES.update(deps.storage, game_id, |game| {
            if let Some(mut game) = game {
                game.status = GameStatus::Ongoing;
                game.current_chance = current_turn_player;
                game.number_draws.push(number);
                Ok(game)
            } else {
                Err(StdError::generic_err(
                    "Bingo Game: Player chance not updated",
                ))
            }
        })?;

        Ok(Response::new())
    }

    pub fn claim_bingo(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        game_id: u64,
    ) -> StdResult<Response> {
        let bingo_game = GAMES.load(deps.storage, game_id)?;
        let claimer_details = PLAYERS.load(deps.storage, (game_id, info.sender.to_owned()))?;

        if bingo_game.status != GameStatus::Ongoing {
            return Err(StdError::generic_err(
                "Bingo Game: Cannot claim at this moment",
            ));
        };
        if !bingo_game.players.contains(&Some(info.sender.to_owned())) {
            return Err(StdError::generic_err(
                "Bingo Game: Claimer is not participant of game",
            ));
        }

        if claimer_details.to_owned().bingo {
            return Err(StdError::generic_err(
                "Bingo Game: Claimer Already won this game",
            ));
        }

        let claimer_board = claimer_details.board;
        let game_draws_numbers = bingo_game.number_draws;

        let mut all_values_present = true;
        // Checking player board contains all draws value till now.
        for row in claimer_board.iter() {
            for board_number in row.iter() {
                if !game_draws_numbers.contains(board_number) {
                    all_values_present = false;
                }
            }
        }

        if !all_values_present {
            return Err(StdError::generic_err(
                "Bingo Game: Players board numbers didn't matched with draws numbers",
            ));
        }

        GAMES.update(deps.storage, game_id, |game| {
            if let Some(mut game) = game {
                game.status = GameStatus::Finished;
                game.winner = Some(info.sender.to_owned());
                Ok(game)
            } else {
                Err(StdError::generic_err("Bingo Game: Game data not updated"))
            }
        })?;

        PLAYERS.update(deps.storage, (game_id, info.sender), |player| {
            if let Some(mut player) = player {
                player.bingo = true;
                Ok(player)
            } else {
                Err(StdError::generic_err("Bingo Game: Player data not updated"))
            }
        })?;

        Ok(Response::new())
    }
}

fn generate_random_number() -> Option<u64> {
    let mut rng = rand::thread_rng();
    Some(rng.gen_range(0..=255))
}

fn get_2d_array_with_random_number() -> [[Option<u64>; 5]; 5] {
    let mut array: [[Option<u64>; 5]; 5] = [[None; 5]; 5];
    let mut count = 0;

    while count < 25 {
        let row = count / 5;
        let col = count % 5;
        array[row][col] = generate_random_number();
        count += 1;
    }

    array
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        TotalGames {} => to_binary(&query::get_total_games(deps)?),
        PlayerDetails {
            game_id,
            player_address,
        } => to_binary(&query::get_player(deps, game_id, player_address)?),
        DrawsNumbers { game_id } => to_binary(&query::get_draws_number(deps, game_id)?),
    }
}
mod query {
    use super::*;
    use cosmwasm_std::StdError;

    pub fn get_total_games(deps: Deps) -> StdResult<Option<u64>> {
        let total_games = TOTAL_GAMES.may_load(deps.storage)?;
        Ok(total_games)
    }

    pub fn get_player(deps: Deps, game_id: u64, player_address: Addr) -> StdResult<Player> {
        let player = PLAYERS
            .may_load(deps.storage, (game_id, player_address))?
            .unwrap();
        Ok(player)
    }

    pub fn get_draws_number(deps: Deps, game_id: u64) -> StdResult<Vec<Option<u64>>> {
        let game = GAMES.may_load(deps.storage, game_id)?;
        if game
            .to_owned()
            .expect("No game found for this id")
            .number_draws
            .is_empty()
        {
            return Err(StdError::generic_err("Bingo Game: No number draws yet"));
        }
        Ok(game.unwrap().number_draws)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::GameStatus;
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &[]);
        let admin_address = Addr::unchecked("bingoadmin");
        let instantiate_response = instantiate(
            deps.as_mut(),
            env.to_owned(),
            info,
            InstantiateMsg {
                admin: admin_address,
            },
        );
        println!("Instantiate response: \n {:?}", instantiate_response);
    }

    #[test]
    fn test_bingo_state_with_multi_test() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);

        let code_id = app.store_code(Box::new(code));

        let sender = Addr::unchecked("owner");

        let contract_addr = app
            .instantiate_contract(
                code_id,
                sender.to_owned(),
                &InstantiateMsg {
                    admin: sender.to_owned(),
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        println!("Code ID: {}", code_id);
        println!("Sender: {:?}", sender);
        println!("Address: {:?}", contract_addr);

        let total_games_resp: Option<u64> = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::TotalGames {})
            .unwrap();

        println!("Total games resp: {:?}", total_games_resp);
        assert_eq!(total_games_resp, Some(0), "assertion failed");
    }

    #[test]
    fn test_bingo_create_new_game() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);

        let code_id = app.store_code(Box::new(code));

        let admin = Addr::unchecked("owner");

        let contract_addr = app
            .instantiate_contract(
                code_id,
                admin.to_owned(),
                &InstantiateMsg {
                    admin: admin.to_owned(),
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        println!("Code ID: {}", code_id);
        println!("Admin: {:?}", admin);
        println!("Address: {:?}", contract_addr);

        let total_games_before_resp: Option<u64> = app
            .wrap()
            .query_wasm_smart(contract_addr.to_owned(), &QueryMsg::TotalGames {})
            .unwrap();

        println!("Total games before resp: {:?}", total_games_before_resp);
        assert_eq!(total_games_before_resp, Some(0), "assertion failed");

        let mock_game_details = BingoGame {
            players: vec![None],
            number_draws: vec![None],
            status: GameStatus::Waiting,
            entry_fee: None,
            min_join_duration: 0,
            min_turn_duration: 0,
            winner: None,
            pot: 0,
            current_chance: -1,
        };

        let _call_to_create_new_game = app
            .execute_contract(
                admin.to_owned(),
                contract_addr.to_owned(),
                &ExecuteMsg::CreateNewGame {
                    min_join_duration: 0,
                    min_turn_duration: 0,
                    entry_fee: 0,
                },
                &[],
            )
            .unwrap();

        let total_games_after_resp: Option<u64> = app
            .wrap()
            .query_wasm_smart(contract_addr.to_owned(), &QueryMsg::TotalGames {})
            .unwrap();

        println!(
            "Total games After adding resp: {:?}",
            total_games_after_resp
        );

        //start new game
        let _call_to_start_game = app
            .execute_contract(
                admin.to_owned(),
                contract_addr.to_owned(),
                &ExecuteMsg::StartGame { game_id: 1 },
                &[],
            )
            .unwrap();

        // join new game
        let player1_add = Addr::unchecked("player1");
        let player2_add = Addr::unchecked("player2");

        let _call_to_join_game = app
            .execute_contract(
                player1_add.to_owned(),
                contract_addr.to_owned(),
                &ExecuteMsg::JoinGame { game_id: 1 },
                &[],
            )
            .unwrap();
        //TODO: Game should not be join again
        // let _call2_to_join_game = app.execute_contract(player_add.to_owned(), contract_addr.to_owned(), &ExecuteMsg::JoinGame { game_id: 1 }, &[]).unwrap();

        let _call2_to_join_game = app
            .execute_contract(
                player2_add.to_owned(),
                contract_addr.to_owned(),
                &ExecuteMsg::JoinGame { game_id: 1 },
                &[],
            )
            .unwrap();

        let player1_details: Player = app
            .wrap()
            .query_wasm_smart(
                contract_addr.to_owned(),
                &QueryMsg::PlayerDetails {
                    game_id: 1,
                    player_address: player1_add.to_owned(),
                },
            )
            .unwrap();

        let player2_details: Player = app
            .wrap()
            .query_wasm_smart(
                contract_addr.to_owned(),
                &QueryMsg::PlayerDetails {
                    game_id: 1,
                    player_address: player2_add.to_owned(),
                },
            )
            .unwrap();

        println!("GAME BOARD of PLAYER-1: \n {:?}\n", player1_details);
        println!("GAME BOARD of PLAYER-2: \n {:?}\n", player2_details);

        for _i in 0..20 {
            let _call_to_draw_number_by_player1 = app
                .execute_contract(
                    player1_add.to_owned(),
                    contract_addr.to_owned(),
                    &ExecuteMsg::DrawNumber { game_id: 1 },
                    &[],
                )
                .unwrap();

            let _call_to_draw_number_by_player2 = app
                .execute_contract(
                    player2_add.to_owned(),
                    contract_addr.to_owned(),
                    &ExecuteMsg::DrawNumber { game_id: 1 },
                    &[],
                )
                .unwrap();
        }
        let draws_number: Vec<Option<u64>> = app
            .wrap()
            .query_wasm_smart(
                contract_addr.to_owned(),
                &QueryMsg::DrawsNumbers { game_id: 1 },
            )
            .unwrap();

        println!("DRAWS NUMBERS: \n {:?}\n", draws_number);
    }
}
