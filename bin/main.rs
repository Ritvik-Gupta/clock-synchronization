use std::sync::atomic::{AtomicPtr, Ordering};

#[cfg(feature = "dhat-profile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug)]
#[allow(dead_code)]
struct Data {
    a: i32,
    b: bool,
    c: String,
}

fn main() {
    #[cfg(feature = "dhat-profile")]
    let _profiler = dhat::Profiler::new_heap();

    let mem = Box::leak(Box::new(Data {
        a: -1,
        b: true,
        c: "Yes".to_string(),
    }));
    let main_store = AtomicPtr::new(mem);
    println!("{:?}", unsafe { &*main_store.load(Ordering::SeqCst) });

    let mem2 = Box::leak(Box::new(Data {
        a: 1,
        b: false,
        c: "No".to_string(),
    }));
    main_store
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |memory| {
            unsafe { Box::from_raw(memory) };
            Some(mem2)
        })
        .ok();
    println!("{:?}", unsafe { &*main_store.load(Ordering::SeqCst) });

    println!("{:?}", mem);
    println!("{:?}", mem2);
}
