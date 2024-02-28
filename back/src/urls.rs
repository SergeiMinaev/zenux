use futures_lite::{ Future, FutureExt };
use saras::http::{ Request,Resp };
use urlmatch::urlmatch;
use saras::http;
use crate::core;


struct Path<Fut>
where
	Fut: Future<Output = Resp>,
{
	p: &'static str,
	f: fn(Request) -> Fut,
}

pub async fn url_dispatcher(mut req: Request) -> Resp {
	//println!("Route request: {req:?}");
	let url = req.path.to_string().to_lowercase();
	let routes = vec![
		Path {p: &"", f: |req| core::index(req).boxed()},
	];
	//let protected_routes = vec![];

	for route in routes.iter() {
		let r = urlmatch(&url, route.p);
		req.route = r.keys;
		if r.is_matched {
			return (route.f)(req).await;
		}
	}
	//for route in protected_routes.iter() {
	//	  let r = urlmatch(&url, route.p);
	//	  req.route = r.keys;
	//	  if r.is_matched {
	//		  return check_su(req, route.f).await;
	//	  }
	//}
	http::not_found()

}
