use super::utils;

pub struct ScheduledTransaction {
    address: &str,
    params: ScheduledTransactionParams,
}

pub struct ScheduledTransactionParams {
    temporal_unit: &str,
    recipient: &str,
    value: &str,
    call_gas: &str,
    gas_price: &str,
    execution_window_start: &str,
    execution_window_length: &str,
    bounty: &str,
    feeL &str,
    conditional_call_destination: &str,
    call_data: &str,
    conditional_call_data: &str,
}

// impl ScheduledTransaction {
//     pub fn from_raw(raw_data: &str) -> ScheduledTransaction {
//         let parsed: Vec<&str> = utils::split_n_chars(raw_data, 64);
//         ScheduledTransaction {
//             address: parsed[0],
//             ScheduledTransactionParams {
//                 temporal_unit: parsed[4],
//                 recipient: parsed[5],
                
//             }
//         }
//     }
// }
