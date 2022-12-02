#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = api::SquirePool::new().await?;

    api::serve(db).await
}
