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
        .post("http://localhost:8000/api/new-event-request")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "location": "Mid Valley",
            "address": "123 Main Street, Springfield",
            "startTime": "2025-03-05T17:00:00.000Z",
            "endTime": "2025-03-06T01:00:00.000Z",
            "maxAttendees": 250,
            "latitude": 3.1100465061202556,
            "longitude": 101.68386687698307,
            "facilityId": 2,
            "stateId": 1,
            "districtId": 4
        }));

    let res = req.send().await?;

    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    let req = hc
        .reqwest_client()
        .post("http://localhost:8000/api/new-event-request")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "location": "Mid Valley",
            "address": "123 Main Street, Springfield",
            "startTime": "2025-03-05T17:00:00.000Z",
            "endTime": "2025-03-06T01:00:00.000Z",
            "maxAttendees": 250,
            "latitude": 3.1100465061202556,
            "longitude": 101.68386687698307,
            "facilityId": 2,
            "stateId": 1,
            "districtId": 4
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
        .post("http://localhost:8000/api/new-event-request")
        .header("Accept", "text/html")
        .bearer_auth(access_token)
        .json(&json!({
            "location": "Mid Valley",
            "address": "123 Main Street, Springfield",
            "startTime": "2025-03-05T17:00:00.000Z",
            "endTime": "2025-03-06T01:00:00.000Z",
            "maxAttendees": 250,
            "latitude": 3.1100465061202556,
            "longitude": 101.68386687698307,
            "facilityId": 2,
            "stateId": 1,
            "districtId": 4
        }));

    let res = req.send().await?;
    println!("{:?}", res);
    println!("{:?}\n\n", res.text().await?);

    Ok(())
}
