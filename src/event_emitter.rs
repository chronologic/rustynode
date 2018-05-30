use std::time;

use web3::Web3;
use web3::futures::{Future, Stream};
use web3::contract::{Contract, Options};
use web3::transports::{Http, WebSocket};
use web3::types::{Address, FilterBuilder, U256};
use web3::api::EthFilter;
use web3::Transport;


#[derive(Debug)]
pub struct Event_Emitter<T>
    where T: Transport
{
    pub instance: Contract<T>,
    pub web3: Web3<T>,
}

impl Event_Emitter<WebSocket>
{
    pub fn at(address: Address, web3: Web3<WebSocket>) -> Event_Emitter<WebSocket> {    
        let contract = Contract::from_json(web3.eth(), address, include_bytes!("../build/abis/EventEmitter.abi")).unwrap();

        Event_Emitter {
            instance: contract,
            web3: web3,
        }
    }

    pub fn watch_newTransactionScheduled(&self, scheduled_from: Address) {
        let filter = FilterBuilder::default()
            .address(vec![self.instance.address()])
            .topics(
                Some(vec![
                    "0x94c6f2d01cc82df9dceeabfd7786c57a01cd9796e7cab146d2d0cf5c8380310d".into(),
                ]),
                None,
                Some(vec![
                    scheduled_from.into(),
                ]),
                None,
            )
            .build();

        let event_future = self.web3.eth_subscribe()
            .subscribe_logs(filter)
            .then(|sub| {
                sub
                    .unwrap()
                    .for_each(|log| {
                        println!("got log {:?}", log);
                        Ok(())
                    })
            })
            .map_err(|_| ());
    }

    pub fn works(&self) {
        println!("Works!\n{:?}", self.instance.address());
    }
}
