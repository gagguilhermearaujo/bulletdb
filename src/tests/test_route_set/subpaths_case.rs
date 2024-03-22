#[tokio::test]
async fn test_route_set_with_subpaths() {
    let body = serde_json::json!({
        "documents": {
            "users/guili/subpath3/subpath4": {
                "firstName": "Guili",
                "lastName": "Gonçalves"
            },
            "cars/corolla": {
                "color": "Silver"
            }
        }
    });
    let expected_body = serde_json::json!({"errors": {
        "users/guili/subpath3/subpath4": {
            "status_code": 200,
            "error": null
        },
        "cars/corolla": {
            "status_code": 200,
            "error": null
        }
    }});

    let response_json = crate::tests::test_route_set::do_set_request(body).await;

    assert_eq!(response_json, expected_body);
}
