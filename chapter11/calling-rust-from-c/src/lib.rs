#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub extern "C" fn sub(x: i32, y: i32) -> i32 {
    x - y
}

#[no_mangle]
pub extern "C" fn mul(x: i32, y: i32) -> i32 {
    x * y
}

#[no_mangle]
pub extern "C" fn div(x: i32, y: i32) -> i32 {
    x / y
}
