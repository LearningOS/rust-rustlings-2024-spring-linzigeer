// threads2.rs
//
// Building on the last exercise, we want all the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Default)]
struct JobStatus {
    pub jobs_completed: u32,
}

fn main() {
    let status = Arc::new(RwLock::new(JobStatus::default()));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut r = status_shared.write().unwrap();
            r.jobs_completed += 1;

        });
        handles.push(handle);
    }
    for handle in handles {
        let status_shared = status.clone();
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        let r = status_shared.read().unwrap();
        println!("jobs completed {}", r.jobs_completed);

    }
}
