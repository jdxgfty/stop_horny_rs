.PHONY: build clean docker build-docker build-static mk_app_dir build-docker-static
output_folder := "app/"
DOCKER_CMD = "docker build -t "stop_horny_rs" ."
IMG_PATH := "assets/stop_horny.jpg"

all: build

mk_app_dir:
	-@mkdir -p ${output_folder}

build: src/main.rs mk_app_dir
	@cargo build --release
	@cp target/release/stop_horny ${IMG_PATH} ${output_folder}

build-static: src/main.rs mk_app_dir
	cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/stop_horny ${IMG_PATH} ${output_folder}

build-docker: app/stop_horny
	${DOCKER_CMD}

build-docker-static: app/stop_horny
	${DOCKER_CMD}

clean:
	@rm -r ${output_folder}
