BUILDER = cargo
BUILD_FLAGS = \

BIN = target/debug/borshed

## == Targets ==
.PHONY: all build clean dist run

all: build clean

build:
	@$(BUILDER) build $(BUILD_FLAGS)

clean:
	@echo Clean

run: build clean
	@./$(BIN)

# Package for distribution
dist: build clean
	@cp target/release bin