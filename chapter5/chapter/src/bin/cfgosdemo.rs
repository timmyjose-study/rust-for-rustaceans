fn main() {
    if cfg!(target_os = "windows") {
        println!("Working on Windows");
    } else if cfg!(target_os = "macos") {
        println!("Working on macOS");
    } else if cfg!(target_os = "unix") {
        println!("Working on Linux");
    } else if cfg!(target_os = "linux") {
        println!("Working on linux");
    } else {
        println!("Working on somethign other OS");
    }
}