#![crate_name = "mithril"]
#![crate_type = "lib"]

#![feature(llvm_asm)]
#![feature(repr_simd)]
#![feature(box_syntax)]
#![feature(integer_atomics)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate strum;

pub mod byte_string;
pub mod cryptonight;
pub mod stratum;
pub mod worker;
pub mod u64x2;
pub mod metric;
pub mod bandit_tools;
pub mod mithril_config;
pub mod timer;
pub mod randomx;
