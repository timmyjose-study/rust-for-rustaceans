use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::Builder::new()
        .name("my stonking thread".to_string())
        .spawn(|| {
            thread::sleep(Duration::from_millis(5000));
            if let Some(name) = thread::current().name() {
                println!("Hello from {:?}", name);
            } else {
                println!("Hello from generic thread");
            }
        });

    if let Ok(handle) = handle {
        handle.join().unwrap();
    }
    println!("Adios from main!");
}
