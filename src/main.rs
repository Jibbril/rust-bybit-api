use dotenv::dotenv;

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("Hello World!");

    Ok(())
}
