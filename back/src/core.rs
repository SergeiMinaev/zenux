use std::fs;
use std::io::Read;
use saras::http::{ Request, Resp, html_resp };
use saras::conf::CONF;
use saras::tpl::tpl;


pub async fn index(_req: Request) -> Resp {
	let mut html = fs::File::open("../front/templates/core/index.html").unwrap();
	let mut main_buf: Vec<u8> = Vec::new();
	html.read_to_end(&mut main_buf).unwrap();
	let mut tpls_buf: Vec<u8> = Vec::new();
	for fname in fs::read_dir("../front/templates/core/").unwrap()
			.filter_map(Result::ok)
			.filter_map(|d| d.path().to_str().and_then(
				|f| if f.ends_with("/index.html") == false
				&& f.ends_with(".html") { Some(d) } else { None }
			))
	{
		let mut tpl = fs::File::open(fname.path()).unwrap();
		let mut tpl_buf: Vec<u8> = Vec::new();
		tpl.read_to_end(&mut tpl_buf).unwrap();
		tpls_buf.extend_from_slice(&tpl_buf);
	}
	let main_content = std::str::from_utf8(&main_buf).unwrap();
	let tpl_content = std::str::from_utf8(&tpls_buf).unwrap();
	let main_content = main_content.replace("<#inc-templates>", tpl_content);

	let conf = CONF.read().await;
	let r = tpl(main_content, "DEV", conf.is_dev);
	let r = tpl(r, "PROD", conf.is_dev == false);

	html_resp(200, r.to_string())
}
