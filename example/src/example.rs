
#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {
        time_test!();
        assert!(true);
        assert_eq!(1, 1);
    }

    #[test]
    fn my_test_with_initialization() {
        // Do possibly expensive preparation before.
        let a = 1;
        let b = 2;

        time_test!();

        assert!(true);
        assert_eq!(1, 1);
    }

    #[test]
    fn my_test_with_multiple_timers() {
        time_test!();

        {
            time_test!("all");
            {
                // sub-test 1
                time_test!("sub1");
                assert_eq!(1, 1);
            }

            {
                // sub-test 2
                time_test!("sub2");
                assert!(true);
            }
        }

        // The order of timers will be sub1, sub2, all, <unnamed>.
    }
}
