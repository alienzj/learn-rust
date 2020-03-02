pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn greeting_hello(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!("hello world", "hello".to_owned() + " world");
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn add_two_test() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting dit not contain name, values was `{}`",
            result
        );
    }

    #[test]
    fn greeting_no_name() {
        let result = greeting_hello("hello");
        assert!(result.contains("Hello"));
    }
}

//#[cfg(test)]
#[test]
fn hello_test() {
    assert_ne!(1, 2);
}
