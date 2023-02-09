// re-export module to properly feature gate sgx and regular std environment
#![cfg_attr(not(target_env = "sgx"), no_std)]

#[cfg(feature = "sgx")]
extern crate sgx_tstd as std;

pub use std::cmp::Ordering;
pub use std::fmt;
#[cfg(target_pointer_width = "32")]
#[path = "arch/arch32.rs"]
pub mod arch;
#[cfg(target_pointer_width = "64")]
#[path = "arch/arch64.rs"]
pub mod arch;
pub mod errors;
pub mod hash256;
pub mod hash384;
pub mod hash512;
pub mod rand;
pub mod sha3;
pub mod types;

// #[cfg(feature = "rsa3072")]
#[path = "./"]
pub mod rsa3072 {
    pub mod big;
    pub mod dbig;
    pub mod ff;
    #[cfg(target_pointer_width = "64")]
    #[path = "roms/rom_rsa3072_64.rs"]
    pub mod rom;
    pub mod rsa;
}
