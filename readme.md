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

pub mod stm32f1xx_hal_gpio;
```
* `#![no_std]`などは、`src/lib.rs`で定義する。
* 子モジュールを公開する。 

### stm32f1xx_hal_gpio.rs

```
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
```

* レジスタなどを定義する
* 