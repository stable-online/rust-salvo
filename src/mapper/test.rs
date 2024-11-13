use rbatis::crud;
use rbatis::rbdc::DateTime;
use super::GLOBAL_DB;

/// table
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

pub async fn get_one() {

    println!("{:?}", Activity123::select_by_column(&GLOBAL_DB.clone(), "id", "1").await.unwrap());
}
