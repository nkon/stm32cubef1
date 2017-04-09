#![no_std]
#![feature(core_intrinsics)] // core_intrinsics を使う。子ファイルではなく、lib.rs に必要。

pub mod stm32f1xx_hal_gpio;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
