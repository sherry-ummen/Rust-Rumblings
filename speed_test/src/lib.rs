use std::thread;
#[no_mangle]
pub extern fn multi() {
    //let sw = Stopwatch::start_new();
    let handles: Vec<_> = (0..1000)
        .map(|_| {
            thread::spawn(|| {
                let mut x = 0;
                for _ in 0..5_000_000 {
                    x += 1;
                }
                x
            })
        })
        .collect();

    for h in handles {
//        println!("Thread finished with count={}",
                 h.join().map_err(|_| "Could not join a thread!").unwrap();
    }
    //println!("done in {}ms", sw.elapsed_ms());
}