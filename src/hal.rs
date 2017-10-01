#![allow(non_snake_case)]

//! Interface of stm32f1xx_hal.c

extern "C" {
    pub fn HAL_GetTick() -> u32;
    pub fn HAL_Delay(delay: u32) -> ();
}


/// Wrapper of `HAL_GetTick()`.
/// Return the value of the SysTick counter.
pub fn GetTick() -> u32 {
    unsafe { HAL_GetTick() }
}


/// Wrapper of `HAL_Delay()`.
/// delay `delay` counts of SysTick.
pub fn Delay(delay: u32) -> () {
    unsafe {
        HAL_Delay(delay);
    }
}
