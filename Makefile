BUILDER = cargo build

## == Targets ==
.PHONY: all

all: build dist clean

build:
	$(BUILDER)

dist: build
	@cp target/release bin
