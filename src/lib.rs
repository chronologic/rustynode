// extern crate ethabi;
extern crate web3;

pub mod event_emitter;
pub mod timenode;

pub use event_emitter::Event_Emitter;
pub use timenode::Timenode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
