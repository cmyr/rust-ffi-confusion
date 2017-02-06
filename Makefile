ifeq ($(shell uname),Darwin)
        LDFLAGS := -Wl,-dead_strip
    else
        LDFLAGS := -Wl,--gc-sections -lpthread
    endif

all: target/vec
	target/vec

target:
	mkdir -p $@

target/vec: target/main.o target/debug/libvec_test.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libvec_test.a: src/lib.rs Cargo.toml
	cargo build

target/main.o: src/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
