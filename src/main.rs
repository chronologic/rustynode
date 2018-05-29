extern crate rustynode;
extern crate web3;

use rustynode::{Event_Emitter, Timenode};
use web3::types::H160;

use web3::Web3;
use web3::transports::Http;

fn main() {

    let my_address: H160 = "1cb960969f58a792551c4e8791d643b13025256d"
        .parse().unwrap();

    let (_eloop, transport) = Http::new("http://localhost:8545").unwrap();
    let web3 = Web3::new(transport);

    let tn: Timenode = Timenode::boot(web3.clone());
    tn.works();

    let e_e_address: H160 = "bf21760528357ea7ef3f1eaf5513fe5d495f19c4"
        .parse().unwrap();
    
    let e_e: Event_Emitter = Event_Emitter::at(
        e_e_address,
        web3.clone(),
    );

    e_e.works();
}
