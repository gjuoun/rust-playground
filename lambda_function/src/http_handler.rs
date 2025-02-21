use chrono;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde::Serialize;
use serde_json::json;

/// used for chron format
const ISO8086: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    // Create a JSON response
    let json_response = json!({
        "message": format!("Hello {who}, this is Jun Guo from lambda function"),
        "timestamp": chrono::Utc::now().format(ISO8086).to_string(),
    });

    // Return something that implements IntoResponse.
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&json_response)?.into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{Request, RequestExt};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = Request::default();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            format!("{{\"message\":\"Hello world, this is Jun Guo from lambda function\",\"timestamp\":\"{}\"}}", chrono::Utc::now().format(ISO8086).to_string())
        );
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "lambda_function".into());

        let request = Request::default().with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            format!("{{\"message\":\"Hello lambda_function, this is Jun Guo from lambda function\",\"timestamp\":\"{}\"}}", chrono::Utc::now().format(ISO8086).to_string())
        );
    }
}
