mod test;

use salvo::Router;


/**
*
**/
pub fn init_router() -> Router {

    let r = Router::new();

    r.push(
        test::init_router()
    )
}