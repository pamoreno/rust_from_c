#[unsafe(no_mangle)]
pub extern "C" fn greet_from_rust() {
    println!("Hello from Rust!");
}

#[unsafe(no_mangle)]
pub extern "C" fn add_numbers(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_numbers(2, 2);
        assert_eq!(result, 4);
    }
}
