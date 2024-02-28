cd ${ZENUX_HOME}/manage

urxvt -e bash ./static-srv.sh &
urxvt -e bash ./app-srv.sh &
urxvt -e bash ./watcher.sh
