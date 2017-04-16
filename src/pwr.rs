#![allow(non_snake_case)]

//! Interface of stm32f1xx_hal.c

pub const SLEEPENTRY_WFI: u8 = 0x01;
pub const SLEEPENTRY_WFE: u8 = 0x02;


extern "C" {
    pub fn HAL_PWR_EnterSLEEPMode(Regulator: u32, SLEEPEntry: u8) -> ();
}

pub fn EnterSLEEPMode(mode: u8) -> () {
    unsafe {
        HAL_PWR_EnterSLEEPMode(0, mode);
    }
}
