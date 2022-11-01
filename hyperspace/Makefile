module-hyperspace=hyperspace
composable_centauri_image="composablefi/composable-centauri:latest"

run-setup-hyperspace:
	docker run --platform linux/amd64 \
	-d \
	--rm \
	-ti \
	--name composable_centauri \
	-u1000:1000 \
	-p9944:9944 \
	-p9188:9188 \
	-p9988:9988 \
	$(composable_centauri_image)

stop-setup-hyperspace:
	docker stop composable_centauri

build-release-hyperspace:
	cargo b -p $(module-hyperspace) --release

build-docker-image-hyperspace:
	docker build -f hyperspace/Dockerfile -t hyperspace .

tests-hyperspace: 
	make --ignore-errors stop-setup-hyperspace
	sleep 3
	make run-setup-hyperspace
	sleep 60 # QUESTION: is there a better way to wait for the POLKADOT SUCCESS LAUNCH? 
	# I tried with docker logs -f ... | grep -xq '...' but did not work
	cargo t -p $(module-hyperspace) --all-features
	docker stop composable_centauri