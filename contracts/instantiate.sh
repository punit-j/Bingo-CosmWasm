#pass the desiredvalid address on osmosis chain for contract admin
INIT='{"admin": }'
# code_id of the contract deployed on osmosis
CODE_ID=
wallet=""

#instantiate the contract
osmosisd tx wasm instantiate $CODE_ID "$INIT" \
    --from $wallet \
    --label "my contract" \
    --gas-prices 0.025uosmo \
    --gas auto \
    --gas-adjustment 1.3 -b block -y \
    --no-admin