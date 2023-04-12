.PHONY: all \
	install \
	update \
	update-all \
	format \
	vet \
	serve \
	serve-race \
	logs \
	build \
	test \
	bench \
	clean \
	help \
	test-cover-count \
	cover-count \
	test-cover-atomic \
	cover-atomic \
	html-cover-count \
	html-cover-atomic \
	run-cover-count \
	run-cover-atomic \
	view-cover-count \
	view-cover-atomic

.DEFAULT_GOAL=help

include .env

# Read: https://kodfabrik.com/journal/a-good-makefile-for-go

# Go parameters
CURRENT_PATH=$(shell pwd)
MAIN_PATH=$(CURRENT_PATH)/cmd/main.go
GO_CMD=go
GO_INSTALL=$(GO_CMD) install
GO_RUN=$(GO_CMD) run
GO_BUILD=$(GO_CMD) build
GO_CLEAN=$(GO_CMD) clean
GO_TEST=$(GO_CMD) test
GO_GET=$(GO_CMD) get
GO_MOD=$(GO_CMD) mod
GO_TOOL=$(GO_CMD) tool
GO_VET=$(GO_CMD) vet
GO_FMT=$(GO_CMD) fmt
BINARY_NAME=go-clean-architecture
BINARY_UNIX=$(BINARY_NAME)_unix

## all: Test and build application
all: test build

## install: Run go install
install:
	$(GO_INSTALL) ./...

## update: Update modules
update:
	$(GO_GET) -u ./... && $(GO_MOD) tidy

## update-all: Update all modules
update-all:
	$(GO_GET) -u ./... all && $(GO_MOD) tidy

## format: Run go fmt
format:
	$(GO_FMT) ./...

## vet: Run go vet
vet:
	$(GO_VET) ./...

## serve: Serve API
serve: format vet
	$(GO_RUN) $(MAIN_PATH) run

## serve-race: Serve API with -race option
serve-race: format vet
	$(GO_RUN) run -race $(MAIN_PATH)

## logs: Display server logs
logs:
	$(GO_RUN) $(MAIN_PATH) logs --server

build: format vet
	$(GO_BUILD) -ldflags "-s -w" -o $(BINARY_NAME) -v $(MAIN_PATH)

## test: Run test
test:
	$(GO_TEST) -cover -v ./...

test-cover-count: 
	$(GO_TEST) -covermode=count -coverprofile=cover-count.out ./...

test-cover-atomic: 
	$(GO_TEST) -covermode=atomic -coverprofile=cover-atomic.out ./...

cover-count:
	$(GO_TOOL) cover -func=cover-count.out

cover-atomic:
	$(GO_TOOL) cover -func=cover-atomic.out

html-cover-count:
	$(GO_TOOL) cover -html=cover-count.out

html-cover-atomic:
	$(GO_TOOL) cover -html=cover-atomic.out

run-cover-count: test-cover-count cover-count
run-cover-atomic: test-cover-atomic cover-atomic
view-cover-count: test-cover-count html-cover-count
view-cover-atomic: test-cover-atomic html-cover-atomic

## bench: Run benchmarks
bench: 
	$(GO_TEST) -benchmem -bench=. ./...

## clean: Clean files
clean: 
	$(GO_CLEAN)
	rm -f $(BINARY_NAME)
	rm -f $(BINARY_UNIX)

help: Makefile
	@echo
	@echo "Choose a command run in "$(APP_NAME)":"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' | sed -e 's/^/ /'
	@echo
