# `time_test`

`time_test` is a super simple crate that you can use to easily obtain how long
your tests took to run. Using it is simple (see `example/` for an example use
case):

```rust
#[macro_use]
extern crate time_test;

#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {
        time_test!();

        println!("hello world");
        assert!(true);
        assert_eq!(1, 1);
    }
}
```

Adding the `time_test!()` macro to the line in your test from which you want to
measure the test duration will result in the duration that the test has taken
being shown in the test result line:

```
$ # 1 test thread so that the output is not garbled.
$ cargo test -- --test-threads=1

    Finished dev [unoptimized + debuginfo] target(s) in 0.78 secs
     Running target/debug/deps/example-a84426a5de188514

running 1 test
test example::tests::my_test ... (took PT0.000004178S) ok
```

