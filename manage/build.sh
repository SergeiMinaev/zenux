cd $ZENUX_HOME
source .env

cd manage
./bundle_js.py
./bundle_css.py
./minify.sh
cd $ZENUX_HOME/back
cargo build --release
