#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
extern crate qimalloc;

pub mod constraint;
pub mod contract;
pub mod error;
pub mod little_endian_encoding;
pub mod merkle_binary_tree;
pub mod stack;
pub mod state;
pub mod transaction;
pub mod r#type;
pub mod value;
pub mod vm;
pub mod utxo;

// This is a list of functions that `ewasm` environments support. They provide additional data and
// functionality to execution environments. Each function is implemented in the host environment.
#[cfg(feature = "scout")]
mod native {
    extern "C" {
        pub fn eth2_loadPreStateRoot(offset: *const u32);
        pub fn eth2_blockDataSize() -> u32;
        pub fn eth2_blockDataCopy(outputOfset: *const u32, offset: u32, length: u32);
        pub fn eth2_savePostStateRoot(offset: *const u32);
    }
}

#[cfg(feature = "scout")]
#[no_mangle]
pub extern "C" fn main() {
    let input_size = unsafe { native::eth2_blockDataSize() as usize };

    // Copy input into buffer (buffer fixed at 42kb for now)
    let mut input = [0u8; 42000];
    unsafe {
        native::eth2_blockDataCopy(input.as_mut_ptr() as *const u32, 0, input_size as u32);
    }

    // Get pre-state-root
    let mut pre_state_root = [0u8; 32];
    unsafe { native::eth2_loadPreStateRoot(pre_state_root.as_mut_ptr() as *const u32) }

    // Process input data
    let post_root = process_data_blob(&mut input, &pre_state_root);

    // Return post state
    unsafe { native::eth2_savePostStateRoot(post_root.as_ptr() as *const u32) }
}