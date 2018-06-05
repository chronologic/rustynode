use super::utils;

use ethabi::token::{Token, Tokenizer, LenientTokenizer};
use web3::types::{Address, U256, Bytes};

// #[derive(Debug)]
// enum Error {}

#[derive(Debug)]
pub struct ScheduledTransaction {
    address: ethabi::Token,
    params: ScheduledTransactionParams,
}

#[derive(Debug)]
pub struct ScheduledTransactionParams {
    temporal_unit: ethabi::Token,
    recipient: ethabi::Token,
    value: ethabi::Token,
    call_gas: ethabi::Token,
    gas_price: ethabi::Token,
    execution_window_start: ethabi::Token,
    execution_window_length: ethabi::Token,
    bounty: ethabi::Token,
    fee: ethabi::Token,
    conditional_call_destination: ethabi::Token,
    call_data: ethabi::Token,
    conditional_call_data: ethabi::Token,
}

impl ScheduledTransaction {
    pub fn from_raw(raw_data: &[u8]) -> ScheduledTransaction {

        let addr = ethabi::decode(
            &[ethabi::ParamType::Address],
            &raw_data[..32],
        ).unwrap()[0].clone();

        let decoded = ScheduledTransaction::decode_raw_data(&raw_data[96..]).unwrap();

        ScheduledTransaction {
            address: addr,
            params: ScheduledTransactionParams {
                temporal_unit: decoded[0].clone(),
                recipient: decoded[1].clone(),
                value: decoded[2].clone(),
                call_gas: decoded[3].clone(),
                gas_price: decoded[4].clone(),
                execution_window_start: decoded[5].clone(),
                execution_window_length: decoded[6].clone(),
                bounty: decoded[7].clone(),
                fee: decoded[8].clone(),
                conditional_call_destination: decoded[9].clone(),
                call_data: decoded[10].clone(),
                conditional_call_data: decoded[11].clone(),
            }
        }
    }

    fn decode_raw_data(b: &[u8]) -> Result<Vec<ethabi::Token>, ethabi::Error> {

    let param_types: Vec<ethabi::ParamType> = vec![
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Address,
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Address,
        ethabi::ParamType::Bytes,
        ethabi::ParamType::Bytes,
    ];

    let decoded = ethabi::decode(
        &param_types,
        b,
    );

    decoded

    }
}
