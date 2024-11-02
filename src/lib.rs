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

use std::time::Instant;

/// TestTimer allows to easily time tests. It's recommended to use the time_test!() macro instead
/// of invoking this type directly.
pub struct TestTimer(Instant, String);

impl TestTimer {
    pub fn new(desc: String) -> TestTimer {
        TestTimer(Instant::now(), desc)
    }
}

impl Drop for TestTimer {
    fn drop(&mut self) {
        let dur = Instant::now()
            .checked_duration_since(self.0)
            .unwrap_or_default();
        let millis = (dur.as_nanos() as f64) / 1_000_000.0;
        if self.1.is_empty() {
            write!(io::stderr(), "(took {} ms) ", millis).unwrap_or(());
        } else {
            write!(io::stderr(), "({} took {} ms) ", self.1, millis).unwrap_or(());
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
    };
}

#[cfg(test)]
mod tests {
    use crate as time_test;
    #[test]
    fn test_time_test() {
        time_test!();
        assert!(true);
    }
}
