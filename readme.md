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

三番目は xargoがやってくれるので、一番目を行う `build.rs`を書く。

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
```

* 次のものは、ライブラリ側ではなく、アプリ側で持つ(通常は CubeMXで生成される)。
  + スタートアップ(`startup_stm32f103xb.s`) ⇒ `Drivers/CMSIS/Device/ST/STM32F1xx/Source/Templates/gcc/`にテンプレートがあって同じなので、アプリ側で `build.rs`を書かせるよりは、ライブラリ側でリンクしてしまってもいいかも。
  + スタートアップから呼ばれるシステム初期化(`system_stm32f1xx.c:SystemInit()`)
  + クロック初期化(`main.c:SystemClock_Config()`)
  + 割り込みハンドラ(`stm32f1xx_it.c`)
  + リンカスクリプト(`STM32F103RBTx_FLASH.ld`)
