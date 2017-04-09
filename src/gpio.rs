#![allow(non_snake_case)]

// レジスタアドレスの定義
const PERIPH_BASE: u32 = 0x40000000;

const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x10000;
const GPIOA_BASE: u32 = APB2PERIPH_BASE + 0x0800;
const GPIOB_BASE: u32 = APB2PERIPH_BASE + 0x0C00;
const GPIOC_BASE: u32 = APB2PERIPH_BASE + 0x1000;
const GPIOD_BASE: u32 = APB2PERIPH_BASE + 0x1400;
const GPIOE_BASE: u32 = APB2PERIPH_BASE + 0x1800;


pub const PIN_0: u16 = 0x0001;
pub const PIN_1: u16 = 0x0002;
pub const PIN_2: u16 = 0x0004;
pub const PIN_3: u16 = 0x0008;
pub const PIN_4: u16 = 0x0010;
pub const PIN_5: u16 = 0x0020;
pub const PIN_6: u16 = 0x0040;
pub const PIN_7: u16 = 0x0080;
pub const PIN_8: u16 = 0x0100;
pub const PIN_9: u16 = 0x0200;
pub const PIN_10: u16 = 0x0400;
pub const PIN_11: u16 = 0x0800;
pub const PIN_12: u16 = 0x1000;
pub const PIN_13: u16 = 0x2000;
pub const PIN_14: u16 = 0x4000;
pub const PIN_15: u16 = 0x8000;

pub const MODE_INPUT: u32 = 0x00000000;
pub const MODE_OUTPUT_PP: u32 = 0x00000001;
pub const MODE_OUTPUT_OD: u32 = 0x00000011;
pub const MODE_AF_PP: u32 = 0x00000002;
pub const MODE_AF_OD: u32 = 0x00000012;
pub const MODE_AF_INPUT: u32 = MODE_INPUT;
pub const MODE_ANALOG: u32 = 0x00000003;
pub const MODE_IT_RISING: u32 = 0x10110000;
pub const MODE_IT_FALLING: u32 = 0x10210000;
pub const MODE_IT_RISING_FALLING: u32 = 0x10310000;
pub const MODE_EVT_RISING: u32 = 0x10120000;
pub const MODE_EVT_FALLING: u32 = 0x10220000;
pub const MODE_EVT_RISING_FALLING: u32 = 0x10320000;

pub const SPEED_FREQ_LOW: u32 = 0x00000002;
pub const SPEED_FREQ_MEDIUM: u32 = 0x00000001;
pub const SPEED_FREQ_HIGH: u32 = 0x00000003;

pub const NOPULL: u32 = 0x00000000;
pub const PULLUP: u32 = 0x00000001;
pub const PULLDOWN: u32 = 0x00000002;

#[repr(C)] // C の struct のインポート
pub struct InitTypeDef {
    pub Pin: u32,
    pub Mode: u32,
    pub Pull: u32,
    pub Speed: u32,
}

#[repr(C)]
pub struct TypeDef {
    CRL: u32,
    CRH: u32,
    IDR: u32,
    ODR: u32,
    BSRR: u32,
    BRR: u32,
    LCKR: u32,
}

extern "C" {
    pub fn HAL_GPIO_Init(GPIOx: &mut TypeDef, GPIO_Init: &InitTypeDef);
    pub fn HAL_GPIO_WritePin(GPIOx: &mut TypeDef, GPIO_Pin: u16, PinState: u32);
    pub fn HAL_GPIO_ReadPin(GPIOx: &mut TypeDef, GPIO_Pin: u16) -> u32;
}

impl TypeDef {
    pub fn Init(&mut self, GPIO_Init: &InitTypeDef) -> () {
        unsafe {
            HAL_GPIO_Init(self, GPIO_Init);
        }
    }

    pub fn WritePin(&mut self, GPIO_Pin: u16, PinState: u32) -> () {
        unsafe {
            HAL_GPIO_WritePin(self, GPIO_Pin, PinState);
        }
    }

    pub fn ReadPin(&mut self, GPIO_Pin: u16) -> u32 {
        let ret: u32;
        unsafe {
            ret = HAL_GPIO_ReadPin(self, GPIO_Pin);
        }
        ret
    }
}

pub fn GPIOA() -> &'static mut TypeDef {
    unsafe { &mut *(GPIOA_BASE as *mut TypeDef) }
}
pub fn GPIOB() -> &'static mut TypeDef {
    unsafe { &mut *(GPIOB_BASE as *mut TypeDef) }
}
pub fn GPIOC() -> &'static mut TypeDef {
    unsafe { &mut *(GPIOC_BASE as *mut TypeDef) }
}
pub fn GPIOD() -> &'static mut TypeDef {
    unsafe { &mut *(GPIOD_BASE as *mut TypeDef) }
}
pub fn GPIOE() -> &'static mut TypeDef {
    unsafe { &mut *(GPIOE_BASE as *mut TypeDef) }
}
