async fn say_hello() {
    println!("Hello from tokio");
}

#[tokio::main]
async fn main() {
    let op = say_hello();
    println!("Main says hi!");
    op.await;
}
