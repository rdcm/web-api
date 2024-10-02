docker-up-env:
	docker compose up -d

docker-up-apps:
	docker compose --profile dev-build up -d

docker-down-env:
	docker compose down

docker-down-apps:
	docker compose --profile dev-build down

build:
	cargo build --workspace

docker-build-apps:
	docker compose --profile dev-build build

lint:
	cargo clippy

format:
	cargo fmt

run:
	./target/debug/host

tests:
	./run_tests.sh