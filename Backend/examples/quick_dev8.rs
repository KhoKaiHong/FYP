use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    let req_login = hc.do_get("/api/events");
    req_login.await?.print().await?;

    let req_login = hc.do_get("/api/events/future");
    req_login.await?.print().await?;

    Ok(())
}