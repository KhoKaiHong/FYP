use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    // User register test
    // let req_login = hc.do_post(
    //     "/api/user-login",
    //     json!({
    //         "icNumber": "031120-07-0559",
    //         "password": "testpassword",
    //         "name": "Khp Kai Hong",
    //         "email": "khokaihong@gmail.com",
    //         "phoneNumber": "+6011-35275289",
    //         "bloodType": "A",
    //         "stateId": 1,
    //         "districtId": 1,
    //     }),
    // );

    // let res = req_login.await?.json_body()?;
    // let token = res["data"]["accessToken"].as_str().unwrap();

    // let req = hc
    //     .reqwest_client()
    //     .get("http://localhost:8000/api/hello?name=John")
    //     .bearer_auth(token);

    // let res = req.send().await?;

    // println!("{:?}", res);
    // println!("{:?}\n\n", res.text().await?);

    // let req = hc
    //     .reqwest_client()
    //     .get("http://localhost:8000/api/hello2/John");

    // let res = req.send().await?;

    // println!("{:?}", res);
    // println!("{:?}\n\n", res.text().await?);

    // User Register tests
    let req_register = hc.do_post(
        "/api/user-register",
        json!({
            "icNumber": "031120-07-0559",
            "password": "testpassword",
            "name": "Kho Kai Hong",
            "email": "khokaihong@gmail.com",
            "phoneNumber": "+6011-35275289",
            "bloodType": "A+",
            "stateId": 1,
            "districtId": 1,
        }),
    );
    req_register.await?.print().await?;

    let req_register = hc.do_post(
        "/api/user-register",
        json!({
            "icNumber": "0311fe20-07-0559",
            "password": "testpassword",
            "name": "Test User",
            "email": "khokaihongg@gmail.com",
            "phoneNumber": "+6011g-35275289",
            "bloodType": "A",
            "stateId": 1,
            "districtId": 1,
        }),
    );
    req_register.await?.print().await?;

    let req_login = hc.do_post(
        "/api/user-login",
        json!({
            "icNumber": "031120-07-0559",
            "password": "testpassword"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/user-login",
        json!({
            "icNumber": "031120-07-0559",
            "password": "htrhbgthtr"
        }),
    );
    req_login.await?.print().await?;

    // Facility register test
    let req_register = hc.do_post(
        "/api/facility-register",
        json!({
            "email": "testfacility@hotmail.com",
            "password": "testpassword",
            "name": "Test Facility",
            "address": "test facility location",
            "phoneNumber": "+604-91837484",
            "stateId": 1,
        }),
    );
    req_register.await?.print().await?;

    let req_register = hc.do_post(
        "/api/facility-register",
        json!({
            "email": "testfacility2@hotmail.com",
            "password": "testpassword2",
            "name": "Test2 Facility",
            "address": "test facility location",
            "phoneNumber": "+604-91837484",
            "stateId": 50,
        }),
    );
    req_register.await?.print().await?;

    let req_login = hc.do_post(
        "/api/facility-login",
        json!({
            "email": "testfacility@hotmail.com",
            "password": "testpassword"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/facility-login",
        json!({
            "email": "testfacility@hotmail.com",
            "password": "htryhtrdghftg"
        }),
    );
    req_login.await?.print().await?;

    // Organiser register test
    let req_register = hc.do_post(
        "/api/organiser-register",
        json!({
            "email": "testorganiser@hotmail.com",
            "password": "testpassword",
            "name": "Test Organiser",
            "phoneNumber": "+604-91837484",
        }),
    );
    req_register.await?.print().await?;

    let req_register = hc.do_post(
        "/api/organiser-register",
        json!({
            "email": "testorganiser2@hotmail.com",
            "password": "testpassword",
            "name": "Test Organiser 2",
            "phoneNumber": "+604-91837484",
        }),
    );
    req_register.await?.print().await?;

    let req_login = hc.do_post(
        "/api/organiser-login",
        json!({
            "email": "testorganiser@hotmail.com",
            "password": "testpassword"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/organiser-login",
        json!({
            "email": "testorganiser2@hotmail.com",
            "password": "testpassword"
        }),
    );
    req_login.await?.print().await?;

    // Admin register test
    let req_register = hc.do_post(
        "/api/admin-register",
        json!({
            "email": "testadmin@hotmail.com",
            "password": "testpassword",
            "name": "Test Admin",
        }),
    );
    req_register.await?.print().await?;

    let req_register = hc.do_post(
        "/api/admin-register",
        json!({
            "email": "testadmin@hotmail.com",
            "password": "testpassword",
            "name": "Test Admin 2",
        }),
    );
    req_register.await?.print().await?;

    let req_login = hc.do_post(
        "/api/admin-login",
        json!({
            "email": "testadmin@hotmail.com",
            "password": "testpassword"
        }),
    );
    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/admin-login",
        json!({
            "email": "testadmin2@hotmail.com",
            "password": "testpassword"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
