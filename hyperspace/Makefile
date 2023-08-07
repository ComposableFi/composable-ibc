module-hyperspace=hyperspace
composable_centauri_image=composablefi/composable-centauri
composable_centauri_image_latest=composablefi/composable-centauri:latest
ifndef GITHUB_SHA
GITHUB_SHA=$(shell git rev-parse HEAD)
endif
composable_centauri_image_with_commit_hash="${composable_centauri_image}:${GITHUB_SHA}"
hyperspace_image="composablefi/hyperspace:latest"

.PHONY: run-setup-hyperspace stop-setup-hyperspace build-release-hyperspace tests-hyperspace
.PHONY: build-docker-image-hyperspace publish-docker-image-hyperspace

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
	docker build -f scripts/hyperspace.Dockerfile -t ${composable_centauri_image_with_commit_hash} .
	docker tag ${composable_centauri_image_with_commit_hash} ${composable_centauri_image_latest}

publish-docker-image-hyperspace:
	docker push "${composable_centauri_image_with_commit_hash}"
	docker push "${composable_centauri_image_latest}"

tests-hyperspace: 
	make --ignore-errors stop-setup-hyperspace
	sleep 3
	make run-setup-hyperspace
	./scripts/wait_for_polkadot_launch_container.sh
	cargo t -p $(module-hyperspace) --all-features
	docker stop composable_centauri

