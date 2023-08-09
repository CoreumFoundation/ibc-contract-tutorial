# Coreum IBC Tutorial

## Overview

This tutorial provides a comprehensive guide to the creating a basic IBC smart contract application on the Coreum Blockchain.

## Table of Contents

1. [Introduction](#introduction)
2. [Makefile Overview](#makefile-overview)
3. [contract.rs Overview](#contractrs-overview)
4. [Getting Started](#getting-started)
5. [Contribution & Issues](#contribution--issues)
6. [Main Module](#main-module)
7. [State Module (state.rs)](#state-module-staters)
8. [Acknowledgment Module (ack.rs)](#acknowledgment-module-ackrs)
9. [Message Module (msg.rs)](#message-module-msgrs)

## Introduction

This README provides an overview of the included Makefile and the `contract.rs` script. The Makefile helps in automating several tasks related to the Coreum blockchain platform, such as development, testing, deployment, and management of smart contracts. The `contract.rs` file contains the Rust-based smart contract logic, which can be deployed onto the Coreum blockchain.

[Jump to Table of Contents](#table-of-contents)

## Makefile Overview

### Variables

- `COREUM_CHAIN_ID`: The ID for the Coreum blockchain.
- `COREUM_DENOM`: The denomination or currency of the Coreum network.
- `COREUM_NODE`: The node address for the Coreum blockchain.
- `COREUM_HOME`: The home directory for the Coreum blockchain.
- `DEV_WALLET`: Name of the developer wallet.
- `CODE_ID`: The ID for the smart contract. You should replace this after instantiating your contract.
- `SUBUNIT`: Denotes a subdivision of the currency unit.

### Commands

- `dev`: Print the current working directory and build the project.
- `test`: Run cargo tests without capturing the output.
- `add_account`: Add a development account using the specified wallet.
- `build`: Build the contract inside a docker container.
- `deploy`: Deploy the contract onto the Coreum blockchain.
- `check`: Check the smart contract information.
- `keys`: List the keys associated with the Coreum node.
- `q`: Query the bank balances for a specific address.
- `fund`: Send funds to a specific address.
- `instantiate`: Instantiate the contract with initial funds and parameters.
- `contract_address`: Get and display the address of the instantiated contract.

[Jump to Table of Contents](#table-of-contents)

## contract.rs Overview

The `contract.rs` file contains the logic for a Rust-based smart contract that can be executed on the Coreum blockchain platform. 

### Major Components:

1. **Imports**: Import necessary components from `cosmwasm_std` library and local modules.
2. **Constants**: Specify contract name and version.
3. **Entry Points**: Functions like `instantiate`, `execute`, and `query` that define the behavior of the contract upon instantiation, execution, and querying.
4. **Helper Functions**: Such as `transfer` that facilitates token transfer over IBC (Inter-Blockchain Communication).
5. **Unit Tests**: A series of tests to ensure the contract's functionality, including token transfers and IBC functionality.

[Jump to Table of Contents](#table-of-contents)

## Getting Started

To start working with the Makefile and the contract:

1. Set up the Coreum node and ensure it's running.
2. Clone your repository containing the Makefile and `contract.rs`.
3. Navigate to the directory.
4. Use the commands provided in the Makefile, such as `make dev`, `make test`, `make deploy`, etc.
5. Modify the `contract.rs` to suit your contract requirements, and test it using the provided unit tests.

[Jump to Table of Contents](#table-of-contents)

## Contribution & Issues

Feel free to contribute to the development of this project by submitting pull requests. If you find any issues or have suggestions, kindly raise an issue in the repository's issue section.

[Jump to Table of Contents](#table-of-contents)

## Main Module

The main module handles the core logic for IBC channel operations including opening, connecting, closing, packet reception, packet acknowledgment, packet timeouts, and validation functions.

### Features

- Conditional compilation using `library` feature.
- Defines IBC version as `counter-1`.
- Entry points for:
  - IBC channel opening handshake.
  - IBC channel connection.
  - IBC channel closure.
  - Receiving IBC packets.
  - Handling IBC packet acknowledgements.
  - Handling IBC packet timeouts.

### Dependencies

- **Standard Modules**: The code relies heavily on the `cosmwasm_std` crate, which provides essential data types and functions for building CosmWasm smart contracts.
- **Local Modules**: Uses local modules such as `ack`, `contract`, `error`, `msg`, and `state` for various functionalities.

[Jump to Table of Contents](#table-of-contents)

## State Module (state.rs)

This module maintains the state of the connections and timeouts using `cw_storage_plus` Map data structures.

### Storage

- `CONNECTION_COUNTS`: Keeps track of the connection counts indexed by channel ID.
- `TIMEOUT_COUNTS`: Maintains the count of timeouts for each channel.

[Jump to Table of Contents](#table-of-contents)

## Acknowledgment Module (ack.rs)

Handles the creation of IBC acknowledgment messages.

### Features

- Defines the `Ack` enum for IBC acknowledgments.
- Provides utility functions to create success and failure acknowledgments.

[Jump to Table of Contents](#table-of-contents)

## Message Module (msg.rs)

Manages the message structures used in the smart contract.

### Message Types

- **InstantiateMsg**: Message for instantiation.
- **ExecuteMsg**: Contains variants for different execution commands like `Transfer` and `Increment`.
- **IbcExecuteMsg**: IBC-specific execute message.
- **QueryMsg**: Defines queries like `GetCount` and `GetTimeoutCount`.

### Response Structs

- `GetCountResponse`: Returns the count of a specified channel.

[Jump to Table of Contents](#table-of-contents)
