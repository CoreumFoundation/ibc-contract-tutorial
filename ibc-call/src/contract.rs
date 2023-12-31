// Conditional compilation: If "library" feature isn't active, import 'entry_point'.
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

// Importing cosmwasm standard library components.
use cosmwasm_std::{
    to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo,
    Response, StdResult,
};

// Importing contract versioning utility from cw2 crate.
use cw2::set_contract_version;

// Local module imports for messages and state.
use crate::msg::{ExecuteMsg, InstantiateMsg, IbcExecuteMsg, QueryMsg, GetCountResponse};
use crate::state::{CONNECTION_COUNTS, TIMEOUT_COUNTS};  // Note: Removed TIMEOUT_COUNTS as it's not in use.

// Constants for contract name and version, fetched from Cargo.toml.
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Entry point for the contract instantiation.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Set the contract version on instantiation.
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Entry point for contract execution.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    // Handle different execution messages.
    match msg {
        ExecuteMsg::Increment { channel, timeout } => Ok(Response::new()
            .add_attribute("method", "execute_increment")
            .add_attribute("channel", channel.clone())
            // Send an IBC message after the packet is received on the other chain.
            .add_message(IbcMsg::SendPacket {
                channel_id: channel,
                data: to_binary(&IbcExecuteMsg::Increment {})?,
                // Set a default timeout of two minutes.
                // timeout: IbcTimeout::with_timestamp(_env.block.time.plus_seconds(120)),
                timeout: timeout,
            })),
    }
}

// Function to be called when an IBC packet is received on the other chain.
pub fn try_increment(deps: DepsMut, channel: String) -> StdResult<u32> {
    CONNECTION_COUNTS.update(deps.storage, channel, |count| -> StdResult<_> {
        Ok(count.unwrap_or_default() + 1)
    })
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount { channel } => to_binary(&query_count(deps, channel)?),
        QueryMsg::GetTimeoutCount { channel } => to_binary(&query_timeout_count(deps, channel)?),
    }
}

fn query_count(deps: Deps, channel: String) -> StdResult<GetCountResponse> {
    let count = CONNECTION_COUNTS
        .may_load(deps.storage, channel)?
        .unwrap_or_default();
    Ok(GetCountResponse { count })
}

fn query_timeout_count(deps: Deps, channel: String) -> StdResult<GetCountResponse> {
    let count = TIMEOUT_COUNTS
        .may_load(deps.storage, channel)?
        .unwrap_or_default();
    Ok(GetCountResponse { count })
}

// Unit tests for the contract.
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, WasmMsg};

    // Test to ensure contract initializes correctly.
    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, InstantiateMsg {}).unwrap();
        assert_eq!(0, res.messages.len());
    }

    
    // Test the increment functionality over IBC.
    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let channel: String = "channel-0".into();  // Type annotation for clarity.

        let msg = ExecuteMsg::Increment { 
            channel: channel.clone(),
            timeout: IbcTimeout::with_timestamp(mock_env().block.time.plus_seconds(120)),
        };

        let info = mock_info("sender", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Verify the increment message attributes.
        assert_eq!(res.attributes[0].key, "method");
        assert_eq!(res.attributes[0].value, "execute_increment");

        assert_eq!(res.attributes[1].key, "channel");
        assert_eq!(res.attributes[1].value, channel);
    }
}
