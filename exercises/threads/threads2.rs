// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{ Arc, Mutex, MutexGuard };
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // RAII version that disables once out of scope
            let mut locked_status: MutexGuard<'_, _> = status_shared.lock().unwrap();
            locked_status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // Each handle runs first, before they all join
        let locked_status = status.lock().unwrap();
        println!("jobs completed {}", locked_status.jobs_completed);
        // POC: We only need to join the first, since all wait to take mutex before loop is done
        // which only then we can process
        return ()
    }
}
