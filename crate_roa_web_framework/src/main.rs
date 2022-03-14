use roa::core::App;
use roa::preload::*;
use std::error::Error as StdError;

#[async_std::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let mut app = App::new(());
    app.end(|mut ctx| async move { ctx.write_text("Hello, World").await });
    app.listen("127.0.0.1:8100", |addr| {
        println!("Server is listening on {}", addr)
    })?
    .await?;
    Ok(())
}
