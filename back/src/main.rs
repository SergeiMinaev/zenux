use futures_lite::{ future };
use saras::listener;
use zenux::urls;


fn main() {
	let _ = future::block_on(listener::run(urls::url_dispatcher));
}
