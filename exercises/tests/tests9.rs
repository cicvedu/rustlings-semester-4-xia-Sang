// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

// 使用 #[no_mangle] 保证函数名不会被修改，
// 这样外部代码可以通过这个名字调用该函数
#[no_mangle]
pub extern "C" fn my_demo_function(a: u32) -> u32 {
    Foo::my_demo_function(a)
}

// 使用 #[link_name] 将函数名与模块内的函数名关联起来
#[no_mangle]
#[link_name = "my_demo_function"]
pub extern "C" fn my_demo_function_alias(a: u32) -> u32 {
    Foo::my_demo_function(a)
}

mod Foo {
    // 没有 `extern` 等于 `extern "Rust"`
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 外部导入的函数默认是 UNSAFE 的，
        // 因为它们可能来自其他不受信任的语言。
        // 你可以将它们包装在安全的 Rust API 中，
        // 以减轻调用者的负担。
        //
        // SAFETY: 我们知道这些函数是安全 Rust 函数的别名。
        unsafe {
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456);
        }
    }
}

// extern "Rust" {
//     fn my_demo_function(a: u32) -> u32;
//     fn my_demo_function_alias(a: u32) -> u32;
// }

// mod Foo {
//     // No `extern` equals `extern "Rust"`.
//     fn my_demo_function(a: u32) -> u32 {
//         a
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_success() {
//         // The externally imported functions are UNSAFE by default
//         // because of untrusted source of other languages. You may
//         // wrap them in safe Rust APIs to ease the burden of callers.
//         //
//         // SAFETY: We know those functions are aliases of a safe
//         // Rust function.
//         unsafe {
//             my_demo_function(123);
//             my_demo_function_alias(456);
//         }
//     }
// }
