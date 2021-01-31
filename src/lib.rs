pub mod circuit;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode(bits: &[u8]) -> usize {
    let mut result: usize = 0;
    for bit in bits {
        result = (result << 1) + if *bit > 0 { 1 } else { 0 }
    }

    result
}

#[wasm_bindgen]
pub fn decode(mut value: usize, bits: &mut [u8]) {
    let mut bit_idx: usize = bits.len() - 1;
    while value > 0 {
        bits[bit_idx] = (value & 1) as u8;
        if bit_idx == 0 {
            break;
        }

        bit_idx -= 1;
        value >>= 1;
    }
}
