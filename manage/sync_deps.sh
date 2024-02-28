cd $ZENUX_HOME
source .env
if [[ $MODE != "prod" ]]; then
	echo "Mode is not 'prod'. Closing."
	exit
fi

while IFS= read -r url; do
	echo "Syncing:" $url
	name=$(echo $url | tr '/' '\n' | tail -n 1)
	if [[ ! -d ./front/ext/${name} ]]; then
		cd ./front/ext
		git clone $url
	fi
	cd ${ZENUX_HOME}/front/ext/${name}
	git pull
done < ./manage/front_deps.txt
echo "Done"
