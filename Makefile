TARGET=target
DEBUGGER_DIR=mach-test-browser-standalone
DEBUGGER_SRC=$(DEBUGGER_DIR)/Cargo.toml
DEBUG_OUTPUT=$(DEBUGGER_DIR)/public/default.log

PHONY: build debug docs test clean

build:
	cargo build

# TODO differentiate compile error from test assertion failure so that the
# output will not be rendered if it is empty
debug: $(TARGET) $(DEBUGGER_SRC)
	-RUST_TEST_THREADS=1 cargo test --features debug_renderevent -- --nocapture > $(DEBUG_OUTPUT)
	open 'http://localhost:8888'

debug-server: $(DEBUGGER_SRC)
	cd $(DEBUGGER_DIR); cargo run

$(TARGET):
	mkdir $(TARGET)

$(DEBUGGER_SRC):
	git submodule init
	git submodule update

docs:
	cargo doc --open

test:
	cargo test

clean:
	cargo clean
