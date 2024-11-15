use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    // let req_login = hc.do_get("/api/districts");
    // req_login.await?.print().await?;

    let req_login = hc.do_get("/api/states");
    req_login.await?.print().await?;

    let req_login = hc.do_get("/api/bloodtypes");
    req_login.await?.print().await?;

    Ok(())
}
