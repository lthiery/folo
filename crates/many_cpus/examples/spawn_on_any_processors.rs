//! Spawns one thread for each processor in the default processor set but allows the OS to decide
//! which processor each thread runs on. This can be used to observe how the OS schedules threads
//! across processors when not provided any constraints.
//!
//! Each thread will do a bit of work (10 seconds of spinning CPU) and then terminate.

use std::time::Instant;

use many_cpus::ProcessorSet;

fn main() {
    // We spawn N threads, where N is the number of processors.
    // However, we do not pin them to any specific processor.
    // This means that the OS can schedule them however it likes.

    let processor_set = ProcessorSet::default();

    let mut threads = Vec::with_capacity(processor_set.len());

    for _ in 0..processor_set.len() {
        let thread = std::thread::spawn(move || {
            let start = Instant::now();

            let mut x: u64 = 0;

            loop {
                for _ in 0..100_000 {
                    x = x.wrapping_add(1);
                }

                // Every thread spins the CPU for 10 seconds.
                if start.elapsed().as_secs() > 10 {
                    println!("Thread finished after {x} iterations");
                    break;
                }
            }
        });

        threads.push(thread);
    }

    println!("Spawned {} threads", threads.len());

    for thread in threads {
        thread.join().unwrap();
    }

    println!("All threads have finished.");
}
