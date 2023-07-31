# BINGO Cosm-Wasm
---

## Pre-Requisite :slot_machine:
Before proceeding further one must needs to meet the following pre-requirements:
* Setup Cosm-wasm account on any  working testnet. Lets take [Osmosis](https://osmosis.zone/) chain for now.
* Setup the osmosisd client on your machine by folowing setups in the [doc](https://docs.osmosis.zone/cosmwasm/testnet/cosmwasm-deployment/).
* For getting osmosis native test-tokens on your account you can use osmosis public [faucet](https://faucet.testnet.osmosis.zone/).
* Your system also need to have docker installed, to build the contract wasm. If it's not then follow up the [doc](https://docs.docker.com/engine/install/).
  
## Build Contract :dvd::-
To build the contarct run:-
```bash
sudo bash ./contracts/build.sh
```


## Deploy Contract :floppy_disk::-
After meeting all the pre-requisites one can deploy the builded contract wasm on above setup network. Run below command:-
```bash
sudo bash ./contarcts/deploy.sh
```
**NOTE**:-
  1. Wallet address is to be feeded in `deploy.sh` before running the above cmd.
  2. After successful deployment please note the returned `CODE_ID`, it will be use while interacting with deployed contract.

## Instantiate Contract :calling::-
Contract instantiation is initial call to a contract to setup it's state and accessifiers. To instantiate the contract run:-
```bash
sudo bash ./contracts/instantiate.sh
```
**NOTE:-**
  1. Before running above cmd feed admin-address  in `INIT='{"admin": }'` and CODE_ID of the deployed contarct.
  2. Only this passed admin can create new game so be catious before feeding it.

## Interact With Bingo-game :video_game::-
To interact the contract run the below cmd:
```bash
sudo bash ./contracts/interaction.sh
```
- A promt will appear which shows different option choice to interact with the contract.
**NOTE**: Fill All the required details in .`contracts/interaction.sh` before running the above cmd.