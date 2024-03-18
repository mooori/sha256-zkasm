//#![no_main]
//#![no_std]

//use panic_halt as _;

use sha2::{Digest, Sha256};

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn assert_eq(actual: u64, expected: u64);
}
#[cfg(not(target_arch = "wasm32"))]
fn assert_eq(actual: u64, expected: u64) {
    assert_eq!(actual, expected);
}

fn subslice_to_u64(slice: &[u8], begin: usize, end: usize) -> u64 {
    let mut result = 0u64;
    for i in begin..end {
        result <<= 8;
        result += slice[i] as u64;
    }
    result
}

//#[no_mangle]
pub fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let result = hasher.finalize();

    unsafe {
        assert_eq(subslice_to_u64(&result, 0, 8), 13352372148217134600);
        assert_eq(subslice_to_u64(&result, 8, 16), 11902541952223915002);
        assert_eq(subslice_to_u64(&result, 16, 24), 14160706888648589550);
        assert_eq(subslice_to_u64(&result, 24, 32), 10414846460208074217);
    }
}
