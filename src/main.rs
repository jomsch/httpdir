use async_std::prelude::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(|r| async { Ok("Hello World") });
    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
