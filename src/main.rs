use bytes::BytesMut;
use jemallocator;
use tokio::{runtime::Builder, sync::oneshot};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let mut runtime = Builder::new()
        .threaded_scheduler()
        .num_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let (tx, rx) = oneshot::channel::<()>();

    runtime.spawn(async move {
        let buf = BytesMut::with_capacity(1);
        buf.freeze();
        tx.send(())
    });

    runtime.block_on(rx).unwrap();
}
