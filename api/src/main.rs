use actix_web::{get, post, web, Responder, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Clone)]
struct MyResult {
    name: String,
    method: String,
    filter: Option<String>,
    columns: Option<String>,
    rows: Option<String>,
    format: Option<String>,
}

#[derive(Deserialize)]
struct UrlParams {
    filter: Option<String>,
    columns: Option<String>,
    rows: Option<String>,
    format: Option<String>,
}

/**
 * GET with path and url params
 */
#[get("/api/{view_name}")]
async fn get_api(view_name: web::Path<String>, url_params: web::Query<UrlParams>) -> Result<impl Responder> {
    let obj = MyResult {
        name: view_name.to_string(),
        method: "GET".to_string(),
        filter: url_params.filter.clone(),
        columns: url_params.columns.clone(),
        rows: url_params.rows.clone(),
        format: url_params.format.clone(),
    };
    Ok(web::Json(obj))
}

/**
 * POST with path and url params
 */
#[post("/api/{view_name}")]
async fn post_api(view_name: web::Path<String>, url_params: web::Query<UrlParams>) -> Result<impl Responder> {
    let obj = MyResult {
        name: view_name.to_string(),
        method: "GET".to_string(),
        filter: url_params.filter.clone(),
        columns: url_params.columns.clone(),
        rows: url_params.rows.clone(),
        format: url_params.format.clone(),
    };
    Ok(web::Json(obj))
}


// TODO: serve files

// http status code and custom headers in response

// stream result


/**
 * main app
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(get_api).service(post_api))
        .bind(("127.0.0.1", 80))?
        .run()
        .await
}
