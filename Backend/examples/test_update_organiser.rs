use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    let req_login = hc.do_post(
        "/api/organiser-login",
        json!({
            "email": "organiser1@example.com",
            "password": "password123"
        }),
    );

    let res = req_login.await?;
    let json = res.json_body()?;
    let access_token = json["data"]["accessToken"].as_str().unwrap();

    let req = hc
        .reqwest_client()
        .patch("http://localhost:8000/api/organiser")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "currentPassword": "password123",
            "password": "password1234",
            "name": "New org name",
            "email": "neworg@yahoo.com",
            "phoneNumber": "+6018-9876524",
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
        .patch("http://localhost:8000/api/organiser")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "currentPassword": "password123",
            "password": "password1234",
            "name": "New org name",
            "email": "neworg@yahoo.com",
            "phoneNumber": "+6018-9876524",
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
        .patch("http://localhost:8000/api/organiser")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "currentPassword": "password123",
            "password": "password1234",
            "name": "New org name",
            "email": "neworg@yahoo.com",
            "phoneNumber": "+6018-9876524",
        }));

    let res = req.send().await?;
    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    Ok(())
}
