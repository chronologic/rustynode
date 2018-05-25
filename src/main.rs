extern crate rustynode;
extern crate web3;

use rustynode::Timenode;
use web3::types::Address;

fn main() {
    let tn: Timenode = Timenode::boot();

    let my_address: Address = "0x1cb960969f58a792551c4e8791d643b13025256d"
        .parse()
        .unwrap();
    tn.works();
}
