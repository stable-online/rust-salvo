use salvo::handler;

#[handler]
pub async fn test_1() -> &'static str {
    "hello world"
}