use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, html_sql};

use crate::entity::activity::Activity123;

crud!(Activity123{});

#[html_sql("src/mapper/html/activity.html")]
pub async fn select_by_condition(
    rb: &dyn Executor,
    name: &str,
    dt: &DateTime,
    a: bool,
) -> rbatis::Result<Vec<Activity123>> {
    impled!()
}