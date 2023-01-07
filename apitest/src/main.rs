use actix_web::{
    get,
    http::header::{ContentType},web, HttpResponse, 
};
use async_stream::stream;
use std::convert::Infallible;
use dotenv::dotenv;
use actix_cors::Cors;


use oracle::{Connection, Result};
use serde_json::{Map, Value};




#[get("/stream/{view_name}")]
async fn stream_api(_view_name: web::Path<String>) -> HttpResponse {


    let database_connection_string =
    std::env::var("DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING must be set.");
    let database_username = std::env::var("DB_USERNAME").expect("DB_USERNAME must be set.");
    let database_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.");
    let database_select = std::env::var("DB_SELECT").expect("DB_SELECT must be set.");
    let conn = Connection::connect(
        database_username,
        database_password,
        database_connection_string,
    ).unwrap();
    
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .streaming(stream! {

            let mut stmt = conn
            .statement(database_select.as_str())
            .fetch_array_size(1000)
            .prefetch_rows(1000)
            .build().unwrap();
            let rows = stmt.query(&[]).unwrap();
    
            let mut column_names = vec![];
            let mut column_types = vec![];

            for info in rows.column_info() {
                let col_name = info.name().to_string();
                let col_type = info.oracle_type().to_string();
                column_names.push(col_name);
                column_types.push(col_type);
            }

            for row_result in rows {
                let row = row_result.unwrap();
                let mut json_map = Map::new();
        
                for i in 0..column_names.len() {
                    let col_name = &column_names[i];
                    let col_type = &column_types[i];
                    let col_value_result: Result<String> = row.get(i);
                    
                    if col_value_result.is_ok() {
                        if col_type == "DATE" {
                            // todo
                        } else {
                            json_map.insert(col_name.clone(), Value::String(col_value_result.unwrap()));
                        }
                    } else {
                        json_map.insert(col_name.clone(), Value::Null);
                    }
                }

                let obj = Value::Object(json_map);                  
                yield Ok::<_, Infallible>(web::Bytes::from(serde_json::to_string(&obj).unwrap()));
            }

        })
}

/**
 * main app
 */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    use actix_web::{App, HttpServer};

 

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().service(stream_api).wrap(cors)
    })
    
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
