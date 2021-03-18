use actix_web::client::{Client, ClientRequest, SendRequestError};
use actix_web::dev::PayloadStream;
use actix_web::http::StatusCode;
use actix_web::web::{Bytes, Json, Payload};
use serde_json::Value;

#[inline]
fn set_headers(req: &mut ClientRequest) {
    req.set_header("accept", "application/json, text/plain, */*");
    req.set_header("accept-encoding", "gzip, deflate, br");
    req.set_header("accept-language", "zh-CN,zh;q=0.9,en;q=0.8");
    req.set_header("origin", "https://live.bilibili.com");
    req.set_header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36");
}

async fn get_json(url: &str) -> Option<Value> {
    let mut req = Client::new().get(url);
    set_headers(&mut req);
    match req.send().await {
        Ok(mut resp) => match resp.json::<Value>().await {
            Ok(v) => Some(v),
            Err(err) => {
                error!("JSON Parse Error: {}", err);
                None
            }
        },
        Err(err) => {
            error!("HTTP Request Error: {}", err);
            None
        }
    }
}

pub async fn get_web_area_list() -> Option<Value> {
    get_json(
        "https://api.live.bilibili.com/xlive/web-interface/v1/index/getWebAreaList?source_id=2",
    )
}

pub async fn get_parent_area_list_order_by_online(parent_area_id: i32) -> Option<Value> {
    get_json(
        &format!("https://api.live.bilibili.com/xlive/web-interface/v1/second/getList?platform=web&parent_area_id={}&area_id=0&sort_type=online&page=1", parent_area_id),
    )
}
