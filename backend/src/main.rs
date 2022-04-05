use futures_lite::future;
use std::collections::HashMap;
use saras::server;
use saras::http;
use urlmatch::urlmatch;


struct Path {
    p: &'static str,
    f: fn(&HashMap<String, &str>) -> http::Resp,
}

fn url_dispatcher(url: &str) -> http::Resp {
    let routes = vec![
        Path {p: &"/", f: home},
        //Path {p: &"/catalogue/:ctg/:id", f: catalogue},
        //Path {p: &"/profile/:username", f: profile},
        //Path {p: &"/profile", f: profile},
    ];
    for route in routes.iter() {
        let r = urlmatch(url, route.p);
        if r.is_matched {
            return (route.f)(&r.keys);
        }
    }
    http::Resp{code: 404, text: "Not found".to_string()}
}

fn home(args: &HashMap<String, &str>) -> http::Resp {
    let text = format!("home(), args: {:?}", args);
    http::Resp{ code: 200, text: text }
}
fn catalogue(args: &HashMap<String, &str>) -> http::Resp {
    let text = format!("catalogue(), args: {:?}", args);
    http::Resp{ code: 200, text: text }
}
fn profile(args: &HashMap<String, &str>) -> http::Resp {
    let text = format!("profile(), args: {:?}", args);
    http::Resp{ code: 200, text: text }
}

fn main() {
    let _ = future::block_on(server::run(url_dispatcher));
}

