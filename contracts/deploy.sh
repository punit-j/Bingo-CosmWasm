# store the code on chain

# Wallet address on osmosis
wallet=

RES=$(osmosisd tx wasm store artifacts/contracts.wasm \
--from $wallet \
--gas-prices 0.1uosmo \
--gas auto \
--gas-adjustment 1.3 -y \
--output json -b block)

CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo $CODE_ID

#CODE_ID = 