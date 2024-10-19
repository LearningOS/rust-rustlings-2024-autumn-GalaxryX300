// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}

// 实现 trait `AppendBar` 为 `Vec<String>`
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));  // 修改：将 "Bar" 附加到向量的末尾
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));  // 修改：检查向量的最后一个元素是否为 "Bar"
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));  // 修改：检查向量的倒数第二个元素是否为 "Foo"
    }
}
