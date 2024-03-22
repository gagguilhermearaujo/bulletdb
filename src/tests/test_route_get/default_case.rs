#[tokio::test]
async fn test_route_get() {
    crate::tests::test_route_set::do_set_request(serde_json::json!({
        "documents": {
            "test_get_1": {
                "firstName": "Guili",
                "lastName": "Test 1"
            },
            "test_get_2": {
                "firstName": "Guili",
                "lastName": "Test 2"
            },
        }
    }))
    .await;

    let address = crate::spawn_test_app();

    // Prepare a test JSON request
    let request_body = serde_json::json!({
        "documents": {
            "test_get_1": {},
            "test_get_2": {}
        }
    });

    // Send a request to the route
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/get", &address))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert the response status
    assert!(response.status().is_success());

    // Assert the response body
    let body: serde_json::Value =
        serde_json::from_str(&response.text().await.expect("Failed to read response body"))
            .unwrap();
    let expected_body = serde_json::json!({
        "documents": {
            "test_get_1": {
                "data": {
                    "firstName": "Guili",
                    "lastName": "Test 1"
                },
                "status_code": 200,
                "error": null
            },
            "test_get_2": {
                "data": {
                    "firstName": "Guili",
                    "lastName": "Test 2"
                },
                "status_code": 200,
                "error": null
            }
        }
    });
    assert_eq!(body, expected_body);
}
