//! `time_test` is a super simple crate that you can use to easily obtain how long
//! your tests took to run. Using it is simple (see `example/` for an example use
//! case):
//!
//! ```
//! #[macro_use] extern crate time_test;
//!
//! #[cfg(test)]
//! mod tests {
//!     #[test]
//!     fn my_test() {
//!         time_test!();
//!
//!         println!("hello world");
//!
//!         {
//!             time_test!("sub-assert 1");
//!             assert!(true);
//!         }
//!         assert_eq!(1, 1);
//!     }
//! }
//! ```
//!
//! Adding the `time_test!()` macro to the line in your test from which you want to
//! measure the test duration will result in the duration that the test has taken
//! being shown in the test result line:
//!
//! ```bash
//! $ # 1 test thread so that the output is not garbled.
//! $ cargo test -- --test-threads=1
//!
//!     Finished dev [unoptimized + debuginfo] target(s) in 0.78 secs
//!      Running target/debug/deps/example-a84426a5de188514
//!
//! running 1 test
//! test example::tests::my_test ... (sub-assert 1 took PT0.000002421S) (took PT0.000004178S) ok
//! ```


use std::io::{self, Write};

use time;

/// TestTimer allows to easily time tests. It's recommended to use the time_test!() macro instead
/// of invoking this type directly.
pub struct TestTimer(time::Timespec, String);

impl TestTimer {
    pub fn new(desc: String) -> TestTimer {
        TestTimer(time::get_time(), desc)
    }
}

impl Drop for TestTimer {
    fn drop(&mut self) {
        let dur = time::get_time() - self.0;
        if self.1.is_empty() {
            write!(io::stderr(), "(took {}) ", dur).unwrap_or(());
        } else {
            write!(io::stderr(), "({} took {}) ", self.1, dur).unwrap_or(());
        }
    }
}

#[macro_export]
macro_rules! time_test {
    () => {
        use time_test::TestTimer;
        let _tt = TestTimer::new(String::new());
    };
    ($desc:expr) => {
        use time_test::TestTimer;
        let _tt = TestTimer::new($desc.to_string());
    }
}
