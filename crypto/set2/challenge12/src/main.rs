use base64;
use std::default::{Default};

#[derive(Default)]
struct EcbOracle {
	unknown: Vec<u8>,
	key: Vec<u8>,
	test: String,
}

impl EcbOracle {
	fn unknown_mut(&mut self) -> &mut Vec<u8> {
		&mut self.unknown
	}

	fn test_mut(&mut self) -> &mut String {
		&mut self.test
	}
}

fn main() {
    let unknown = base64::decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();

    let mut oracle = EcbOracle::default();
    let thingy = vec![b'a'];
    *oracle.unknown_mut() = unknown;
    println!("{:?}", oracle.unknown);

}
