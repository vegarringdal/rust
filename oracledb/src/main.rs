// "TESTDB", "TESTDB", "(DESCRIPTION=(CONNECT_TIMEOUT=5)(ADDRESS=(PROTOCOL=TCP)(HOST=192.168.10.40)(PORT=1522))(CONNECT_DATA=(SERVICE_NAME=xe)))"

use oracle::{Connection, Result};
use serde_json::{Map, Value};
use chrono::{NaiveDateTime};

fn main() {
    let result = connect_db();
    if result.is_err() {
        println!("error, {:?}", result)
    } else {
        println!("great {:?}", result)
    }

}



fn connect_db() -> Result<()> {
    let conn = Connection::connect(
            "TESTDB", 
            "TESTDB", 
            "(DESCRIPTION=(CONNECT_TIMEOUT=5)(ADDRESS=(PROTOCOL=TCP)(HOST=127.0.0.1)(PORT=1522))(CONNECT_DATA=(SERVICE_NAME=xe)))"
        )?;

    // Create a prepared statement
    let rows = conn.query("select * from t_person fetch first 2 rows only", &[])?;


    let mut column_names = vec![];
    let mut column_types = vec![];

    for info in rows.column_info() {
        let col_name= info.name().to_string();
        let col_type = info.oracle_type().to_string();
        column_names.push(col_name);
        column_types.push(col_type);

    }

    for row_result in rows {
        let row = row_result?;

        let mut json_map = Map::new();

        for i in 0..column_names.len() {
            let col_name = &column_names[i];
            
            
            let col_type = &column_types[i];
            //println!("datatype on {} is {}", col_name, col_type);

            
            let col_value_result:Result<String> = row.get(i);
            if col_value_result.is_ok() {
                if col_type == "DATE" {
                    let date_str =col_value_result.unwrap();
                    let datetime = NaiveDateTime::parse_from_str("2022-01-01 20:15:15", "%Y-%m-%d %H:%M:%S");
                    println!("datetime {} - {:?}", date_str, datetime);
                    if datetime.is_ok() {
                        // prob not 100% isodate, depends on what we get from oracle?... but a start :-)
                        json_map.insert(col_name.clone(), Value::String(datetime.unwrap().format("%Y-%m-%dT%H:%M:%S.00Z").to_string()));
                    }
                    
                } else {
                    json_map.insert(col_name.clone(), Value::String(col_value_result.unwrap()));
                }
                
            } else{
                json_map.insert(col_name.clone(), Value::Null);
            }          
        }
        let obj = Value::Object(json_map);
        if obj.is_object(){
            println!("row: {:?}", serde_json::to_string(&obj).unwrap())
        }
        

    }

    Ok(())

}
