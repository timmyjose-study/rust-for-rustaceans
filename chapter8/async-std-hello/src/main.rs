use async_std::task;

async fn say_hello() {
    println!("Hello from async_std!");
}

fn main() {
    let op = say_hello();
    println!("Main says hi!");
    task::block_on(op);
}
