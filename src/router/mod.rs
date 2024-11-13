mod test;

use salvo::Router;

// init router
pub fn init_router() -> Router {
    let r = Router::new();

    r.push(
        test::init_test_router()
    )
}