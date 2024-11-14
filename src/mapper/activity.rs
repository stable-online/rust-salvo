use rbatis::dark_std::defer;
use serde_json::json;
use tokio::time::timeout;
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::MysqlTableMapper;
use rbatis::{crud, html_sql, RBatis};
use rbatis::rbdc::db::ExecResult;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Activity123 {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

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