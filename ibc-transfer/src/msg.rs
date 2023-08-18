use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, IbcTimeout};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer {
        channel_id: String,
        to_address: String,
        amount: Coin,
        timeout: IbcTimeout,
    },
}

