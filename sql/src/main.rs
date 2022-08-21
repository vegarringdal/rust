use sqlparser::ast::{Query, Select, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {
    let user_where_input = "person = 1";
    let select_str = "select * from sometable where";

    let sql = format!("{} {}", select_str, user_where_input);

    println!("result {:?}", check_if_sql_is_select(&sql));


    // unaparsable

    let user_where_input = "person = 1";
    let select_str = "selectd * from sometable where";

    let sql = format!("{} {}", select_str, user_where_input);

    println!("result {:?}", check_if_sql_is_select(&sql));

    // multi

    let user_where_input = "1 = 1;drop table users";
    let select_str = "select * from sometable where";

    let sql = format!("{} {}", select_str, user_where_input);

    println!("result {:?}", check_if_sql_is_select(&sql));


    // multi

    let user_where_input = "";
    let select_str = "drop table users";

    let sql = format!("{} {}", select_str, user_where_input);

    println!("result {:?}", check_if_sql_is_select(&sql));
}

fn check_if_sql_is_select(input: &str) -> Result<(), String> {
    let dialect = GenericDialect {};
    let ast_result = Parser::parse_sql(&dialect, input);

    if ast_result.is_err() {
        return Err("unable to parse string".to_owned());
    }

    let ast = ast_result.unwrap();

    if ast.len() > 1 {
        Err("Not allowed to have more then 1 statement".to_owned())
    } else {
        let expression1 = &ast[0];

        match expression1 {
            Statement::Query(..) => Ok(()),
            _ => Err("Only query statement is allowed".to_owned()),
        }
    }
}
