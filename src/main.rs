mod bootstrap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::init()?;

    Ok(())
}
