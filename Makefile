DEFAULT         := debug
COMPILER        := rustc
COMPILER_DOCS   := rustdoc
MITHRIL_ROOT    := src/mithril.rs
BUILD_OPTIONS   := -g

BINARY_DIR      := bin
MITHRIL_OUTPUT  := $(BINARY_DIR)/libmithril.rlib

ifdef VERBOSE
	ECHO :=
else
	ECHO := @
endif

.PHONY : all help prepare build build-tests docs clean

all: $(DEFAULT)

help:
	@echo "--- Mithril Makefile"
	@echo ""
	@echo " make              - runs the default task (debug)"
	@echo " make debug        - runs task build and build-tests"
	@echo " make build        - builds the library file"
	@echo " make build-tests  - builds and runs the tests"
	@echo " make docs         - builds and tests the documentation using rustdoc"
	@echo ""
	@echo "---"

debug: build build-tests

prepare:
	$(ECHO)mkdir -p bin

build: prepare
	$(ECHO)$(COMPILER) --crate-type=lib $(BUILD_OPTIONS) $(MITHRIL_ROOT) -o $(MITHRIL_OUTPUT)

build-tests: prepare
	$(ECHO)$(COMPILER) --test $(TEST_OPTIONS) $(MITHRIL_ROOT) -o $(BINARY_DIR)/mithril_tests
	$(ECHO)./bin/mithril_tests

docs:
	$(ECHO)$(COMPILER_DOCS) $(MITHRIL_ROOT)
	$(ECHO)open doc/mithril/index.html
	$(ECHO)$(COMPILER_DOCS) --test $(MITHRIL_ROOT) -L $(BINARY_DIR)/

clean:
	$(ECHO)rm -rf doc/
	$(ECHO)rm -rf $(BINARY_DIR)
