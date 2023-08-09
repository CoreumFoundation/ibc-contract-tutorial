// Conditionally compile the entry_point if not in library mode
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

// Import necessary standard modules from cosmwasm_std
use cosmwasm_std::{
    from_binary, DepsMut, Env, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcOrder, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, StdResult,
};

// Import local modules for IBC handling and related functionalities
use crate::{
    ack::{make_ack_fail, make_ack_success},
    contract::try_increment,
    error::Never,
    msg::IbcExecuteMsg,
    state::{CONNECTION_COUNTS, TIMEOUT_COUNTS},
    ContractError,
};

// Define the version for IBC
pub const IBC_VERSION: &str = "counter-1";

// Entry point to handle IBC channel opening handshake phases: `OpenInit` and `OpenTry`
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse, ContractError> {
    validate_order_and_version(msg.channel(), msg.counterparty_version())?;
    Ok(None)
}

// Entry point to handle IBC channel connection phase
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    validate_order_and_version(msg.channel(), msg.counterparty_version())?;

    // Initialize counter for this channel to zero
    let channel = msg.channel().endpoint.channel_id.clone();
    CONNECTION_COUNTS.save(deps.storage, channel.clone(), &0)?;

    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_connect")
        .add_attribute("channel_id", channel))
}

// Entry point to handle IBC channel closure
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let channel = msg.channel().endpoint.channel_id.clone();
    // Clear the channel state
    CONNECTION_COUNTS.remove(deps.storage, channel.clone());
    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_close")
        .add_attribute("channel", channel))
}

// Entry point for receiving IBC packets
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    // Handle the packet and ensure we always ACK regardless of success or failure
    match do_ibc_packet_receive(deps, env, msg) {
        Ok(response) => Ok(response),
        Err(error) => Ok(IbcReceiveResponse::new()
            .add_attribute("method", "ibc_packet_receive")
            .add_attribute("error", error.to_string())
            .set_ack(make_ack_fail(error.to_string()))),
    }
}

// Inner logic for handling packet reception
pub fn do_ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    let channel = msg.packet.dest.channel_id;
    let msg: IbcExecuteMsg = from_binary(&msg.packet.data)?;

    match msg {
        IbcExecuteMsg::Increment {} => execute_increment(deps, channel),
    }
}

// Execute increment action upon IBC packet receive
fn execute_increment(deps: DepsMut, channel: String) -> Result<IbcReceiveResponse, ContractError> {
    let count = try_increment(deps, channel)?;
    Ok(IbcReceiveResponse::new()
        .add_attribute("method", "execute_increment")
        .add_attribute("count", count.to_string())
        .set_ack(make_ack_success()))
}

// Entry point for handling IBC packet acknowledgements
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    _ack: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // No state tracking for acknowledgements, but can expand if needed
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_ack"))
}

// Entry point for handling IBC packet timeouts
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    TIMEOUT_COUNTS.update(
        deps.storage,
        msg.packet.src.channel_id,
        |count| -> StdResult<_> { Ok(count.unwrap_or_default() + 1) },
    )?;
    // No additional handling for timeouts, but can expand if needed
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_timeout"))
}

// Validate the order and version for the IBC channel
pub fn validate_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
) -> Result<(), ContractError> {
    // Ensure we're working with an unordered channel
    if channel.order != IbcOrder::Unordered {
        return Err(ContractError::OrderedChannel {});
    }

    if channel.version != IBC_VERSION {
        return Err(ContractError::InvalidVersion {
            actual: channel.version.to_string(),
            expected: IBC_VERSION.to_string(),
        });
    }

    // Ensure compatibility with counterparty version
    if let Some(counterparty_version) = counterparty_version {
        if counterparty_version != IBC_VERSION {
            return Err(ContractError::InvalidVersion {
                actual: counterparty_version.to_string(),
                expected: IBC_VERSION.to_string(),
            });
        }
    }

    Ok(())
}
