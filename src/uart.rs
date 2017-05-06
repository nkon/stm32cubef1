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
    Instance: &'static mut Regs,
    pub Init: Init,
    pTxBuffPtr: *mut u8,
    TxXferSize: u16,
    TxXferCount:u16,
    pRxBuffPtr: *mut u8,
    RxXferSize: u16,
    RxXferCount: u16,
    hdmatx: u32,
    hdmarx: u32,
    Lock: u8,
    State: u8,
    errorCode: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct Init {
    BaudRate: u32,
    WordLength: u32,
    StopBits: u32,
    Parity: u32,
    Mode: u32,
    HwFlowCtl: u32,
    OverSampling: u32,
}

/** @defgroup UART_Flags   UART FLags
  *        Elements values convention: 0xXXXX
  *           - 0xXXXX  : Flag mask in the SR register
  */
const UART_FLAG_CTS: u32 = 1 << 9;
const UART_FLAG_LBD: u32 = 1 << 8;
const UART_FLAG_TXE: u32 = 1 << 7;
const UART_FLAG_TC: u32 = 1 << 6;
const UART_FLAG_RXNE: u32 = 1 << 5;
const UART_FLAG_IDLE: u32 = 1 << 4;
const UART_FLAG_ORE: u32 = 1 << 3;
const UART_FLAG_NE: u32 = 1 << 2;
const UART_FLAG_FE: u32 = 1 << 1;
const UART_FLAG_PE: u32 = 1 << 0;

/******************  Bit definition for USART_CR1 register  *******************/
const USART_CR1_SBK: u32 = 1 << 0;
const USART_CR1_RWU: u32 = 1 << 1;
const USART_CR1_RE: u32 = 1 << 2;
const USART_CR1_TE: u32 = 1 << 3;
const USART_CR1_IDLEIE: u32 = 1 << 4;
const USART_CR1_RXNEIE: u32 = 1 << 5;
const USART_CR1_TCIE: u32 = 1 << 6;
const USART_CR1_TXEIE: u32 = 1 << 7;
const USART_CR1_PEIE: u32 = 1 << 8;
const USART_CR1_PS: u32 = 1 << 9;
const USART_CR1_PCE: u32 = 1 << 10;
const USART_CR1_WAKE: u32 = 1 << 11;
const USART_CR1_M: u32 = 1 << 12;
const USART_CR1_UE: u32 = 1 << 13;

/******************  Bit definition for USART_CR2 register  *******************/
const USART_CR2_ADD: u32 = 1 << 0;
const USART_CR2_LBDL: u32 = 1 << 5;
const USART_CR2_LBDIE: u32 = 1 << 6;
const USART_CR2_LBCL: u32 = 1 << 8;
const USART_CR2_CPHA: u32 = 1 << 9;
const USART_CR2_CPOL: u32 = 1 << 10;
const USART_CR2_CLKEN: u32 = 1 << 12;
const USART_CR2_STOP: u32 = 1 << 13;
const USART_CR2_LINEN: u32 = 1 << 14;

/******************  Bit definition for USART_CR3 register  *******************/
const USART_CR3_EIE: u32 = 1 << 0;
const USART_CR3_IREN: u32 = 1 << 1;
const USART_CR3_IRLP: u32 = 1 << 2;
const USART_CR3_HDSEL: u32 = 1 << 3;
const USART_CR3_NACK: u32 = 1 << 4;
const USART_CR3_SCEN: u32 = 1 << 5;
const USART_CR3_DMAR: u32 = 1 << 6;
const USART_CR3_DMAT: u32 = 1 << 7;
const USART_CR3_RTSE: u32 = 1 << 8;
const USART_CR3_CTSE: u32 = 1 << 9;
const USART_CR3_CTSIE: u32 = 1 << 10;

const UART_IT_MASK: u32 = USART_CR1_PEIE | USART_CR1_TXEIE | USART_CR1_TCIE | USART_CR1_RXNEIE | USART_CR1_IDLEIE | USART_CR2_LBDIE | USART_CR3_CTSIE | USART_CR3_EIE;

/** @defgroup UART_Interrupt_definition  UART Interrupt Definitions
  *        Elements values convention: 0xY000XXXX
  *           - XXXX  : Interrupt mask (16 bits) in the Y register
  *           - Y  : Interrupt source register (2bits)
  *                 - 0001: CR1 register
  *                 - 0010: CR2 register
  *                 - 0011: CR3 register
  */ 
const UART_CR1_REG_INDEX: u32 = 1;
const UART_CR2_REG_INDEX: u32 = 2;
const UART_CR3_REG_INDEX: u32 = 3;

const UART_IT_PE: u32 = (UART_CR1_REG_INDEX << 28 | USART_CR1_PEIE);
const UART_IT_TXE: u32 = (UART_CR1_REG_INDEX << 28 | USART_CR1_TXEIE);
const UART_IT_TC: u32 = (UART_CR1_REG_INDEX << 28 | USART_CR1_TCIE);
const UART_IT_RXNE: u32 = (UART_CR1_REG_INDEX << 28 | USART_CR1_RXNEIE);
const UART_IT_IDLE: u32 = (UART_CR1_REG_INDEX << 28 | USART_CR1_IDLEIE);
const UART_IT_LBD: u32 = (UART_CR2_REG_INDEX << 28 | USART_CR2_LBDIE);
const UART_IT_CTS: u32 = (UART_CR3_REG_INDEX << 28 | USART_CR3_CTSIE);
const UART_IT_ERR: u32 = (UART_CR3_REG_INDEX << 28 | USART_CR3_EIE);

