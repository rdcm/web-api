up:
ifeq ($(flag),e)
	docker-compose up -d
else ifeq ($(flag),a)
	docker-compose --profile dev-build up -d
else
	$(warning "pass flag=e (environment only) or flag=a (all include applications)")
endif

down:
ifeq ($(flag),e)
	docker-compose down
else ifeq ($(flag),a)
	docker-compose --profile dev-build down
else
	$(warning "pass flag=e (environment only) or flag=a (all include applications)")
endif

build:
ifeq ($(flag),l)
	cargo build --workspace
else ifeq ($(flag),i)
	docker-compose --profile dev-build build
else
	$(warning "pass flag=l (for build local) or flag=i (for build images)")
endif

lint:
	cargo clippy

format:
	cargo fmt

run:
	./target/debug/host

tests:
	./run_tests.sh