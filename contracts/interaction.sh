#!/bin/bash

#Fill the below details before executing script
CONTRACT_ADDR=""
CODE_ID=
wallet=""

echo ">>>>>>>>>>>>>>>  Welcome to the Bingo Game! <<<<<<<<<<<<<<<<"
echo "Please select an option:"
echo "1. Create new Game"
echo "2. Start newly created game"
echo "3. Join Bingo Game"
echo "4. Draw number"
echo "5. Get total games"
echo "6. Get active games ID"
echo "7. Get Player details of particular game"
echo "8. Get all drawed numbers in a game"


read  -p "Enter your choice (1, 2, 3, ...): " choice

case $choice in
    1)
        echo "Option 1 selected: Create new game"
        # Fill the below JSON as per your's requirement
        NEW_GAME='{"CreateNewGame": {"min_join_duration": , "min_turn_duration": , "entry_fee": , "token_address": ""}}'
        osmosisd tx wasm execute $CONTRACT_ADDR "$NEW_GAME" --from $wallet --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y        
        ;;
    2)
        echo "Option 2 selected: start newly created game"
        # Fill the game_id for which game is to be started
        START_GAME='{"StartGame": {"game_id: "}}'
        osmosisd tx wasm execute $CONTRACT_ADDR "$START_GAME" --from $wallet --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y        
        ;;
    3)
        echo "Option 3 selected: Join Game"
        # FIll up the game_id for which user wants to join game
        JOIN_GAME='{"JoinGame": {"game_id: "}}'
        osmosisd tx wasm execute $CONTRACT_ADDR "$JOIN_GAME" --from $wallet --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y
        ;;

    4)
        echo "Option 4 selected:  Draw a number"
        #Fill the game_id for which player wants to draw number
        DRAW_NUMBER='{"DrawNumber": {"game_id: "}}'
        osmosisd tx wasm execute $CONTRACT_ADDR "$DRAW_NUMBER" --from $wallet --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y
        ;;
        
    5)
        echo "Option 5 selected:  Total Games count"
        TOTAL_GAMES='{"TotalGames": {}}'
        osmosisd query wasm contract-state smart $CONTRACT_ADDR "$TOTAL_GAMES" --output json
        ;;
    6)
        echo "Option 6 selected:  Active Game IDs"
        ACTIVE_GAMES='{"ActiveGames": {}}'
        osmosisd query wasm contract-state smart $CONTRACT_ADDR "$ACTIVE_GAMES" --output json
        ;;
    7)
        echo "Option 7 selected:  Player Details of a Game"
        #Fill the game_id and player_address to get the details of that player in a game
        PLAYER_DETAILS='{"PlayerDetails": {"game_id": , "player_address": ""}}'
        osmosisd query wasm contract-state smart $CONTRACT_ADDR "$PLAYER_DETAILS" --output json
        ;;
    8)
        echo "Option 8 selected:  Get all drawed numbers in a game"
        #Fill the game_id for which player wants to draw number
        DRAWS_NUMBERS='{"DrawsNumbers": {"game_id": }}'
        osmosisd query wasm contract-state smart $CONTRACT_ADDR "$DRAWS_NUMBERS" --output json
        ;;

    *)  # This is the default case for unmatched inputs
        echo "Invalid choice. Please enter a valid option."
        ;;
esac
