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

3. 編譯並運行主程式：
```bash
cd ../wasm_runner
cargo run
```

## 功能說明

這個示例包含以下功能：

1. 基本數學運算：`add(a: i32, b: i32) -> i32`
2. 遞迴函數：`fibonacci(n: i32) -> i32`
3. 回調處理：`process_callback(callback_ptr: i32) -> i32`

## 範例輸出

運行程式後，你將看到以下輸出：
```
5 + 37 = 42
Fibonacci(10) = 55
Callback result: 84
``` 