extern crate "rustc-serialize" as serialize;

use serialize::base64::{STANDARD, ToBase64};

let mut bytes = Vec::from_elem(sha.output_bytes(), 0u8);
sha.result(bytes.as_mut_slice());
println!("{}", bytes.to_base64(STANDARD));
