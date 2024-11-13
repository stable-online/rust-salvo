use salvo::handler;
use crate::service::test;

#[handler]
pub async fn test_1() -> &'static str {
    test::test_service()
}