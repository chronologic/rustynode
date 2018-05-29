use std::collections::HashMap;

use web3::Web3;
// use web3::futures::Future;
// use web3::contract::{Contract, Options};
use web3::transports::Http;
// use web3::types::{Address, U256};

#[derive(Debug)]
pub struct Timenode {
    cache: Cache,
    pub web3: Web3<Http>,
    //
}

#[derive(Debug, Default)]
pub struct Cache {
    // store - The addresses of the scheduled transactions that the timenode is watching
    store: HashMap<String, String>,
    // polling - The address of the condtional transactions the timenode is polling
    poll_store: HashMap<String, String>,
}

impl Timenode {
    pub fn boot(web3: Web3<Http>) -> Timenode {
        Timenode {
            cache: Cache::default(),
            web3: web3,
        }
    }
    pub fn subscribe_to(scheduler_contract: String, event_emitter: String) {
        unimplemented!();
    }
    pub fn works(&self) {
        println!("Works!\n{:?}", &self);
    }
}
