use hyper::{header, Body, Request, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
//Simply takes a serde_json Value and creates a a Hyper response from it.
pub fn json_response(json: Value) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json.to_string()))
        .expect("Unable to create response.")
}

//Takes a Request and parses it into a struct or JsonValue.
pub async fn parse_body<T: DeserializeOwned>(req: &mut Request<Body>) -> Result<T, String> {
    let body = hyper::body::to_bytes(req.body_mut())
        .await
        .map_err(|_| "Internal Server Error".to_string())?;
    serde_json::from_slice(&body).map_err(|e| format!("Failed to parse JSON: {}", e))
}
