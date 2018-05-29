use web3::Web3;
use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::{Address, U256};

#[derive(Debug)]
pub struct Event_Emitter {
    pub instance: Contract<Http>,
    pub web3: Web3<Http>,
}

impl Event_Emitter {
    pub fn at(address: Address, web3: Web3<Http>) -> Event_Emitter {    
        let contract = Contract::from_json(web3.eth(), address, include_bytes!("../build/abis/EventEmitter.abi")).unwrap();

        Event_Emitter {
            instance: contract,
            web3: web3,
        }
    }

    pub fn works(&self) {
        println!("Works!\n{:?}", &self);
    }
}
