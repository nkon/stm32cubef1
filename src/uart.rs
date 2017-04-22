#![allow(non_snake_case)]

//! Interface of stm32f1xx_hal_usart.c
//! # Examples

// レジスタアドレスの定義
const PERIPH_BASE: u32 = 0x40000000;

const APB1PERIPH_BASE: u32 = PERIPH_BASE;
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x10000;
const USART2_BASE: u32 = APB1PERIPH_BASE + 0x4400;
const USART3_BASE: u32 = APB1PERIPH_BASE + 0x4800;
const USART1_BASE: u32 = APB2PERIPH_BASE + 0x3800;

pub enum Status {
    HalOk,
    HalBusy,
}

#[repr(C)]
pub struct Regs {
    SR: u32, /* USART Status register */
    DR: u32, /* USART Data register */
    BRR: u32, /* USART Baud rate register */
    CR1: u32, /* USART Control register 1 */
    CR2: u32, /* USART Control register 2 */
    CR3: u32, /* USART Control register 3 */
    BTPR: u32, /* USART Guard time and prescaler register */
}

#[repr(C)]
pub struct Handle {
    dummy: u32,
}

extern "C" {
    pub fn HAL_UART_Transmit_IT(husart: &mut Handle, pTxData: *const u8, Size: u16) -> u32;
    pub fn HAL_UART_Receive_IT(husart: &mut Handle, pRxData: *const u8, Size: u16) -> u32;
    pub fn HAL_UART_TransmitReceive_IT(husart: &mut Handle,
                                       pTxData: *const u8,
                                       pRxData: *const u8,
                                       Size: u16)
                                       -> u32;
}

impl Handle {
    pub fn Transmit_IT(&mut self, pTxData: &str) -> Status {
        let ret: u32;
        unsafe {
            ret = HAL_UART_Transmit_IT(self, pTxData.as_ptr(), pTxData.len() as u16);
        }
        match ret {
            0 => Status::HalOk,
            _ => Status::HalBusy,
        }
    }

    pub fn Receive_IT(&mut self, pTxData: &str) -> Status {
        let ret: u32;
        unsafe {
            ret = HAL_UART_Receive_IT(self, pTxData.as_ptr(), pTxData.len() as u16);
        }
        match ret {
            0 => Status::HalOk,
            _ => Status::HalBusy,
        }
    }
}

pub fn USART2() -> &'static mut Regs {
    unsafe { &mut *(USART2_BASE as *mut Regs) }
}
pub fn USART3() -> &'static mut Regs {
    unsafe { &mut *(USART3_BASE as *mut Regs) }
}
pub fn USART1() -> &'static mut Regs {
    unsafe { &mut *(USART1_BASE as *mut Regs) }
}
