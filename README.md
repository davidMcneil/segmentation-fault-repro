# Segmentation Fault Repro

Run `cargo run` and you should get a `Segmentation fault`.

I am not sure if this error is occurring in `bytes`, `tokio`, or `jemalloc`. If you remove `jemalloc` as the default allocator the program runs fine. If you switch to `tokio`'s `basic_schedular` the program also runs fine.
