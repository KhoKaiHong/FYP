use std::collections::HashMap;

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    // User login test
    let user_req_login = hc.do_post(
        "/api/userlogin",
        json!({
            "icNumber": "900101-01-1234",
            "password": "password123"
        }),
    );

    let res = user_req_login.await?.json_body()?;
    let user_access_token = res["data"]["accessToken"].as_str().unwrap();
    let user_refresh_token = res["data"]["refreshToken"].as_str().unwrap();

    let hc = httpc_test::new_client("http://localhost:8000")?;

    // User login test
    let user_req_login2 = hc.do_post(
        "/api/userlogin",
        json!({
            "icNumber": "900101-01-1234",
            "password": "password123"
        }),
    );

    let res = user_req_login2.await?.json_body()?;
    let user_access_token2 = res["data"]["accessToken"].as_str().unwrap();
    let user_refresh_token2 = res["data"]["refreshToken"].as_str().unwrap();

    let mut map = HashMap::new();
    map.insert("refreshToken", user_refresh_token);

    let req = hc
        .reqwest_client()
        .post("http://localhost:8000/api/logout")
        .json(&map)
        .bearer_auth(user_access_token);

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let mut map = HashMap::new();
    map.insert("refreshToken", user_refresh_token2);

    let req = hc
        .reqwest_client()
        .post("http://localhost:8000/api/logoutall")
        .json(&map)
        .bearer_auth(user_access_token2);

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    // let req = hc
    //     .reqwest_client()
    //     .get("http://localhost:8000/api/hello2/John")
    //     .bearer_auth("gtfhtgrhgthntg");

    // let res = req.send().await?;

    // println!("{:?}", res);
    // println!("{:?}\n\n", res.text().await?);

    // let req_login = hc.do_get("/api/hello?name=John");
    // req_login.await?.print().await?;

    // let req_login = hc.do_get("/api/hello2/Mike");
    // req_login.await?.print().await?;

    // Login tests
    // let req_login = hc.do_post(
    //     "/api/userlogin",
    //     json!({
    //         "icNumber": "900101-01-1234",
    //         "password": "password123"
    //     }),
    // );
    // req_login.await?.print().await?;

    // let req_login = hc.do_post(
    //     "/api/userlogin",
    //     json!({
    //         "icNumber": "900101-01-1234",
    //         "password": "htrghtghn"
    //     }),
    // );
    // req_login.await?.print().await?;

    // // Facility login test
    // let req_login = hc.do_post(
    //     "/api/facilitylogin",
    //     json!({
    //         "email": "sultanah.aminah@example.com",
    //         "password": "password123"
    //     }),
    // );
    // req_login.await?.print().await?;

    // let req_login = hc.do_post(
    //     "/api/facilitylogin",
    //     json!({
    //         "email": "sultanah.aminah@example.com",
    //         "password": "welcome"
    //     }),
    // );
    // req_login.await?.print().await?;

    // // Organiser login test
    // let req_login = hc.do_post(
    //     "/api/organiserlogin",
    //     json!({
    //         "email": "organiser1@example.com",
    //         "password": "password123"
    //     }),
    // );
    // req_login.await?.print().await?;

    // let req_login = hc.do_post(
    //     "/api/organiserlogin",
    //     json!({
    //         "email": "jyhmnhjmhjmku",
    //         "password": "password123"
    //     }),
    // );
    // req_login.await?.print().await?;

    Ok(())
}
