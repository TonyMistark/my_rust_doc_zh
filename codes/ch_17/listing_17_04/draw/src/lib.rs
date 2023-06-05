pub trait Draw {
    fn draw (&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
