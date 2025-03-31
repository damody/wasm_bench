# Rust 與 WebAssembly 雙向呼叫示例

這個專案展示了如何在 Rust 程式中編譯和呼叫 WebAssembly 模組，以及如何實現雙向呼叫功能。

## 專案結構

- `wasm_lib/`: WebAssembly 庫程式碼
- `wasm_runner/`: Rust 主程式，用於呼叫 WebAssembly 函數

## 編譯步驟

1. 首先安裝 Rust 的 WebAssembly 目標：
```bash
rustup target add wasm32-unknown-unknown
```

2. 編譯 WebAssembly 庫：
```bash
cd wasm_lib
cargo build --target wasm32-unknown-unknown --release
```

3. 編譯動態連結檔：
```bash
cd rust_dylib
cargo build --release
```

4. 編譯並運行主程式：
```bash
cd ../wasm_runner
cargo run --release
```

## 功能說明

這個示例包含以下功能：

1. 基本數學運算：`add(a: i32, b: i32) -> i32`
2. 遞迴函數：`fibonacci(n: i32) -> i32`
3. 回調處理：`process_callback(callback_ptr: i32) -> i32`

## 範例輸出

運行程式後，你將看到以下輸出：
```
基本功能測試：
--------------
WASM: 5 + 37 = 42
Rust: 5 + 37 = 42
Lua:  5 + 37 = 42
DyLib: 5 + 37 = 42
WASM: Fibonacci(10) = 55
Rust: Fibonacci(10) = 55
Lua:  Fibonacci(10) = 55
DyLib: Fibonacci(10) = 55
WASM: 7 * 3 = 21
Rust: 7 * 3 = 21
Lua:  7 * 3 = 21
DyLib: 7 * 3 = 21

性能比較測試：
--------------

加法性能比較：
Rust 原生加法: 1000000 次迭代耗時 0ns，平均每次 0ns，最後結果 42
WASM 加法: 1000000 次迭代耗時 17.0666ms，平均每次 17ns，最後結果 42
LuaJIT 加法: 1000000 次迭代耗時 33.4592ms，平均每次 33ns，最後結果 42
動態鏈結庫加法: 1000000 次迭代耗時 861.4µs，平均每次 0ns，最後結果 42

斐波那契數列性能比較：
Rust 原生斐波那契: 1000 次迭代耗時 400ns，平均每次 0ns，最後結果 55
WASM 斐波那契: 1000 次迭代耗時 147.8µs，平均每次 147ns，最後結果 55
LuaJIT 斐波那契: 1000 次迭代耗時 669.7µs，平均每次 669ns，最後結果 55
動態鏈結庫斐波那契: 1000 次迭代耗時 138.4µs，平均每次 138ns，最後結果 55

乘法（跨邊界調用）性能比較：
Rust 原生乘法: 1000000 次迭代耗時 100ns，平均每次 0ns，最後結果 21
WASM 乘法（含跨邊界調用）: 1000000 次迭代耗時 22.6936ms，平均每次 22ns，最後結果 21
LuaJIT 乘法: 1000000 次迭代耗時 32.158ms，平均每次 32ns，最後結果 21
動態鏈結庫乘法: 1000000 次迭代耗時 859.4µs，平均每次 0ns，最後結果 21
``` 