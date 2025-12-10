use crate::models::api_test::ApiTest;
use axum::{Json, Router, extract::State, routing::get};
use rand::prelude::*; // 引入 SliceRandom trait
use rand::rng;
use sqlx::MySqlPool;

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/api_test", get(get_random_api_test))
}

async fn get_random_api_test(State(pool): State<MySqlPool>) -> Json<ApiTest> {
    let api_tests = sqlx::query_as::<_, ApiTest>("SELECT * FROM api_test")
        .fetch_all(&pool)
        .await
        .expect("查询数据库失败");

    let mut rng = rng();

    let random_api_test = api_tests
        .choose(&mut rng)
        .cloned()
        .expect("api_test 表为空，无法随机选择");

    Json(random_api_test)
}
