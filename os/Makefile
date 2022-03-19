# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

# PLATFORM
PLATFORM ?= qemu

# FS
FS_IMG := ../user/target/$(TARGET)/$(MODE)/fs.img

#SBI
SBI := ../bootloader/opensbi-img/fw_jump.bin

# CPU_NUM
CPU_NUM := 2

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

kernel:
	cp src/linker-$(PLATFORM).ld src/linker.ld
	cargo build --$(MODE)
	rm src/linker.ld

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

build: $(KERNEL_BIN)

run: build
	qemu-system-riscv64 \
		-m 128M \
    	-machine virt \
    	-nographic \
    	-bios $(SBI) \
		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp $(CPU_NUM)

debug:
	qemu-system-riscv64 \
		-m 128M \
    	-machine virt \
    	-nographic \
    	-bios $(SBI) \
		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-s -S \
		-smp $(CPU_NUM)

disassem:
	@$(OBJDUMP) -all $(KERNEL_ELF)


gdb:
	riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
