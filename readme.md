# How to build library

1. Download STM32Cube Firmware pagkage from http://www.st.com
2. Extract STM32Cube_FW_**_V*.*.*.zip in this directory.
3. set the directory name on cube_top variable in `build.rs`.
4. `$ xargo build`.

You can use lib crate on your application.

```
extern crete stm32cubef1;
use stm32cubef1;
```
in your application.

# How to build documents

```
$ rustdoc src/lib.rs
```
and read doc/lib/index.html

--------------------------

# work logs

## make new project

```
$ xargo new stm32cubef1
     Created library `stm32cubef1` project
$ cd stm32cubef1/
$ xargo test
   Compiling stm32cubef1 v0.1.0 (file://$(PROJECTS)/stm32cubef1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
     Running target/debug/deps/stm32cubef1-1e5d0cc818b54107

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests stm32cubef1

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

$ tree .
.
├── Cargo.lock
├── Cargo.toml
├── readme.md
├── src
│   └── lib.rs
└── target
    └── debug
        ├── build
        ├── deps
        │   ├── libstm32cubef1-18a29c0a3ade3fd6.rlib
        │   └── stm32cubef1-1e5d0cc818b54107
        ├── examples
        ├── incremental
        ├── libstm32cubef1.d
        ├── libstm32cubef1.rlib
        ├── native
        ├── stm32cubef1-1e5d0cc818b54107
        └── stm32cubef1-1e5d0cc818b54107.d

8 directories, 10 files

$ cat src/lib.rs 
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

* `cargo`ではなく`xargo`でプロジェクトを作成する。
* `--bin`を付けないと lib crate を作成する。
* lib crate では src/lib.rs がエントリーポイント。
* デフォルトではデモテストが生成されている。`xargo test`でテストが走る。

## Extract Cube_FW

STM32CubeMXを起動して最新のFWパッケージをダウンロードしておく。
`~/STM32CubeMX/Repository/stm32cube_fw_f1_v140.zip`にあるので、プロジェクトディレクトリに展開する。
大量にあるので、ディレクトリごと`.gitignoer`にも登録しておこう。

## 名前空間

これを使う他のプロジェクトから見ると次のような対応になる。

|    パス              |    見え方                    |
|----------------------|------------------------------|
|src/lib.rs            |stm32cubef1::*                |
|src/lib.rs:func_A()   |stm32cubef1::func_A()         |
|src/gpio.rs:write()   |stm32cubef1::gpio::write()    |
|src/adc/lib.rs:read() |stm32cubef1::adc::read()      |
|src/adc/ex.rs:config()|stm32cubef1::adc::rx::config()|

* `lib.rs`はルート、`.`、`index.html`、`readme.md`のようなもの。
* `use stm32cubef1`としたら、表の「見え方」の通りに見える。
* `use stm32cubef1::*`としたら、`stm32cubef1::`を省略したように見える。

## build.rs

* Cのファイルは、gcc でビルドして `libstm32cubef1.a`にまとめる。
* rustでラッパを書く。
* (libcoreと)それらをまとめて、`libstm32cubef1.rlib`とする。

3番目は xargoがやってくれるので、1番目を行う `build.rs`を書く。

`Cargo.toml`
```
[package]
name = "stm32cubef1"
version = "0.1.0"
authors = ["My Name <my_email@example.com>"]
build = "build.rs"
```

`build.rs`
```
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let cube_top = "STM32Cube_FW_F1_V1.4.0";

    let out_dir = env::var("OUT_DIR").unwrap();

    let inc_dirs = [
        &format!("-I{}/Drivers/CMSIS/Device/ST/STM32F1xx/Include", cube_top),
        &format!("-I{}/Drivers/CMSIS/Include", cube_top),
        &format!("-I{}/Drivers/STM32F1xx_HAL_Driver/Inc", cube_top),
        "-Icubemx/Inc"
    ];

    let defines = [
        "-DSTM32F103xB"
    ];

    let srcs = [
        [&format!("{}/Drivers/STM32F1xx_HAL_Driver/Src", cube_top), "stm32f1xx_hal_gpio.c"],
        [&format!("{}/Drivers/STM32F1xx_HAL_Driver/Src", cube_top), "stm32f1xx_hal_rcc.c"],
        [&format!("{}/Drivers/STM32F1xx_HAL_Driver/Src", cube_top), "stm32f1xx_hal_cortex.c"],
        [&format!("{}/Drivers/STM32F1xx_HAL_Driver/Src", cube_top), "stm32f1xx_hal.c"]
    ];

    let mut objs: Vec<String> = Vec::new();

    for src in &srcs {
        let obj = src[1].to_string().replace(".c", ".o");

        Command::new("arm-none-eabi-gcc")
            .arg("-c")
            .args(&["-mcpu=cortex-m3", "-mthumb", "-mfloat-abi=soft"])
            .args(&defines)
            .args(&inc_dirs)
            .arg(&format!("{}/{}",src[0], src[1]))
            .arg("-o")
            .arg(&format!("{}/{}", out_dir, obj))
            .status().unwrap();

        objs.push(obj);
    }

    Command::new("arm-none-eabi-ar")
        .args(&["crus", "libstm32cubef1.a"])
        .args(&objs)
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=stm32cubef1");

    println!("cargo:rerun-if-changed=build.rs");
}
```
* ライブラリでは、Cube_FWをコンパイルして、`libstm32cubef1.a`に突っ込む。
* ラッパーインタフェースを作成して、`src/lib.rs`につなげる。
* 次のものは、ライブラリ側ではなく、アプリ側で持つ(通常は CubeMXで生成される)。
  + スタートアップ(`startup_stm32f103xb.s`) ⇒ `Drivers/CMSIS/Device/ST/STM32F1xx/Source/Templates/gcc/`にテンプレートがあって同じなので、アプリ側で `build.rs`を書かせるよりは、ライブラリ側でリンクしてしまってもいいかも
  + スタートアップから呼ばれるシステム初期化(`system_stm32f1xx.c:SystemInit()`)
  + クロック初期化(`main.c:SystemClock_Config()`)
  + 割り込みハンドラ(`stm32f1xx_it.c`)
  + リンカスクリプト(`STM32F103RBTx_FLASH.ld`)

ビルドには、通常はアプリ側で生成される`stm32f1xx_hal_conf.h`などが無いといけない。適当に生成したプロジェクトから`cubemx/`以下にコピーしてまかなう。

![architecture.png](architecture.png)

### lib.rs
```
#![no_std]
#![feature(core_intrinsics)] // core_intrinsics を使う。子ファイルではなく、lib.rs に必要。

