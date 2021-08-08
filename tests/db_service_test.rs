use ntx_ventures::services::db_service;
/*
connection has been successfully setup
Insert function is working
Read function is working
Delete function is working
*/
#[tokio::test]
async fn test_db_operations() {
    let result = db_service::setup().await;
    let res = match result {
        Ok(r) => r,
        Err(error) => panic!("Problem getting db conn: {:?}", error),
    };
    /* SELECT, ADD, DELETE Tests */
    // insert test
    let insert = r#"INSERT INTO example (id, name) VALUES(3, "test")"#;
    let r1_wrapped = db_service::query(&res, insert).await;
    let r1 = match r1_wrapped {
        Ok(r) => r,
        Err(error) => panic!("Problem getting db conn: {:?}", error),
    };

    // delete test
    let delete = "DELETE FROM example WHERE id = 3";
    let r2_wrapped = db_service::query(&res, delete).await;
    let r2 = match r2_wrapped {
        Ok(r) => r,
        Err(error) => panic!("Problem getting db conn: {:?}", error),
    };
    assert_eq!(r2.rows_affected(), 1);

    // select test
    let select = "SELECT * FROM example";
    let r3_wrapped = db_service::select_query(&res, select).await;
    let r3 = match r3_wrapped {
        Ok(r) => r,
        Err(error) => panic!("Problem getting db conn: {:?}", error),
    };
    assert_eq!(r3.len(), 1);
}
