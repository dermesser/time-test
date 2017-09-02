use std::io::{self, Write};

extern crate time;

/// TestTimer allows to easily time tests. It's recommended to use the time_test!() macro.
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
            write!(io::stderr(), "(took {}) ", dur).is_ok();
        } else {
            write!(io::stderr(), "({} took {}) ", self.1, dur).is_ok();
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