pub mod gpio;
```
* `#![no_std]`などは、`src/lib.rs`で定義する。
* 子モジュールを公開する。 

### gpio.rs

```
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
```
* ネームスペースを活用したネーミングにする。
* レジスタなどを定義する
  + レジスタバンクを `#[repl(C)]`の構造体として定義する。
  + GPIO ポートごとに、マクロでアドレスを指定する。
* APIは、レジスタバンク(の構造体)に対するトレイトとして実装する。
  + そうすることで、オブジェクト指向っぽいインスタンスになる。
  + 記述も短くなる。

### uart.rs

UART と USART があるが、CubeMXのインタフェースとしては UARTを使えば良い。

```#![allow(non_snake_case)]

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
pub struct TypeDef {
    SR: u32, /* USART Status register */
    DR: u32, /* USART Data register */
    BRR: u32, /* USART Baud rate register */
    CR1: u32, /* USART Control register 1 */
    CR2: u32, /* USART Control register 2 */
    CR3: u32, /* USART Control register 3 */
    BTPR: u32, /* USART Guard time and prescaler register */
}

#[repr(C)]
pub struct HandleTypeDef {
    dummy: u32,
}

extern "C" {
    pub fn HAL_UART_Transmit_IT(husart: &mut HandleTypeDef, pTxData: *const u8, Size: u16) -> u32;
    pub fn HAL_UART_Receive_IT(husart: &mut HandleTypeDef, pRxData: *const u8, Size: u16) -> u32;
}

impl HandleTypeDef {
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

pub fn USART2() -> &'static mut TypeDef {
    unsafe { &mut *(USART2_BASE as *mut TypeDef) }
}
pub fn USART3() -> &'static mut TypeDef {
    unsafe { &mut *(USART3_BASE as *mut TypeDef) }
}
pub fn USART1() -> &'static mut TypeDef {
    unsafe { &mut *(USART1_BASE as *mut TypeDef) }
}
```

* API は、レジスタ群(TypeDef)ではなく、ハンドル構造体(HandleTypeDef)に対して定義されている。
* しかし、huart の実体は CubeMXによって生成された `usart.c`で行われるので、ライブラリ側ではなくアプリ側での作業が必要。
* HALの HandleTypeDef API は、いくつも問題があるので、より良いAPIを作りたい。
    + 送信関数が、送信データのバッファを直接参照している。⇒ スタック上のバッファを渡せない。
    + グローバルなバッファに書いて、送信関数にバッファのポインタを渡すと、グローバルなバッファが競合する。
    + 受信のコールバックが書きにくい。

    