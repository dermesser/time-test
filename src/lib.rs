use std::io::{self, Write};

extern crate time;

/// TestTimer allows to easily time tests. It's recommended to use the time_test!() macro.
pub struct TestTimer(time::Timespec);

impl TestTimer {
    pub fn new() -> TestTimer {
        TestTimer(time::get_time())
    }
}

impl Drop for TestTimer {
    fn drop(&mut self) {
        write!(io::stderr(), "(took {}) ", time::get_time() - self.0).is_ok();
    }
}

#[macro_export]
macro_rules! time_test {
    () => {
        use time_test::TestTimer;
        let _tt = TestTimer::new();
    }
}
