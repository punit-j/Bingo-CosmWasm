use std::env;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{BingoGame, ADMIN, GAMES, TOTAL_GAMES};

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
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    use ExecuteMsg::*;

    match msg {
        StartNewGame { game_details } => execute::start_new_game(deps, info, game_details),
    }
}

mod execute {
    use super::*;
    use cosmwasm_std::StdError;

    pub fn start_new_game(
        deps: DepsMut,
        info: MessageInfo,
        game_details: BingoGame,
    ) -> StdResult<Response> {
        let contract_admin = ADMIN.load(deps.storage)?;
        let mut total_games_count = TOTAL_GAMES.load(deps.storage)?;
        if contract_admin != info.sender {
            return Err(StdError::generic_err(
                "Unauthorised access: Only Admin can start a new game",
            ));
        }
        total_games_count += 1;
        GAMES.save(deps.storage, total_games_count, &game_details)?;
        TOTAL_GAMES.save(deps.storage, &total_games_count)?;

        Ok(Response::new())
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        TotalGames {} => to_binary(&query::get_total_games(deps)?),
    }
}
mod query {
    use super::*;

    pub fn get_total_games(deps: Deps) -> StdResult<Option<u64>> {
        let total_games = TOTAL_GAMES.may_load(deps.storage)?;
        Ok(total_games)
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
}
