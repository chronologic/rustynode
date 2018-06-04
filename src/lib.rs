#![feature(extern_prelude)]

extern crate ethabi;
extern crate rustc_hex;
extern crate web3;

pub mod event_emitter;
// pub mod scheduled_transaction;
pub mod timenode;

pub use event_emitter::Event_Emitter;
// pub use scheduled_transaction::ScheduledTransaction;
pub use timenode::Timenode;

mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