const HAL_UART_STATE_RESET: u8 = 0x00; /* Peripheral is not initialized */
const HAL_UART_STATE_READY: u8 = 0x01; /* Peripheral Initialized and ready for use */
const HAL_UART_STATE_BUSY: u8 = 0x02; /* an internal process is ongoing */
const HAL_UART_STATE_BUSY_TX: u8 = 0x12; /* Data Transmission process is ongoing */
const HAL_UART_STATE_BUSY_RX: u8 = 0x22; /* Data Reception process is ongoing */
const HAL_UART_STATE_BUSY_TX_RX: u8 = 0x32; /* Data Transmission and Reception process is ongoing */
const HAL_UART_STATE_TIMEOUT: u8 = 0x03; /* Timeout state */
const HAL_UART_STATE_ERROR: u8 = 0x04; /* Error */

use lock::Lock;
const QUEUE_LENGTH: usize = 32;

use gpio;
use gpio::GPIOA;

use core::ptr;

#[repr(C)]
pub struct NewHandle<'a> {
    Instance: &'static mut Regs,
    Init: Init,
    pTxBuffPtr: &'a mut[u8],
    TxXferSize: u16,
    TxXferCount:u16,
    pRxBuffPtr: &'a mut[u8],
    RxXferSize: u16,
    RxXferCount: u16,
    lock: Lock,
    state: u8,
    errorCode: u32,
}

extern "C" {
    pub fn HAL_UART_Transmit_IT(husart: &mut Handle, pTxData: *const u8, Size: u16) -> u32;
    pub fn HAL_UART_Receive_IT(husart: &mut Handle, pRxData: *const u8, Size: u16) -> u32;
    pub fn HAL_UART_TransmitReceive_IT(husart: &mut Handle,
                                       pTxData: *const u8,
                                       pRxData: *const u8,
                                       Size: u16)
                                       -> u32;
    pub fn memcpy_offset(dst: *mut u8, src: *const u8, len: u8, offset: u8);
}

static mut TX_BUFFER:[u8;QUEUE_LENGTH] = [0;QUEUE_LENGTH];
static mut RX_BUFFER:[u8;QUEUE_LENGTH] = [0;QUEUE_LENGTH];


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

    pub fn SetBuffer(&mut self) {
        unsafe {
            self.pTxBuffPtr = TX_BUFFER.as_mut_ptr();
            self.TxXferSize =  QUEUE_LENGTH as u16;
            self.TxXferCount = 0;
            self.pRxBuffPtr = RX_BUFFER.as_mut_ptr();
            self.RxXferSize = QUEUE_LENGTH as u16;
            self.RxXferCount = 0;
        }
    }
    pub fn Transmit_Q(&mut self, pTxData: &[u8]) -> Status {
        GPIOA().WritePin(gpio::PIN_5, gpio::Level::Low);
        let ret: u32;
        unsafe {
            self.Lock = 1;
            // for i in 0..pTxData.len() {
            //     self.pTxBuffPtr[self.TxXferCount as usize + i] = pTxData[i];
            // }
            memcpy_offset(self.pTxBuffPtr, pTxData.as_ptr(), pTxData.len() as u8, self.TxXferCount as u8);
            self.TxXferCount += pTxData.len() as u16;
            self.State = HAL_UART_STATE_BUSY_TX;
            self.Lock = 0;
            HAL_UART_ENABLE_IT(&mut self.Instance, UART_IT_TXE);
        GPIOA().WritePin(gpio::PIN_5, gpio::Level::High);
            ret = 0;
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

#[inline]
fn HAL_UART_ENABLE_IT(huart: &mut Regs, int: u32) {
    if (int >> 28) == UART_CR1_REG_INDEX {
        huart.CR1 |= int & UART_IT_MASK;
    } else if (int >> 28) == UART_CR2_REG_INDEX {
        huart.CR2 |= int & UART_IT_MASK;
    } else {
        huart.CR3 |= int & UART_IT_MASK;
    }
}

impl<'a> NewHandle<'a> {
    pub fn new(regs: &'static mut Regs, init: Init) -> Self {
        unsafe {
            NewHandle {
                Instance: regs,
                Init: init,
                pTxBuffPtr: &mut TX_BUFFER,
                TxXferSize: QUEUE_LENGTH as u16,
                TxXferCount:0,
                pRxBuffPtr: &mut RX_BUFFER,
                RxXferSize: QUEUE_LENGTH as u16,
                RxXferCount: 0,
                state: HAL_UART_STATE_RESET,
                lock: Lock::Unlocked,
                errorCode: 0,
            }
        }
    }

    #[inline]
    pub fn TxAvailable(&self) -> usize {
        (self.TxXferSize - self.TxXferCount) as usize
    }

    pub fn Transmit(&mut self, pTxData: &[u8], size: usize) -> Option<usize> {
        if self.TxAvailable() >= size {
            self.lock.get_lock();
            for i in 0..size {
                self.pTxBuffPtr[self.TxXferCount as usize + i] = pTxData[i];
            }
            self.TxXferCount += size as u16;
            self.state = HAL_UART_STATE_BUSY_TX;
            self.lock.unlock();
            HAL_UART_ENABLE_IT(&mut self.Instance, UART_IT_TXE);
            Some(size)
        } else {
            None
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
