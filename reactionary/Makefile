SRC         := src/
BUILD       := build/
PID_FILE    := .serve_pid
SERVE_ADDR  := localhost
SERVE_PORT  := 8000
SERVER_BIN  := http-server
SERVER_OPTS := . -s -a $(SERVE_ADDR) -p $(SERVE_PORT)
ifneq (,$(wildcard $(PID_FILE)))
SERVE_PID   := $(shell cat $(PID_FILE))
endif

.PHONY: all
all: build

.PHONY: build
build: # Build source files
	@echo Building [$(SRC)]...
	jsx $(SRC) $(BUILD)

.PHONY: watch
watch: # Watch source files and build when changes are detected
	jsx --watch $(SRC) $(BUILD)

.PHONY: serve
serve: # Serve the application
ifeq (,$(wildcard $(PID_FILE)))
	@($(SERVER_BIN) $(SERVER_OPTS)  & echo "$$!" > $(PID_FILE))
	@echo Serving app on [$(SERVE_ADDR):$(SERVE_PORT)]...
else
	@echo Already serving app on [$(SERVE_ADDR):$(SERVE_PORT)]...
endif

.PHONY: stop
stop: # Stop a running server
ifneq (,$(wildcard $(PID_FILE)))
	@kill $(SERVE_PID)
	@$(RM) $(PID_FILE)
	@echo Stopped process [$(SERVE_PID)]...
else
	@echo No server running...
endif

.PHONY: setup
setup: # Install development dependencies
	@echo Installing development dependencies...
	@npm install

.PHONY: clean
clean: # Clean build files
	@echo Cleaning [$(BUILD)]...
	$(RM) -r $(BUILD)

.PHONY: distclean
distclean: clean # Clean build and development files
	@echo Cleaning [node_modules]...
	$(RM) -r node_modules
