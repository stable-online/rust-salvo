use rbatis::rbdc::DateTime;
use crate::mapper;
use crate::mapper::GLOBAL_DB;

pub async fn test_service() -> &'static str {
    println!("{:?}", mapper::activity::select_by_condition(
        &GLOBAL_DB.acquire().await.unwrap(),
        "xiaoming",
        &DateTime::now(),
        true).await.unwrap()
    );

    "hello service"
}