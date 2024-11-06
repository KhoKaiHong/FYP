use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    // User login test
    let req_login = hc.do_post(
        "/api/userlogin",
        json!({
            "ic_number": "900101-01-1234",
            "password": "password123"
        }),
    );

    let res = req_login.await?.json_body()?;
    let token = res["result"]["access_token"].as_str().unwrap();

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/hello?name=John")
        .bearer_auth(token);

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/hello2/John");

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    // Login tests
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
            "ic_number": "900101-01-1234",
            "password": "htrghtghn"
        }),
    );
    req_login.await?.print().await?;

    // Facility login test
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
            "email": "sultanah.aminah@example.com",
            "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // Organiser login test
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
            "email": "jyhmnhjmhjmku",
            "password": "password123"
        }),
    );
    req_login.await?.print().await?;

    // Admin login test
    let req_login = hc.do_post(
        "/api/adminlogin",
        json!({
            "email": "admin1@example.com",
            "password": "password123"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/adminlogin",
        json!({
            "email": "admin1@example.com",
            "password": "passwhtrhngtord123"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
