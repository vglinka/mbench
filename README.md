## Microbenchmarks macro

Measures the average time for a given number of iterations.
Easy to use, zero deps.

**Cargo.toml**

```toml
[dependencies]
mbench = "0.1"
```

**main.rs**

```rust
#[macro_use]
extern crate mbench;

fn my_func(x: u32) { vec![x; u16::MAX as usize].sort(); }

fn main() {
    let mut text = String::new();
    microbench!(
        text = "Hello world!".to_string();
        my_func(42);
        // ... code ....
    );
    println!("{text}");
}
```

**Output**

```sh
Elapsed / 10 = 3.70ms
Hello world!
```

The default is 10 iterations, but you can set an arbitrary number `microbench!(4, {/* code */});`.

```rust
microbench!(1, {
    text = "Hello world!".to_string();
    my_func(42);
});
```

```sh
Elapsed / 1 = 3.34ms
Hello world
```

In addition, there is a simplified syntax for setting the number of
iterations (1,5,10,50,100,500,1000):

```rust
microbench1!(/* code */);
microbench5!(/* code */);
microbench10!(/* code */);
microbench50!(/* code */);
microbench100!(/* code */);
microbench500!(/* code */);
microbench1000!(/* code */);
```

With macro `fixedbench!([0..=2], 10, {/* code */});` you can set
the range in milliseconds (and optionally 10 iterations in this example).
If the code runs longer or faster than the interval specified,
then there will be a panic.

This can be used in tests. The test fails if the code
becomes slower after the change. The default is 1 iteration.

```rust
#[macro_use]
extern crate mbench;

fn my_func(ms: u64) {
    let ms = std::time::Duration::from_millis(ms);
    std::thread::sleep(ms);
}

#[cfg(test)]
mod tests {
    use super::my_func;

    #[test]
    fn simple() {
        fixedbench!([..=2], {
            my_func(3);
        });
    }
}
```

```sh
running 1 test
test tests::simple ... FAILED

failures:

---- tests::simple stdout ----
Elapsed / 1 = 3.36ms
thread 'tests::simple' panicked at 'expected [..=2]ms, finished in 3ms.', main.rs:15:9
```

Keep in mind that the running time of the code may vary on different
machines. After completing work on a piece of code, such a test
should be removed or commented out.
