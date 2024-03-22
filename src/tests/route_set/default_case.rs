#[tokio::test]
pub async fn test_route_set() {
    let body = serde_json::json!({
        "documents": {
            "guili": {
                "firstName": "Guili",
                "lastName": "Gonçalves"
            },
            "nah": {
                "firstName": "Nayara",
                "lastName": "Gonçalves"
            },
            "john": {
                "firstName": "John",
                "lastName": "Doe"
            },
        }
    });
    let expected_body = serde_json::json!({"errors": {
        "guili": {
            "status_code": 200,
            "error": null
        },
        "nah": {
            "status_code": 200,
            "error": null
        },
        "john": {
            "status_code": 200,
            "error": null
        },
    }});

    let response_json = crate::tests::route_set::do_set_request(body).await;

    assert_eq!(response_json, expected_body);
}
