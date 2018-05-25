use web3::Web3;
use web3::transports::Http;

#[derive(Debug)]
pub struct Timenode {
    cache: Cache,
    web3: Web3<Http>,
    //
}

#[derive(Debug, Default)]
pub struct Cache {
    // store - The addresses of the scheduled transactions that the timenode is watching
    // polling - The address of the condtional transactions the timenode is polling
}

impl Timenode {
    pub fn boot() -> Timenode {
        let (_eloop, transport) = Http::new("http://localhost:8545").unwrap();
        let web3 = Web3::new(transport);

        Timenode {
            cache: Cache::default(),
            web3: web3,
        }
    }
    pub fn subscribe_to(scheduler_contract: String, event_emitter: String) {
        unimplemented!();
    }
    pub fn works(&self) {
        println!("Works!\n{:?}", &self);
    }
}
