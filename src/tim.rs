#![allow(non_snake_case)]

// レジスタアドレスの定義
const PERIPH_BASE: u32 = 0x40000000;

const APB1PERIPH_BASE: u32 = PERIPH_BASE;
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x10000;

const TIM2_BASE: u32 = APB1PERIPH_BASE + 0x0000;
const TIM3_BASE: u32 = APB1PERIPH_BASE + 0x0400;
const TIM4_BASE: u32 = APB1PERIPH_BASE + 0x0800;
const TIM1_BASE: u32 = APB2PERIPH_BASE + 0x2C00;

#[repr(C)]
pub struct Regs {
    CR1: u32,
    CR2: u32,
    SMCR: u32,
    DIER: u32,
    SR: u32,
    EGR: u32,
    CCMR1: u32,
    CCMR2: u32,
    CCER: u32,
    CNT: u32,
    PSC: u32,
    ARR: u32,
    RCR: u32,
    CCR1: u32,
    CCR2: u32,
    CCR3: u32,
    CCR4: u32,
    BDTR: u32,
    DCR: u32,
    DMAR: u32,
    OR: u32,
}

#[repr(C)]
pub struct BASE_Init {
    Prescaler: u32,
    CounterMode: u32,
    Period: u32,
    ClockDivision: u32,
    RepetitionCounter: u32,
}

#[repr(C)] // C の struct のインポート
pub struct Handle {
    TIM_Regs: Regs,
    TIM_Base_Init: BASE_Init,
    HAL_TIM_ActiveChannel: u32,
    DMA_Handle: u32,
    HAL_LockRegs: u32,
    HAL_TIM_StateRegs: u32,
}

extern "C" {
    pub fn HAL_TIM_Base_Init(htim: &Handle);
    pub fn HAL_TIM_Base_Start(htim: &Handle);
    pub fn HAL_TIM_Base_Stop(htim: &Handle);
    pub fn HAL_TIM_Base_Start_IT(htim: &Handle);
    pub fn HAL_TIM_Base_Stop_IT(htim: &Handle);
    pub fn HAL_TIM_OC_Init(htim: &Handle);
    pub fn HAL_TIM_OC_Start(htim: &Handle);
    pub fn HAL_TIM_OC_Stop(htim: &Handle);
    pub fn HAL_TIM_OC_Start_IT(htim: &Handle);
    pub fn HAL_TIM_OC_Stop_IT(htim: &Handle);
    pub fn HAL_TIM_PWM_Init(htim: &Handle);
    pub fn HAL_TIM_PWM_Start(htim: &Handle);
    pub fn HAL_TIM_PWM_Stop(htim: &Handle);
    pub fn HAL_TIM_PWM_Start_IT(htim: &Handle);
    pub fn HAL_TIM_PWM_Stop_IT(htim: &Handle);
}

impl Handle {
    pub fn Base_Init(&mut self) -> () {
        unsafe {
            HAL_TIM_Base_Init(self);
        }
    }

    pub fn Base_Start(&mut self) -> () {
        unsafe {
            HAL_TIM_Base_Start(self);
        }
    }

    pub fn Base_Stop(&mut self) -> () {
        unsafe {
            HAL_TIM_Base_Stop(self);
        }
    }

    pub fn Base_Start_IT(&mut self) -> () {
        unsafe {
            HAL_TIM_Base_Start_IT(self);
        }
    }

    pub fn Base_Stop_IT(&mut self) -> () {
        unsafe {
            HAL_TIM_Base_Stop_IT(self);
        }
    }

    pub fn OC_Init(&mut self) -> () {
        unsafe {
            HAL_TIM_OC_Init(self);
        }
    }

    pub fn OC_Start(&mut self) -> () {
        unsafe {
            HAL_TIM_OC_Start(self);
        }
    }

    pub fn OC_Stop(&mut self) -> () {
        unsafe {
            HAL_TIM_OC_Stop(self);
        }
    }

    pub fn OC_Start_IT(&mut self) -> () {
        unsafe {
            HAL_TIM_OC_Start_IT(self);
        }
    }

    pub fn OC_Stop_IT(&mut self) -> () {
        unsafe {
            HAL_TIM_OC_Stop_IT(self);
        }
    }

    pub fn PWM_Init(&mut self) -> () {
        unsafe {
            HAL_TIM_PWM_Init(self);
        }
    }

    pub fn PWM_Start(&mut self) -> () {
        unsafe {
            HAL_TIM_PWM_Start(self);
        }
    }

    pub fn PWM_Stop(&mut self) -> () {
        unsafe {
            HAL_TIM_PWM_Stop(self);
        }
    }

    pub fn PWM_Start_IT(&mut self) -> () {
        unsafe {
            HAL_TIM_PWM_Start_IT(self);
        }
    }

    pub fn PWM_Stop_IT(&mut self) -> () {
        unsafe {
            HAL_TIM_PWM_Stop_IT(self);
        }
    }
}

pub fn TIM2() -> &'static mut Regs {
    unsafe { &mut *(TIM2_BASE as *mut Regs) }
}
pub fn TIM3() -> &'static mut Regs {
    unsafe { &mut *(TIM3_BASE as *mut Regs) }
}
pub fn TIM4() -> &'static mut Regs {
    unsafe { &mut *(TIM4_BASE as *mut Regs) }
}
pub fn TIM1() -> &'static mut Regs {
    unsafe { &mut *(TIM1_BASE as *mut Regs) }
}
