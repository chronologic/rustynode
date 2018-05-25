extern crate rustynode;

use rustynode::Timenode;

fn main() {
    let tn: Timenode = Timenode::boot();
    tn.works();
}