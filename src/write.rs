use std::fs;
use crate::structure::Account;
use borsh::{BorshSerialize};

pub fn write_to_file(acc: Account, path: &str) {
  fs::write(
    path, 
    acc.try_to_vec().unwrap()
  ).unwrap();
}