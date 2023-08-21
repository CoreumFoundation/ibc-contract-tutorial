# Coreum IBC Smart Contract Tutorial

# ================== CONFIGURATION ==================
# COREUM_CHAIN_ID="coreum-devnet-1"
# COREUM_DENOM=udevcore
# COREUM_NODE=http://localhost:26657


COREUM_CHAIN_ID="coreum-mainnet-1"
COREUM_DENOM=ucore
COREUM_NODE=https://full-node.mainnet-1.coreum.dev:26657

COREUM_VERSION="{Cored version}"
COREUM_CHAIN_ID_ARGS=--chain-id=$(COREUM_CHAIN_ID)
COREUM_NODE_ARGS=--node=$(COREUM_NODE)
COREUM_HOME=$(HOME)/.core/"$(COREUM_CHAIN_ID)"
COREUM_BINARY_NAME=$(shell arch | sed s/aarch64/cored-linux-arm64/ | sed s/x86_64/cored-linux-amd64/)

DEV_WALLET=dev-wallet
CODE_ID=1
SUBUNIT=gil
_CONTRACT_ADDRESS_=devcore14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sd4f0ak
_WALLET_ADDRESS_=devcore17vlkqgpxnrqqrgq877v7a4ymfma0r03g02agg8

IBC_TRANSFER_CHANNEL=channel-0
IBC_TRANSFER_RECEIVER=osmo1pwvcapna75slt3uscvupfe52492yuzhflhakem
IBC_TRANSFER_AMOUNT=1000udevcore
IBC_TRANSFER_SENDER=${DEV_WALLET}

# ================== DEVELOPMENT & TESTING ==================
dev:
	@echo "Building the contract..."
	@echo "${PWD}"
	@echo `basename "${PWD}"`
	cargo build

test:
	@echo "Running the unit tests..."
	cargo test -- --nocapture

# ================== DEPLOYMENT ================== note: old optimizer was 0.12.6
build:
	@echo "Building the contract using Docker..."
	docker run --rm -v "${PWD}":/code --mount type=volume,source=`basename "${PWD}"`_cache,target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.14.0

deploy:
	@echo "Deploying the contract to the chain..."
	RES=$$(cored-00 tx wasm store artifacts/ibc_tutorial.wasm --from ${DEV_WALLET} --gas auto --gas-adjustment 1.3 -y -b block --output json "$(COREUM_NODE_ARGS)" "$(COREUM_CHAIN_ID_ARGS)") ; \
	echo $$RES ; \
	CODE_ID=$$(echo $$RES | jq -r '.logs[0].events[-1].attributes[-1].value') ; \
	echo "Code ID: $$CODE_ID"

instantiate:
	@echo "Instantiating the contract..."
	cored-00 tx wasm instantiate $(CODE_ID) "{}" \
	--amount="10000000$(COREUM_DENOM)" --no-admin --label "dev test" --from ${DEV_WALLET} --gas auto --gas-adjustment 1.3 -b block -y $(COREUM_NODE_ARGS) $(COREUM_CHAIN_ID_ARGS)

contract_address:
	@echo "Fetching the contract address..."
	CONTRACT_ADDRESS=$(shell cored-00 q wasm list-contract-by-code $(CODE_ID) --output json $(COREUM_NODE_ARGS) $(COREUM_CHAIN_ID_ARGS) | jq -r '.contracts[-1]')
	@echo $$CONTRACT_ADDRESS

# ================== UTILITIES ==================
keys:
	@echo "Listing the keys..."
	cored-00 keys list

check:
	@echo "Checking smart contract information..."
	cored-00 q wasm code-info $(CODE_ID) $(COREUM_NODE_ARGS) $(COREUM_CHAIN_ID_ARGS)

q:
	@echo "Querying bank balances..."
	cored-00 q bank balances $(_WALLET_ADDRESS_)

fund:
	@echo "Funding a specific address..."
	cored-00 tx bank send alice $(_WALLET_ADDRESS_) 10000000udevcore --broadcast-mode=block

# =================== QUERIES ==========================
get_count:
	cored-00 q wasm contract-state smart $(_CONTRACT_ADDRESS_) '{"get_count": {"channel": "channel-0"}}' $(COREUM_NODE_ARGS) $(COREUM_CHAIN_ID_ARGS)
get_timeout_count:
	cored-00 q wasm contract-state smart $(_CONTRACT_ADDRESS_) '{"get_timeout_count": {"channel": "channel-0"}}' $(COREUM_NODE_ARGS) $(COREUM_CHAIN_ID_ARGS)

# =================== IBC  ==========================
channels:
	cored-00 query ibc channel channels $(COREUM_CHAIN_ID_ARGS) $(COREUM_NODE_ARGS)
ibc_transfer_cli:
	@echo "Initiating IBC transfer..."
	cored-00 tx ibc-transfer transfer transfer $(IBC_TRANSFER_CHANNEL) $(IBC_TRANSFER_RECEIVER) $(IBC_TRANSFER_AMOUNT) --from $(IBC_TRANSFER_SENDER) $(COREUM_CHAIN_ID_ARGS) $(COREUM_NODE_ARGS)
ibc_call_wasm_timeout:
	cored-00 tx wasm execute $(_CONTRACT_ADDRESS_) "{\"increment\":{\"channel\":\"channel-0\", \"timeout\": { \"timestamp\": \"9999999000001000000\" }}}" --from ${DEV_WALLET} -b block -y $(COREUM_NODE_ARGS) $(COREUM_CHAIN_ID_ARGS)
