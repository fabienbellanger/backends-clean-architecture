.PHONY: help \
	serve \
	watch \
	lint \
	lint-audit \
	test \
	clean \
	sqlx-prepare \
	doc \
	doc-deps \
	docker \
	docker-pull \
	docker-up \
	docker-up-no-daemon \
	docker-down \
	docker-down-rm \
	docker-cli-build \
	docker-cli-register

.DEFAULT_GOAL=help

# Parameters
APP_NAME="Rust Clean Architecture"
CURRENT_PATH=$(shell pwd)
DOCKER_COMPOSE=docker-compose
DOCKER=docker
CARGO=cargo
CARGO_BIN_NAME="clean-architecture-infrastructure"
USER_LASTNAME="Admin"
USER_FIRSTNAME="Test"
USER_EMAIL="test2@testest.com"
USER_PASSWORD="00000000"

help: Makefile
	@echo
	@echo "Choose a command run in "$(APP_NAME)":"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' | sed -e 's/^/ /'
	@echo

## serve: Start web server
serve:
	cd infrastructure && $(CARGO) run -- serve

## watch: Start web server with hot reload
watch:
	cd infrastructure && $(CARGO) watch -x "run -- serve"

## lint: Run clippy and rustfmt
lint:
	$(CARGO) fmt
	$(CARGO) clippy -- -Dwarnings

## lint-audit: Run clippy, rustfmt and audit
lint-audit: lint
	$(CARGO) audit

## test: Launch unit tests in a single thread
test:
	$(CARGO) test -- --test-threads=1 --nocapture

## clean: Remove target directory
clean:
	$(CARGO) clean

## sqlx-prepare: Prepare for sqlx offline mode
sqlx-prepare:
	cd infrastructure && $(CARGO) sqlx prepare -- --bin $(CARGO_BIN_NAME)

## doc: Open Rust documentation without dependencies
doc:
	$(CARGO) doc --open --no-deps --document-private-items

## doc: Open Rust documentation with dependencies
doc-deps:
	$(CARGO) doc --open --document-private-items

## docker: Stop running containers, build docker-compose.yml file and run containers
docker: docker-down sqlx-prepare docker-up

## docker-pull: Pull images
docker-pull:
	$(DOCKER_COMPOSE) pull

## docker-up: Build docker-compose.yml file and run containers
docker-up: docker-pull
	$(DOCKER_COMPOSE) up --build --force-recreate -d

## docker-up-no-daemon: Build docker-compose.yml file and run containers in non daemon mode
docker-up-no-daemon: docker-pull
	$(DOCKER_COMPOSE) up --build

## docker-down: Stop running containers
docker-down:
	$(DOCKER_COMPOSE) down --remove-orphans

## docker-down-rm: Stop running containers and remove linked volumes
docker-down-rm:
	$(DOCKER_COMPOSE) down --remove-orphans --volumes

## docker-cli-build: Build project for CLI
docker-cli-build: docker-pull
	$(DOCKER) build -f Dockerfile -t rust-clean-architecture-infrastructure-cli .

## docker-cli-register: Run CLI container to register an admin user
docker-cli-register: docker-cli-build
	$(DOCKER) run -i --rm --net rust_clean_archi_backend --link rust_clean_archi_mariadb rust-clean-architecture-infrastructure-cli \
	register -l $(USER_LASTNAME) -f $(USER_FIRSTNAME) -e $(USER_EMAIL) -p $(USER_PASSWORD)
