#[no_mangle]
pub extern "C" fn print_int(integer: i64) -> i64 {
    println!("{}", integer);
    integer
}
