# Segmentation Fault Repro

This was discussed [here](https://www.reddit.com/r/rust/comments/e9nilt/segfault_when_using_bytes_tokio_and_jemalloc_who/) and resolved with [this ticket](https://github.com/tokio-rs/bytes/issues/340). 

Run `cargo run` and you should get a `Segmentation fault`.

I am not sure if this error is occurring in `bytes`, `tokio`, or `jemalloc`. If you remove `jemalloc` as the default allocator the program runs fine. If you switch to `tokio`'s `basic_schedular` the program also runs fine.
