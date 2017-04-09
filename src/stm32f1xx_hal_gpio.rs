#![allow(non_snake_case)]

use core::intrinsics::volatile_store; // メモリ直書きには volatile_store を使う。

// レジスタアドレスの定義
const PERIPH_BASE: u32      = 0x40000000;

const APB2PERIPH_BASE: u32  = PERIPH_BASE + 0x10000;
const GPIOA_BASE: u32       = APB2PERIPH_BASE + 0x0800;
pub const GPIO_PIN_5: u16   = 0x0020;

const AHBPERIPH_BASE: u32   = PERIPH_BASE + 0x20000;
const RCC_BASE: u32         = AHBPERIPH_BASE + 0x1000;
const APB2ENR_OFFSET: u32   = 0x18;

#[repr(C)] // C の struct のインポート
pub struct GPIO_InitTypeDef {
    pub Pin: u32,
    pub Mode: u32,
    pub Pull: u32,
    pub Speed: u32
}

#[repr(C)]
pub struct TypeDef {
    CRL: u32,
    CRH: u32,
    IDR: u32,
    ODR: u32,
    BSRR: u32,
    BRR: u32,
    LCKR: u32
}

extern {
    pub fn HAL_GPIO_Init(GPIOx: &mut TypeDef, GPIO_Init: &GPIO_InitTypeDef);
    pub fn HAL_GPIO_WritePin(GPIOx: &mut TypeDef, GPIO_Pin: u16, PinState: u32);
}

pub fn Init(GPIOx: &mut TypeDef, GPIO_Init: &GPIO_InitTypeDef) -> () {
    unsafe {
        HAL_GPIO_Init(GPIOx, GPIO_Init);
    }
}

pub fn WritePin(GPIOx: &mut TypeDef, GPIO_Pin: u16, PinState: u32) -> () {
    unsafe {
        HAL_GPIO_WritePin(GPIOx, GPIO_Pin, PinState);
    }
}

pub fn GPIOA() -> &'static mut TypeDef {unsafe {&mut *(GPIOA_BASE as *mut TypeDef)}}

/*
#define __HAL_RCC_GPIOA_CLK_ENABLE()   do { \
                                        __IO uint32_t tmpreg; \
                                        SET_BIT(RCC->APB2ENR, RCC_APB2ENR_IOPAEN);\
                                        /* Delay after an RCC peripheral clock enabling */\
                                        tmpreg = READ_BIT(RCC->APB2ENR, RCC_APB2ENR_IOPAEN);\
                                        UNUSED(tmpreg); \
                                      } while(0)
*/
pub fn GPIOA_CLK_ENABLE() -> () {
    let apb2enr = (RCC_BASE + APB2ENR_OFFSET) as *mut u32;
    unsafe {
        volatile_store(apb2enr, *apb2enr | (1 << 2));
    }
}

