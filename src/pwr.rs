#![allow(non_snake_case)]

//! Interface of stm32f1xx_hal.c

pub const SLEEPENTRY_WFI: u8 = 0x01;
pub const SLEEPENTRY_WFE: u8 = 0x02;


extern "C" {
    pub fn HAL_PWR_EnterSLEEPMode(Regulator: u32, SLEEPEntry: u8) -> ();
}

/// Enter sleep mode.
/// mode==SLEEPENTRY_WFI => enter WFI(Wait for Interrupt) mode.
/// mode==SLEEPENTRY_WFE => enter WFE(Wait for Event) mode.
pub fn EnterSLEEPMode(mode: u8) -> () {
    unsafe {
        HAL_PWR_EnterSLEEPMode(0, mode);
    }
}
