extern crate borsh_derive;
extern crate borsh;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A {
    x: u64,
    y: String,
}

fn main() {
    // 0. Read the input file and generate the code to deserialise
    // 1. Deserialise the file and create a text version of the object
    // 2. Call $EDITOR with this text version, let the user edit
    // 3. On save, serialize the edited object and write to file
}

#[test]
fn test_simple_struct() {
    let a = A {
        x: 3301,
        y: "liber primus".to_string(),
    };
    let encoded_a = a.try_to_vec().unwrap();
    println!("{:?}", encoded_a);
    let decoded_a = A::try_from_slice(&encoded_a).unwrap();
    assert_eq!(a, decoded_a);
}
