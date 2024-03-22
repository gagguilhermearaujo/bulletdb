mod default_case;
mod subpaths_case;

pub async fn do_set_request(body: serde_json::Value) -> serde_json::Value {
    let address = crate::spawn_test_app();
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/set", &address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let body: serde_json::Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    body
}
