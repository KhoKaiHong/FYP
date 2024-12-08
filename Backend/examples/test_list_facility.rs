use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    let req = hc.do_get("/api/facilities");

    req.await?.print().await?;

    Ok(())
}
