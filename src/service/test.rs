use crate::mapper;

pub async fn test_service() -> &'static str{
    mapper::test::get_one().await;

    "hello service"
}