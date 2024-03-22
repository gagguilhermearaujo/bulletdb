use actix_web::{post, web, HttpResponse, Responder};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std;

#[derive(Serialize, Deserialize)]
struct GetRequest {
    documents: IndexMap<String, RequestOptions>,
    fields_to_include: Option<IndexMap<String, bool>>,
    fields_to_exclude: Option<IndexMap<String, bool>>,
    global_request_options: Option<RequestOptions>,
}

#[derive(Serialize, Deserialize)]

struct RequestOptions {
    order_by: Option<Vec<String>>,
    fields_to_include: Option<Vec<String>>,
    fields_to_exclude: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct GetResponse {
    documents: IndexMap<String, DocumentInGetResponse>,
}

#[derive(Serialize, Deserialize)]
struct DocumentInGetResponse {
    data: Option<Value>,
    status_code: u16,
    error: Option<String>,
}

#[post("/get")]
async fn get(request: web::Json<GetRequest>) -> impl Responder {
    let mut get_response = GetResponse {
        documents: IndexMap::new(),
    };

    for (doc_path, _get_options) in &request.documents {
        match std::fs::File::open(format!("data/{}.json", doc_path)) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                let parsed_data: serde_json::Value = serde_json::from_reader(reader).unwrap();
                get_response.documents.insert(
                    doc_path.to_string(),
                    DocumentInGetResponse {
                        data: Some(parsed_data),
                        status_code: actix_web::http::StatusCode::OK.as_u16(),
                        error: None,
                    },
                )
            }
            Err(_) => get_response.documents.insert(
                doc_path.to_string(),
                DocumentInGetResponse {
                    data: None,
                    status_code: actix_web::http::StatusCode::NOT_FOUND.as_u16(),
                    error: Some("Document with provided path does not exist".to_string()),
                },
            ),
        };
    }
    HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::json())
        .status(actix_web::http::StatusCode::MULTI_STATUS)
        .body(serde_json::to_string(&get_response).unwrap())
}
