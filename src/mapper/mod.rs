pub mod activity;

use once_cell::sync::Lazy;
use rbatis::RBatis;

// global db
pub static GLOBAL_DB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

// 初始化数据库
pub async fn init_db() {
    GLOBAL_DB.link(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:zhaowei123@localhost/test",
    ).await.expect("数据库连接失败");
}
