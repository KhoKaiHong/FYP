use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    // User login test
    let req_login = hc.do_post(
        "/api/user-login",
        json!({
            "icNumber": "900101-01-1234",
            "password": "password123"
        }),
    );

    let res = req_login.await?;
    let json = res.json_body()?;
    let access_token = json["data"]["accessToken"].as_str().unwrap();

    let req = hc
        .reqwest_client()
        .patch("http://localhost:8000/api/user")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "currentPassword": "password123",
            "password": "password1234",
            "stateId": 2,
            "districtId": 2,
        }));

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let req = hc
        .reqwest_client()
        .get("http://localhost:8000/api/get-credentials")
        .header("Accept", "text/html")
        .bearer_auth(access_token);

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let req = hc
        .reqwest_client()
        .patch("http://localhost:8000/api/user")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "currentPassword": "password123",
            "password": "password1234",
            "stateId": 2,
            "districtId": 2,
        }));

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    // Use facility to access endpoint
    let req_login = hc.do_post(
        "/api/facility-login",
        json!({
            "email": "sultanah.aminah@example.com",
            "password": "password123"
        }),
    );

    let res = req_login.await?;
    let json = res.json_body()?;
    let access_token = json["data"]["accessToken"].as_str().unwrap();

    let req = hc
        .reqwest_client()
        .patch("http://localhost:8000/api/user")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "currentPassword": "password123",
            "password": "password1234",
            "stateId": 2,
            "districtId": 2,
        }));

    let res = req.send().await?;
    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    Ok(())
}
