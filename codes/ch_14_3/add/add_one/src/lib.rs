use rand;

pub fn add_one(x: i32) -> i32 {
    x +1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn five_add_one_is_six() {
        let six = add_one(5);
        assert_eq!(six, 6);
    }
}
