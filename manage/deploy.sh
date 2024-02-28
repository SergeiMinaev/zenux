cd $ZENUX_HOME/manage
./sync_deps.sh
./build.sh
sudo /etc/init.d/zenux restart
