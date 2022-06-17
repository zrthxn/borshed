use borsh::{ BorshSerialize, BorshDeserialize };

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct Account {
  address: String,
  balance: f64,

  last_txn: String,
  pubkey: String
}

#[allow(dead_code)]
impl Account {
  pub fn new(  
    address: &String,
    balance: &f64,
    last_txn: &String,
    pubkey: &String) -> Account {
      Account {
        address: address.clone(),
        balance: balance.clone(),
        last_txn: last_txn.clone(),
        pubkey: pubkey.clone()
      }
  }
}
