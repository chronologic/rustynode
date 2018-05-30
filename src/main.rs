extern crate rustynode;
extern crate web3;

use rustynode::{Event_Emitter, Timenode};
use web3::types::H160;

use web3::Web3;
use web3::transports::WebSocket;
use web3::futures::{Future, Stream};


use std::{thread,time};

fn main() {

    let my_address: H160 = "1cb960969f58a792551c4e8791d643b13025256d"
        .parse().unwrap();

    let (_eloop, transport) = WebSocket::new("ws://localhost:8546").unwrap();
    let web3 = Web3::new(transport);

    // let tn: Timenode<WebSocket> = Timenode::boot(web3.clone());
    // tn.works();

    let e_e_address: H160 = "bf21760528357ea7ef3f1eaf5513fe5d495f19c4"
        .parse().unwrap();
    
    let e_e: Event_Emitter<WebSocket> = Event_Emitter::at(
        e_e_address,
        web3.clone(),
    );

    // e_e.get_newTransactionScheduled(7511598);

    e_e.watch_newTransactionScheduled("fe3afc02954c4d989f572f61ee185d654c0c7134".parse().unwrap());

    // thread::sleep(time::Duration::from_secs(10_000))

}
