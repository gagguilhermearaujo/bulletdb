use actix_web::{post, web, HttpResponse, Responder};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{self, io::Write, os::unix::fs::OpenOptionsExt, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct SetRequest {
    documents: IndexMap<String, serde_json::Value>,
    is_transaction: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct SetResponse {
    errors: IndexMap<String, DocumentInSetResponse>,
}

#[derive(Serialize, Deserialize)]
struct DocumentInSetResponse {
    status_code: u16,
    error: Option<String>,
}

#[post("/set")]
async fn set(request: web::Json<SetRequest>) -> impl Responder {
    let mut set_response = SetResponse {
        errors: IndexMap::new(),
    };

    for (doc_path, doc_data) in &request.documents {
        let mut path_buf = PathBuf::from(doc_path);
        let _ = path_buf.pop();
        _ = std::fs::create_dir_all(format!("data/{}", path_buf.to_string_lossy()));

        let file = std::fs::File::options()
            .create(true)
            .mode(0o600)
            .write(true)
            .truncate(true)
            .open(format!("data/{}.json", doc_path))
            .expect("Failed to open file for writing");

        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer(&mut writer, &doc_data).expect("Failed to write to file");
        _ = writer.flush();

        set_response.errors.insert(
            doc_path.to_string(),
            DocumentInSetResponse {
                status_code: actix_web::http::StatusCode::OK.as_u16(),
                error: None,
            },
        );
    }
    HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::json())
        .status(actix_web::http::StatusCode::MULTI_STATUS)
        .body(serde_json::to_string(&set_response).expect("Failed to serialize response"))
}
