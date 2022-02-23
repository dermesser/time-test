#[macro_use]
extern crate time_test;

fn my_test() {
    println!("");
    time_test!("my_test");
    assert!(true);
    assert_eq!(1, 1);
}

fn my_test_with_initialization() {
    println!("");
    // Do possibly expensive preparation before.
    let a = 1;
    let b = a + 2;
    let _ = b + 5;

    time_test!("my_test_with_initialization");

    assert!(true);
    assert_eq!(1, 1);
}

fn my_test_with_multiple_timers() {
    println!("");
    time_test!("root");

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

fn main() {
    my_test();
    my_test_with_initialization();
    my_test_with_multiple_timers();
    println!("");
}
