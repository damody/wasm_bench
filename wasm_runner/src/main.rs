use anyhow::Result;
use wasmtime::*;
use std::time::Instant;
use mlua::prelude::*;
use libloading::{Library, Symbol};

// 這個函數將被導出到 WebAssembly
fn host_function(_caller: Caller<'_, ()>, x: i32) -> i32 {
    x * 3
}

// Rust 原生函數，用於性能比較
fn native_add(a: i32, b: i32) -> i32 {
    a + b
}

fn native_fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    native_fibonacci(n - 1) + native_fibonacci(n - 2)
}

fn native_multiply_by_three(x: i32) -> i32 {
    x * 3
}

// 動態鏈結庫函數類型定義
type DylibAddFunc = unsafe fn(i32, i32) -> i32;
type DylibFibonacciFunc = unsafe fn(i32) -> i32;
type DylibMultiplyByThreeFunc = unsafe fn(i32) -> i32;

#[allow(dead_code)]  // 抑制未使用字段的警告
struct DylibFunctions<'lib> {
    lib: &'lib Library,  // 改為引用，需要保持以確保正確的生命週期
    add: Symbol<'lib, DylibAddFunc>,
    fibonacci: Symbol<'lib, DylibFibonacciFunc>,
    multiply_by_three: Symbol<'lib, DylibMultiplyByThreeFunc>,
}

impl<'lib> DylibFunctions<'lib> {
    fn load() -> Result<Self> {
        // 根據平台選擇正確的庫文件名
        let lib_path = if cfg!(target_os = "windows") {
            "../rust_dylib/target/release/rust_dylib.dll"
        } else if cfg!(target_os = "macos") {
            "../rust_dylib/target/release/librust_dylib.dylib"
        } else {
            "../rust_dylib/target/release/librust_dylib.so"
        };

        unsafe {
            // 使用 Box::leak 來創建一個靜態生命週期的引用
            let lib = Box::leak(Box::new(Library::new(lib_path)?));
            
            // 加載所有符號
            let add = lib.get(b"dylib_add")?;
            let fibonacci = lib.get(b"dylib_fibonacci")?;
            let multiply_by_three = lib.get(b"dylib_multiply_by_three")?;

            Ok(DylibFunctions {
                lib,
                add,
                fibonacci,
                multiply_by_three,
            })
        }
    }
}

// 性能測試函數
fn benchmark_function<F>(name: &str, iterations: u32, mut f: F)
where
    F: FnMut() -> i32,
{
    let start = Instant::now();
    let mut result = 0;
    for _ in 0..iterations {
        result = f();
    }
    let duration = start.elapsed();
    println!(
        "{}: {} 次迭代耗時 {:?}，平均每次 {:?}，最後結果 {}",
        name,
        iterations,
        duration,
        duration / iterations,
        result
    );
}

