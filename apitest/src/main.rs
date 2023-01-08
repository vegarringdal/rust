use actix_web::{
    get,
    http::header::{ContentType},web, HttpResponse, 
};
use async_stream::stream;
use std::convert::Infallible;
use dotenv::dotenv;
use actix_cors::Cors;
use chrono::NaiveDateTime;

use oracle::{Result, ResultSet, Row};
use serde_json::{Map, Value};
use r2d2_oracle::OracleConnectionManager;
use actix_web::web::Data;

pub type DbPool = r2d2::Pool<OracleConnectionManager>;

#[get("/stream/{view_name}")]
async fn stream_api(pool_data: web::Data<DbPool>, _view_name: web::Path<String>) -> HttpResponse {

    let database_select = std::env::var("DB_SELECT").expect("DB_SELECT must be set.");
    println!("DB_SELECT {}", database_select);

    let conn = pool_data.get().unwrap();

    async fn get_rows(rows: &mut ResultSet<'_, Row>) -> Option<Result<Row>>{
        // next should have a await, so I could handle more requests on 1 thread
        let row_result = rows.next();
        row_result
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .streaming(stream! {

            let mut stmt = conn
            .statement(database_select.as_str())
            .fetch_array_size(100)
            .prefetch_rows(100)
            .build().unwrap();
            let mut rows = stmt.query(&[]).unwrap();
    
            let mut column_names = vec![];
            let mut column_types = vec![];

            for info in rows.column_info() {
                let col_name = info.name().to_string();
                let col_type = info.oracle_type().to_string();
                column_names.push(col_name);
                column_types.push(col_type);
            }         

            loop {
                // THis is still blocking...
                let row_result = get_rows(&mut rows).await;
                
                if row_result.is_none() {
                    break;
                }

                let row = row_result.unwrap().unwrap();

                let mut json_map = Map::new();
        
                for i in 0..column_names.len() {
                    let col_name = &column_names[i];
                    let col_type = &column_types[i];
                    let col_value_result: Result<String> = row.get(i);
                    
                    if col_value_result.is_ok() {
                        if col_type == "DATE" {

                            let date_str = col_value_result.unwrap();
                            let datetime = NaiveDateTime::parse_from_str(date_str.as_str(), "%Y-%m-%d %H:%M:%S");
                            if datetime.is_ok() {
                                // prob not 100% isodate, depends on what we get from oracle?... but a start :-)
                                json_map.insert(
                                    col_name.clone(),
                                    Value::String(
                                        datetime
                                            .unwrap()
                                            .format("%Y-%m-%dT%H:%M:%S.00Z")
                                            .to_string(),
                                    ),
                                );
                            }
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

    

    let database_connection_string =
    std::env::var("DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING must be set.");
    let database_username = std::env::var("DB_USERNAME").expect("DB_USERNAME must be set.");
    let database_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.");

    println!("DB_CONNECTION_STRING {}", database_connection_string);
    println!("DB_USERNAME {}", database_username);


    let manager = OracleConnectionManager::new(database_username.as_str(), database_password.as_str(), database_connection_string.as_str());
    let pool = r2d2::Pool::builder()
         .max_size(15)
         .build(manager)
         .unwrap();
    

    let timeout = std::time::Duration::from_secs(20);
    let keepalive = std::time::Duration::from_secs(20);

    
    let pool_clone = Data::new(pool.clone());
    HttpServer::new(move || {
        println!("Thread created {}", 1);
        let cors = Cors::permissive();
        App::new().app_data(pool_clone.clone()).service(stream_api).wrap(cors)
    }).client_request_timeout(timeout).keep_alive(keepalive).workers(1)
    
    .bind(("0.0.0.0", 1080))?
    .run()
    .await
    
    
}
