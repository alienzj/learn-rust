pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn greeting_hello(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn it_works_v2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
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

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    //#[test]
    //fn greater_than_100() {
    //    Guess::new(200);
    //}
}

//#[cfg(test)]
#[test]
fn hello_test() {
    assert_ne!(1, 2);
}
