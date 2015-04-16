TARGET=target
DEBUG_OUTPUT=$(TARGET)/debug.logs
VIZ_GENERATOR=tests/mithril-ci-offline/generate_index

PHONY: build debug docs test clean

build:
	cargo build

debug: $(TARGET) $(VIZ_GENERATOR)
	-RUST_TEST_THREADS=1 cargo test -- --nocapture > $(DEBUG_OUTPUT)
	$(VIZ_GENERATOR) $(DEBUG_OUTPUT) | xargs open

$(TARGET):
	mkdir $(TARGET)

$(VIZ_GENERATOR):
	git submodule init
	git submodule update

docs:
	cargo doc --open

test:
	cargo test

clean:
	cargo clean
