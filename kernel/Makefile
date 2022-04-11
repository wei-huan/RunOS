# Building
# target toolchain
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
SDCARD := /dev/sda
APPS := ../user/src/bin/*

# FS
FS_IMG := ../user/target/$(TARGET)/$(MODE)/fs.img
# PLATFORM qemu or k210
PLATFORM ?= qemu
# SBI opensbi or rustsbi
SBI ?= opensbi
# SBI_IMG
SBI_IMG :=../bootloader/$(SBI)-img/$(SBI)-$(PLATFORM).bin
# BOOTLOADER
BOOTLOADER := $(SBI_IMG)

# KERNEL ENTRY
ifeq ($(PLATFORM), qemu)
	KERNEL_ENTRY_PA := 0x80200000
else ifeq ($(PLATFORM), k210)
ifeq ($(SBI), rustsbi)
	KERNEL_ENTRY_PA := 0x80020000
	K210_BOOTLOADER_SIZE := 131072
else
	KERNEL_ENTRY_PA := 0x80080000
	K210_BOOTLOADER_SIZE := 524288
endif
endif

# Run K210
K210-SERIALPORT	= /dev/ttyUSB0
K210-BURNER	= ../tools/kflash.py

# CPU_NUM
CPU_NUM := 2

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -all -S

switch-check:
ifeq ($(PLATFORM), qemu)
	(which last-qemu) || (rm -f last-k210 && touch last-qemu && make clean)
else ifeq ($(PLATFORM), k210)
	(which last-k210) || (rm -f last-qemu && touch last-k210 && make clean)
endif

build: switch-check $(KERNEL_BIN) fs-img

sdcard: fs-img
	@echo "Are you sure write to $(SDCARD) ? [y/N] " && read ans && [ $${ans:-N} = y ]
	@sudo dd if=/dev/zero of=$(SDCARD) bs=1048576 count=32
	@sudo dd if=$(FS_IMG) of=$(SDCARD)

fs-img: $(APPS)
	@cd ../user && make build
	@rm -f $(FS_IMG)
	@cd ../myfs-pack && cargo run --release -- -s ../user/src/bin/ -t ../user/target/riscv64gc-unknown-none-elf/release/

$(APPS):

kernel:
	cp src/linker-$(PLATFORM).ld src/linker.ld
	cargo build --$(MODE) --features "platform-$(PLATFORM)" --features "$(SBI)"
	rm src/linker.ld

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

run: build
ifeq ($(PLATFORM),qemu)
ifeq ($(SBI),opensbi)
	@qemu-system-riscv64 \
		-m 512M \
		-machine virt \
		-nographic \
		-bios $(SBI_IMG) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp $(CPU_NUM)
else
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(SBI_IMG) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-smp $(CPU_NUM)
endif
else
	(which $(K210-BURNER)) || (cd .. && git clone https://github.com/sipeed/kflash.py.git && mv kflash.py tools)
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	@dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(K210_BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@sudo chmod 777 $(K210-SERIALPORT)
	python3 $(K210-BURNER) -p $(K210-SERIALPORT) -b 1500000 $(KERNEL_BIN)
	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(K210-SERIALPORT) 115200
endif

debug: build
	@qemu-system-riscv64 \
		-m 128M \
    	-machine virt \
    	-nographic \
    	-bios $(SBI_IMG) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-s -S \
		-smp $(CPU_NUM)

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less >> ../disasm.txt

gdb:
	riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'

clean:
	@cargo clean

.PHONY: switch-check build fs-img sdcard kernel run debug disasm gdb clean