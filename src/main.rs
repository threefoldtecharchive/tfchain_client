mod lib;
use keyring::AccountKeyring;

pub fn main() {
    let from = AccountKeyring::Alice.pair();

    let client = lib::Client::new(String::from("ws://localhost:9944"), from);
    
    match client.create_twin(String::from("1.1.1.1")) {
        Ok(hash) => {
            println!("tx included in hash: {:?}", hash);
        },
        Err(err) => {
            println!("err: {:?}", err)
        }
    }
}