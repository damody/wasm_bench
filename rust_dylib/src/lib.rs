
#[no_mangle]
pub extern "C" fn dylib_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn dylib_fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    dylib_fibonacci(n - 1) + dylib_fibonacci(n - 2)
}

#[no_mangle]
pub extern "C" fn dylib_multiply_by_three(x: i32) -> i32 {
    x * 3
}
