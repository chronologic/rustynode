use std::collections::HashMap;

use super::Event_Emitter;
use super::utils;

use web3::Web3;
use web3::futures::{Future, Stream};
// use web3::contract::{Contract, Options};
use web3::transports::WebSocket;
use web3::Transport;
use web3::types::{Address, U256};

use rustc_hex::ToHex;

#[derive(Debug)]
pub struct Timenode<T>
where
    T: Transport,
{
    cache: Cache,
    pub web3: Web3<T>,
    //
}

#[derive(Debug, Default)]
pub struct Cache {
    // store - The addresses of the scheduled transactions that the timenode is watching
    store: HashMap<String, String>,
    // polling - The address of the condtional transactions the timenode is polling
    poll_store: HashMap<String, String>,
}

impl Timenode<WebSocket> {
    pub fn boot(web3: Web3<WebSocket>) -> Timenode<WebSocket> {
        Timenode {
            cache: Cache::default(),
            web3: web3,
        }
    }
    pub fn subscribe_to(&self, event_emitter: Address, scheduler_contract: Address) {
        // Create an instance of the Event Emitter (TODO check if the Timenode already
        // has it stored.)
        let e: Event_Emitter<WebSocket> = Event_Emitter::at(event_emitter, self.web3.clone());

        e.watch_newTransactionScheduled(scheduler_contract)
            .then(|sub| {
                sub.unwrap().for_each(|log| {
                    // println!(
                        
                        utils::decode_raw_data(
                            &log.data.0[96..]
                        );
                    // );
                        // "got log\n {:?}", 
                        // utils::split_n_chars(
                            // &log.data.0[32..].to_hex(),
                            // 64,
                        // )
                    // );
                    Ok(())
                })
            })
            .wait();
    }

    pub fn works(&self) {
        println!("Works!\n{:?}", &self);
    }
}
