fn main() {
    println!("{}", hello());
}

fn hello() -> String {
    "Hello, world".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello();
        assert_eq!(result, "Hello, world");
    }
}
