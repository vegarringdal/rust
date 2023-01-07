use actix_files as fs;
use actix_web::{
    get,
    http::header::{ContentDisposition, ContentType, DispositionType},
    post, web, Error, HttpRequest, HttpResponse, Responder, Result,
};
use async_stream::stream;
use json::JsonValue;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::{thread, time};

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
async fn get_api(
    view_name: web::Path<String>,
    url_params: web::Query<UrlParams>,
) -> Result<impl Responder> {
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
async fn post_api(
    body: web::Bytes,
    view_name: web::Path<String>,
    url_params: web::Query<UrlParams>,
) -> Result<impl Responder> {
    let obj = MyResult {
        name: view_name.to_string(),
        method: "GET".to_string(),
        filter: url_params.filter.clone(),
        columns: url_params.columns.clone(),
        rows: url_params.rows.clone(),
        format: url_params.format.clone(),
    };

    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    if injson["name"].is_null() {
        println!("missing name:{}", injson);
    } else {
        println!("payload:{}", injson);
    }

    Ok(web::Json(obj))
}

/**
 * serve static files
 */
#[get("/{filename:.*}")]
async fn files(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let path_string = path.into_os_string().into_string().unwrap();
    let www_folder = String::from("./www/") + &path_string;
    println!("Current file beeing requested{}", www_folder);
    let file = fs::NamedFile::open(www_folder)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[get("/stream/{view_name}")]
async fn stream_api(_view_name: web::Path<String>) -> HttpResponse {
   /*       let ten_millis = time::Duration::from_nanos(1); 
 */
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .streaming(stream! {

            for _ in 0..100000 {
               /*  thread::sleep(ten_millis);   */
                yield Ok::<_, Infallible>(web::Bytes::from("{\"id\":\"638b9d658ff026c95149675c\",\"index\":0,\"guid\":\"949900d1-d771-4545-b431-ca6cd6e0be62\",\"isActive\":false,\"isDumb\":true,\"balance\":3343.27,\"own\":1792.26,\"picture\":\"http://placehold.it/32x32\",\"age\":33,\"eyeColor\":\"blue\",\"name\":\"Johanna Zamora\",\"gender\":\"female\",\"country\":\"\",\"company\":\"ZOLAREX\",\"email\":\"johannazamora@zolarex.com\",\"phone\":\"+1 (821) 513-3194\",\"address\":\"660 Brigham Street, Gadsden, Virgin Islands, 5463\",\"about\":\"Do labore labore aute voluptate ipsum. Occaecat eu officia qui Lorem tempor proident id occaecat occaecat cupidatat do commodo. Irure ipsum incididunt nostrud nostrud laborum anim ea consectetur nostrud laborum consequat. Enim eiusmod veniam ipsum in cillum eiusmod ea aute incididunt nostrud velit exercitation fugiat tempor. Sunt voluptate consectetur minim adipisicing veniam aute esse. Sunt dolor cillum sint tempor incididunt anim commodo amet aliquip nostrud.\\r\\n\",\"registered\":\"2018-01-30T08:19:18 -01:00\",\"latitude\":null,\"longitude\":null,\"tags\":\"tempor\",\"friends\":\"Weber Tran\",\"date1\":\"2022-11-30T19:03:01.969Z\",\"date2\":\"2022-11-30T19:03:01.969Z\",\"greeting\":\"Hello, Johanna Zamora! You have 10 unread messages.\",\"favoriteFruit\":\"strawberry\"}"))
            }


        })
}

/**
 * main app
 */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(get_api)
            .service(post_api)
            .service(stream_api)
            .service(files)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
