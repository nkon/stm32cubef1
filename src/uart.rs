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

#[repr(C)]
struct Init {
    BaudRate: u32,
    WordLength: u32,
    StopBits: u32,
    Parity: u32,
    Mode: u32,
    HwFlowCtl: u32,
    OverSampling: u32,
}

const HAL_UART_STATE_RESET: u32 = 0x00; /* Peripheral is not initialized */
const HAL_UART_STATE_READY: u32 = 0x01; /* Peripheral Initialized and ready for use */
const HAL_UART_STATE_BUSY: u32 = 0x02; /* an internal process is ongoing */
const HAL_UART_STATE_BUSY_TX: u32 = 0x12; /* Data Transmission process is ongoing */
const HAL_UART_STATE_BUSY_RX: u32 = 0x22; /* Data Reception process is ongoing */
const HAL_UART_STATE_BUSY_TX_RX: u32 = 0x32; /* Data Transmission and Reception process is ongoing */
const HAL_UART_STATE_TIMEOUT: u32 = 0x03; /* Timeout state */
const HAL_UART_STATE_ERROR: u32 = 0x04; /* Error */

use lock::Lock;
use queue::Queue;
const QUEUE_LENGTH: usize = 32;

#[repr(C)]
struct NewHandle<'a> {
    Instance: &'static mut Regs,
    Init: Init,
    pTxBuffPtr: &'a mut Queue<'a, u8>,
    TxXferSize: u16,
    TxXfreCount:u16,
    pRxBuffPtr: &'a mut Queue<'a, u8>,
    RxXferSize: u16,
    RxXferCount: u16,
    Lock: Lock,
    State: u32,
    ErrorCode: u32,
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

// static mut TX_MEMORY: [u8;QUEUE_LENGTH] = [0;QUEUE_LENGTH];
// static mut RX_MEMORY: [u8;QUEUE_LENGTH] = [0;QUEUE_LENGTH];

// static mut TX_QUEUE: Queue<u8> = Queue::new(&'static mut TX_MEMORY);
// static mut RX_QUEUE: Queue<u8> = Queue::new(&'static mut RX_MEMORY);

impl<'a> NewHandle<'a> {
    pub fn new(regs: &'static mut Regs, init: Init, txq: &'a mut Queue<'a, u8>, rxq: &'a mut Queue<'a, u8>) -> Self {
        NewHandle {
            Instance: regs,
            Init: init,
            pTxBuffPtr: txq,
            TxXferSize: QUEUE_LENGTH as u16,
            TxXfreCount:0,
            pRxBuffPtr: rxq,
            RxXferSize: QUEUE_LENGTH as u16,
            RxXferCount: 0,
            Lock: Lock::Unlocked,
            State: HAL_UART_STATE_RESET,
            ErrorCode: 0,
        }
    }
    pub fn Transmit(&mut self, pTxData: *const u8, size: usize) -> Option<usize> {
        None
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
