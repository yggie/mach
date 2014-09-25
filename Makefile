DEFAULT         := debug
CARGO           := cargo

ifdef VERBOSE
	ECHO :=
else
	ECHO := @
endif

.PHONY : all help build build-tests docs clean

all: $(DEFAULT)

help:
	@echo "--- Mithril Makefile"
	@echo ""
	@echo " make              - runs the default task (debug)"
	@echo " make debug        - runs the build-tests task"
	@echo " make build        - builds the library file"
	@echo " make build-tests  - builds and runs the tests"
	@echo " make docs         - builds and tests the documentation using rustdoc"
	@echo ""
	@echo "---"

debug: build-tests

build:
	$(ECHO)$(CARGO) build

build-tests:
	$(ECHO)$(CARGO) test

docs:
	$(ECHO)$(CARGO) doc
	$(ECHO)open target/doc/mithril/index.html

clean:
	$(ECHO)$(CARGO) clean
