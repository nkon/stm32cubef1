#![allow(non_snake_case)]

//! Interface of stm32f1xx_hal.c

extern "C" {
    pub fn HAL_GetTick() -> u32;
    pub fn HAL_Delay(delay: u32) -> ();
}

pub fn GetTick() -> u32 {
    unsafe { HAL_GetTick() }
}

pub fn Delay(delay: u32) -> () {
    unsafe {
        HAL_Delay(delay);
    }
}
