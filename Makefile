TARGET_TRIPLE=aarch64-unknown-linux-gnu

# without trailing `-` !!
TOOLCHAIN_PREFIX=aarch64-elf

LDFLAGS=-nostartfiles -nostdlib --build-id=none --gc-sections
CARGO=cargo

RUST_SRCS=src/lib.rs
RUST_BUILT_BINARY_PATH=target/$(TARGET_TRIPLE)/debug
RUST_BUILT_BINARY_NAME=libblink_rust.a
STARTCODE=start/start.S
LDSCRIPT=start/ldscript.ld

BUILD_PATH=build

OUTNAME=kernel.bin

all: $(RUST_SRCS) $(START_CODE) $(LDSCRIPT)
	$(CARGO) rustc --target $(TARGET_TRIPLE) -- -O
	mkdir -p $(BUILD_PATH)
	$(TOOLCHAIN_PREFIX)-gcc -c $(STARTCODE) -o $(BUILD_PATH)/start.o
	cp ${RUST_BUILT_BINARY_PATH}/${RUST_BUILT_BINARY_NAME} $(BUILD_PATH)/
	$(TOOLCHAIN_PREFIX)-strip -g --strip-unneeded --strip-dwo $(BUILD_PATH)/$(RUST_BUILT_BINARY_NAME)
	$(TOOLCHAIN_PREFIX)-ld $(LD_FLAGS) -T $(LDSCRIPT) $(BUILD_PATH)/start.o $(BUILD_PATH)/$(RUST_BUILT_BINARY_NAME) -o $(BUILD_PATH)/kernel.elf
	$(TOOLCHAIN_PREFIX)-objcopy -O binary $(BUILD_PATH)/kernel.elf $(BUILD_PATH)/$(OUTNAME)

clean:
	rm -rf $(BUILD_PATH)
