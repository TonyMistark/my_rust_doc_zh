#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn every_times_loop_sleep_1_second() {
        for i in 1..10 {
            println!("iteration: {}", i);
            std::thread::sleep(std::time::Duration::from_secs(1))
        }
    }

    #[test]
    #[ignore]
    fn every_times_loop_sleep_10_second() {
        for i in 1..5 {
            println!("sleep 10 second iteration: {}", i);
            std::thread::sleep(std::time::Duration::from_secs(10))
        }
    }
}
