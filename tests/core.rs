extern crate macro_py;

#[cfg(test)]
mod tests {
    use macro_py::py;

    #[test]
    fn test() {
        py!(
            def f() {} 
        );
    }
}