use salvo::Router;
use crate::controller::test::test_1;

pub fn init_router() -> Router {

    Router::with_path("hello/ss").get(test_1).post(test_1)
}