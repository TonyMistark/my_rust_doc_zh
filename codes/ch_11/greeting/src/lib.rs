// pub fn greeting(name: &str) -> String {
//     format!("Hello {}!", name)
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(result.contains("Carol"));
//     }
// 
//     #[test]
//     fn greeting_contains_name2() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol")
//             "Greeting did not contain name, value was `{}`",
//             result
//             );
//     }
// }

pub fn greeting(name: &str) -> String {
    // this line changed
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

