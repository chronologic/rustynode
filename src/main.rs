extern crate ethabi;
extern crate rustynode;
extern crate web3;

use rustynode::{Event_Emitter, Timenode};
use web3::types::H160;

use web3::Web3;
use web3::transports::WebSocket;
use web3::futures::{Future, Stream};


use std::{thread, time};

// use rustynode::ScheduledTransaction;

fn main() {

    let my_address: H160 = "1cb960969f58a792551c4e8791d643b13025256d".parse().unwrap();

    let (_eloop, transport) = WebSocket::new("ws://localhost:8546").unwrap();
    let web3 = Web3::new(transport);

    let tn: Timenode<WebSocket> = Timenode::boot(web3.clone());

    let e_e_address: H160 = "bf21760528357ea7ef3f1eaf5513fe5d495f19c4".parse().unwrap();

    let scheduler_address: H160 = "fe3afc02954c4d989f572f61ee185d654c0c7134".parse().unwrap();

    tn.subscribe_to(e_e_address, scheduler_address);
}

// use ethabi::token::{Token, Tokenizer, LenientTokenizer};

// fn main() {
//     let addr = &"30237f8415efd4c7e3b6f8ae2bde75f3c4d86e84";
//     let tokens = LenientTokenizer::tokenize(
//         &ethabi::ParamType::Address,
//         &addr,
//     );
//     let encoded = ethabi::encode(&[tokens.unwrap()]);
//     println!("{:?}", encoded);
// }
