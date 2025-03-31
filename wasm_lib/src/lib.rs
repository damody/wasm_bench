#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// 聲明從主程式導入的函數
#[link(wasm_import_module = "host")]
extern "C" {
    fn host_function(x: i32) -> i32;
}

// 這個函數將調用 Rust 主程式中的函數
#[no_mangle]
pub extern "C" fn call_host_function(x: i32) -> i32 {
    // 安全地調用導入的函數
    unsafe {
        host_function(x)
    }
} 