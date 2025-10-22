Calling Rust code from C involves using Rust's Foreign Function Interface (FFI) capabilities to expose Rust functions with a C-compatible Application Binary Interface (ABI). This allows C code to link against and call these Rust functions as if they were written in C.

## Steps to Call Rust from C


-   Define Rust functions for C ABI:
    -   Mark the Rust function with `#[no_mangle]` to prevent name mangling, ensuring a stable, predictable name for C.
    -   Declare the function as `pub extern "C"` to specify the C calling convention and make it publicly accessible. 

        ```rust
        // some_lib/src/lib.rs
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
        ```

-   Compile Rust code as a static or dynamic library:
    -   In your `Cargo.toml`, set the `crate-type` to `staticlib` or `cdylib`. 

        ```toml
        [package]
        name = "some_lib"
        version = "0.1.0"
        edition = "2024"

        [lib]
        crate-type = ["staticlib"]

        [dependencies]
        ```


-   Build the Rust project: `cargo build --release` (or without `--release` for debugging). This will generate `libsome_lib.a` (static) or `libsome_lib.so` (dynamic) in `target/release/` (or `target/debug/`). 

-   Declare Rust functions in C:
    -   Create a C header file (.h) to declare the Rust functions with their C-compatible signatures. 

        ```C
        #pragma once

        extern void greet_from_rust();
        extern int add_numbers(int a, int b);
        ```

-   Call Rust functions from C:
    -   Include the header file in your C source code.
    -   Call the Rust functions as if they were C functions. 

        ```C
        // main.c
        #include <some_lib.h>
        #include <stdio.h>

        int main(void)
        {
            greet_from_rust();
            int sum = add_numbers(5, 7);
            printf("Sum from Rust: %d\n", sum);
            return 0;
        }
        ```

-   Compile and link the C code:
    -   Compile your C code and link it against the Rust library. 

        ```bash
        gcc main.c -L./some_lib/target/release -I. -lsome_lib -o app
        ```


-   Replace ./some_lib/target/release with the actual path to your Rust library.
-   Run the executable: `./app` 
