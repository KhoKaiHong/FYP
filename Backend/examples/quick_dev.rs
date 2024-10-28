use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/api/userlogin",
        json!({
            "ic_number": "900101-01-1234",
            "password": "password123"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/userlogin",
        json!({
            "ic_number": "ucbjsdncs",
            "password": "password123"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/facilitylogin",
        json!({
            "email": "sultanah.aminah@example.com",
            "password": "password123"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/facilitylogin",
        json!({
            "email": "grfgbvrfgbvre",
            "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/organiserlogin",
        json!({
            "email": "organiser1@example.com",
            "password": "password123"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/organiserlogin",
        json!({
            "email": "organiser1@example.com",
            "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    hc.do_get("/index.html").await?.print().await?;

    Ok(())
}