fn setup_lua(lua: &Lua) -> Result<LuaTable> {
    // 定義 Lua 函數
    lua.load(r#"
        -- 基本函數
        function add(a, b)
            return a + b
        end

        function fibonacci(n)
            if n <= 1 then
                return n
            end
            return fibonacci(n-1) + fibonacci(n-2)
        end

        function multiply_by_three(x)
            return x * 3
        end

        -- 預編譯的測試函數
        test_add = function(a, b) return add(a, b) end
        test_fib = function(n) return fibonacci(n) end
        test_mul = function(x) return multiply_by_three(x) end
    "#).exec()?;

    // 創建返回表
    let funcs = lua.create_table()?;
    
    // 獲取全局函數並存入表中
    funcs.set("add", lua.globals().get::<_, LuaFunction>("add")?)?;
    funcs.set("fibonacci", lua.globals().get::<_, LuaFunction>("fibonacci")?)?;
    funcs.set("multiply_by_three", lua.globals().get::<_, LuaFunction>("multiply_by_three")?)?;
    funcs.set("test_add", lua.globals().get::<_, LuaFunction>("test_add")?)?;
    funcs.set("test_fib", lua.globals().get::<_, LuaFunction>("test_fib")?)?;
    funcs.set("test_mul", lua.globals().get::<_, LuaFunction>("test_mul")?)?;

    Ok(funcs)
}

fn main() -> Result<()> {
    // 載入動態鏈結庫
    let dylib = DylibFunctions::load()?;

    // 創建配置和引擎
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    // 定義要導出到 WebAssembly 的函數
    let host_func = Func::wrap(&mut store, host_function);

    // 創建 linker 並添加導入函數
    let mut linker = Linker::new(&engine);
    linker.define(&mut store, "host", "host_function", host_func)?;

    // 讀取並編譯 wasm 模組
    let module = Module::from_file(&engine, "../wasm_lib/target/wasm32-unknown-unknown/release/wasm_lib.wasm")?;
    
    // 使用 linker 創建實例
    let instance = linker.instantiate(&mut store, &module)?;

    // 獲取導出的函數
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let fibonacci = instance.get_typed_func::<i32, i32>(&mut store, "fibonacci")?;
    let call_host = instance.get_typed_func::<i32, i32>(&mut store, "call_host_function")?;

    // 設置 Lua 環境
    let lua = Lua::new();
    let lua_funcs = setup_lua(&lua)?;

    println!("基本功能測試：");
    println!("--------------");
    
    // 測試基本功能
    let result = add.call(&mut store, (5, 37))?;
    println!("WASM: 5 + 37 = {}", result);
    println!("Rust: 5 + 37 = {}", native_add(5, 37));
    println!("Lua:  5 + 37 = {}", lua_funcs.get::<_, LuaFunction>("add")?.call::<_, i32>((5, 37))?);
    println!("DyLib: 5 + 37 = {}", unsafe { (dylib.add)(5, 37) });

    let result = fibonacci.call(&mut store, 10)?;
    println!("WASM: Fibonacci(10) = {}", result);
    println!("Rust: Fibonacci(10) = {}", native_fibonacci(10));
    println!("Lua:  Fibonacci(10) = {}", lua_funcs.get::<_, LuaFunction>("fibonacci")?.call::<_, i32>(10)?);
    println!("DyLib: Fibonacci(10) = {}", unsafe { (dylib.fibonacci)(10) });

    let input = 7;
    let result = call_host.call(&mut store, input)?;
    println!("WASM: 7 * 3 = {}", result);
    println!("Rust: 7 * 3 = {}", native_multiply_by_three(7));
    println!("Lua:  7 * 3 = {}", lua_funcs.get::<_, LuaFunction>("multiply_by_three")?.call::<_, i32>(7)?);
    println!("DyLib: 7 * 3 = {}", unsafe { (dylib.multiply_by_three)(7) });

    println!("\n性能比較測試：");
    println!("--------------");

    // 性能測試參數
    let iterations = 1000000; // 一百萬次迭代
    let fib_iterations = 1000; // 斐波那契數列用較少的迭代次數
    let test_add_inputs = (5, 37);
    let test_fib_input = 10;
    let test_mul_input = 7;

    // 獲取預編譯的 Lua 測試函數
    let lua_test_add = lua_funcs.get::<_, LuaFunction>("test_add")?;
    let lua_test_fib = lua_funcs.get::<_, LuaFunction>("test_fib")?;
    let lua_test_mul = lua_funcs.get::<_, LuaFunction>("test_mul")?;

    // 測試加法性能
    println!("\n加法性能比較：");
    benchmark_function("Rust 原生加法", iterations, || {
        native_add(test_add_inputs.0, test_add_inputs.1)
    });
    benchmark_function("WASM 加法", iterations, || {
        add.call(&mut store, test_add_inputs).unwrap()
    });
    benchmark_function("LuaJIT 加法", iterations, || {
        lua_test_add.call::<_, i32>((test_add_inputs.0, test_add_inputs.1)).unwrap()
    });
    benchmark_function("動態鏈結庫加法", iterations, || {
        unsafe { (dylib.add)(test_add_inputs.0, test_add_inputs.1) }
    });

    // 測試斐波那契性能
    println!("\n斐波那契數列性能比較：");
    benchmark_function("Rust 原生斐波那契", fib_iterations, || {
        native_fibonacci(test_fib_input)
    });
    benchmark_function("WASM 斐波那契", fib_iterations, || {
        fibonacci.call(&mut store, test_fib_input).unwrap()
    });
    benchmark_function("LuaJIT 斐波那契", fib_iterations, || {
        lua_test_fib.call::<_, i32>(test_fib_input).unwrap()
    });
    benchmark_function("動態鏈結庫斐波那契", fib_iterations, || {
        unsafe { (dylib.fibonacci)(test_fib_input) }
    });

    // 測試乘法和跨邊界調用性能
    println!("\n乘法（跨邊界調用）性能比較：");
    benchmark_function("Rust 原生乘法", iterations, || {
        native_multiply_by_three(test_mul_input)
    });
    benchmark_function("WASM 乘法（含跨邊界調用）", iterations, || {
        call_host.call(&mut store, test_mul_input).unwrap()
    });
    benchmark_function("LuaJIT 乘法", iterations, || {
        lua_test_mul.call::<_, i32>(test_mul_input).unwrap()
    });
    benchmark_function("動態鏈結庫乘法", iterations, || {
        unsafe { (dylib.multiply_by_three)(test_mul_input) }
    });

    Ok(())
}