use web3::Web3;
use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::{Address, U256};

struct Event_Emitter {
    instance: Contract<Http>,
    web3: Web3<Http>,
}

impl Event_Emitter {
    pub fn at(address: String, web3: Web3<Http>) -> Event_Emitter {
        let contract_address: Address = address.parse().unwrap();
        let contract = Contract::from_json(web3.eth(), contract_address, include_bytes!("../build/abis/EventEmitter.abi")).unwrap();

        Event_Emitter {
            instance: contract,
            web3: web3,
        }
    }
}
