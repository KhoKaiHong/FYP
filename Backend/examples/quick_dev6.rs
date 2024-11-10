use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    // User login test
    let req_login = hc.do_post(
        "/api/userlogin",
        json!({
            "icNumber": "900101-01-1234",
            "password": "password123"
        }),
    );

    let res = req_login.await?;
    res.print().await?;
    let json = res.json_body()?;
    let access_token = json["data"]["accessToken"].as_str().unwrap();

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/getcredentials")
        .header("Accept", "text/html")
        .bearer_auth(access_token);
    println!("{:?}", req);

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/getcredentials")
        .bearer_auth("figrniughjnrfi");
    println!("{:?}", req);
    println!();

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    // Facility login test
    let req_login = hc.do_post(
        "/api/facilitylogin",
        json!({
            "email": "sultanah.aminah@example.com",
            "password": "password123"
        }),
    );
    let res = req_login.await?.json_body()?;
    let access_token = res["data"]["accessToken"].as_str().unwrap();

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/getcredentials")
        .bearer_auth(access_token);

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/getcredentials")
        .bearer_auth("figrniughjnrfi");

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    // Organiser login test
    let req_login = hc.do_post(
        "/api/organiserlogin",
        json!({
            "email": "organiser1@example.com",
            "password": "password123"
        }),
    );
    let res = req_login.await?.json_body()?;
    let access_token = res["data"]["accessToken"].as_str().unwrap();

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/getcredentials")
        .bearer_auth(access_token);

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/getcredentials")
        .bearer_auth("figrniughjnrfi");

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    // Admin login test
    let req_login = hc.do_post(
        "/api/adminlogin",
        json!({
            "email": "admin1@example.com",
            "password": "password123"
        }),
    );
    let res = req_login.await?.json_body()?;
    let access_token = res["data"]["accessToken"].as_str().unwrap();

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/getcredentials")
        .bearer_auth(access_token);

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let req = hc
        .reqwest_client()
        .get("http://localhost:3001/api/getcredentials")
        .bearer_auth("figrniughjnrfi");

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    Ok(())
}
