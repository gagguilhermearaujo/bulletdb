#[tokio::test]
async fn test_route_set_with_subpaths() {
    let address = crate::spawn_test_app();

    // Prepare a test JSON request
    let request_body = serde_json::json!({
        "documents": {
            "path1/subpath1": {
                "key1": "value1",
                "key2": "value2"
            },
            "path2/subpath1/subpath2": {
                "key3": "value3",
                "key4": "value4"
            }
        }
    });

    // Send a request to the route
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/set", &address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert the response status
    assert!(response.status().is_success());

    // Assert the response body
    let body = response.text().await.expect("Failed to read response body");
    let expected_body = r#"{"errors":{"path1/subpath1":{"status_code":200,"error":null},"path2/subpath1/subpath2":{"status_code":200,"error":null}}}"#;
    assert_eq!(body, expected_body);
}
