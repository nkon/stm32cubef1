#![no_std]
#![feature(core_intrinsics)] // core_intrinsics を使う。子ファイルではなく、lib.rs に必要。

pub mod gpio;
pub mod tim;
pub mod pwr;
pub mod hal;
pub mod uart;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
