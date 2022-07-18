use std::collections::HashMap;
use futures_lite::{future, Future, FutureExt};
use urlmatch::urlmatch;
use saras::listener;
use saras::http;
use saras::http::Resp;



struct Path<Fut>
where
    Fut: Future<Output = Resp>,
{
    p: &'static str,
    f: fn(HashMap<String, String>) -> Fut,
}

async fn url_dispatcher(url: String) -> Resp {
    let routes = vec![
        Path {p: &"/profile", f: |args| profile(args).boxed()},
        Path {p: &"/catalogue/:ctg/:id", f: |args| catalogue(args).boxed()},
        Path {p: &"/json", f: |args| get_json(args).boxed()},
    ];
    for route in routes.iter() {
        let r = urlmatch(&url, route.p);
        if r.is_matched {
            return (route.f)(r.keys).await;
        }
    }
    http::text_resp(404, "Not found".to_string())
}

async fn catalogue(args: HashMap<String, String>) -> Resp {
    let text = format!("catalogue(), args: {:?}", args);
    http::text_resp(200, text)
}
async fn profile(args: HashMap<String, String>) -> Resp {
    let text = format!("profile(), args: {:?}", args);
    http::text_resp(200, text)
}
async fn get_json(_args: HashMap<String, String>) -> Resp {
    let json = format!(r#"
        {{
            "name": "Adam",
            "age": "{}"
        }}
    "#, i64::MIN);
    http::json_resp(200, json)
}


fn main() {
    let _ = future::block_on(listener::run(url_dispatcher));
}
