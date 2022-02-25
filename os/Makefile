# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

# BOARD
BOARD ?= qemu

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

kernel:
	cp src/linker-$(BOARD).ld src/linker.ld
	cargo build --$(MODE) --features "board-qemu"
	rm src/linker.ld

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

build: $(KERNEL_BIN)

run: build
	qemu-system-riscv64 \
		-m 256M \
    	-machine virt \
    	-nographic \
    	-bios ../opensbi-img/fw_jump.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
		-smp 4

debug:
	qemu-system-riscv64 \
		-m 8M \
    	-machine virt \
    	-nographic \
    	-bios ../rustsbi-img/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
		-s -S \
		-smp 2

disassem:
	@$(OBJDUMP) -all $(KERNEL_ELF)


gdb:
	riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
