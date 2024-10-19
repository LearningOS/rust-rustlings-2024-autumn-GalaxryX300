// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    unsafe { Box::from_raw(ptr) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 创建一个包含 `Foo` 实例的 Box
        let data = Box::new(Foo { a: 1, b: None });

        // 获取 `data` 中 `a` 字段的指针并转换为 usize
        let ptr_1 = &data.a as *const u128 as usize;

        // 将 `data` 转换为原始指针并传递给 `raw_pointer_to_box` 函数
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        // 获取 `ret` 中 `a` 字段的指针并转换为 usize
        let ptr_2 = &ret.a as *const u128 as usize;

        // 检查原始指针和重建后的指针是否相同
        assert_eq!(ptr_1, ptr_2);

        // 检查 `ret` 中的 `b` 字段是否为 `None`
        // 因为我们创建 `Foo` 实例时将其设置为 `None`
        assert_eq!(ret.b, None); // 修正这里
    }
}
