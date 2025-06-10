mod hello_test;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    hello_test::hello();
    Ok(())
}
