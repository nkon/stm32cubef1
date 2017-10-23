#![no_std]
#![feature(core_intrinsics)] // core_intrinsics を使う。子ファイルではなく、lib.rs に必要。

extern crate nostd_tool;

pub mod gpio;
pub mod tim;
pub mod pwr;
pub mod hal;
pub mod uart;
//pub mod lock;
//pub mod queue;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
