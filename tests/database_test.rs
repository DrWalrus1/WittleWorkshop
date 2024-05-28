use rocket::tokio;
use wittle_workshop_api::services::connect_db;

#[derive(sqlx::FromRow, Debug)]
struct Hello {
    id: i32,
    name: String,
    description: String,
}

#[tokio::test]
async fn db_is_live() {
    let db_address = "127.0.0.1";
    let db_port = "5432";
    let db_username = "admin";
    let db_password = "Password1";
    let db_name = "Test";
    let pool = connect_db(db_address, db_port, db_username, db_password, db_name).await;

    let hello = sqlx::query_as::<_, Hello>("SELECT * FROM hello")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(hello.id, 1);
    assert_eq!(hello.name, "Jeff");
    assert_eq!(hello.description, "Jeff's description");
}
