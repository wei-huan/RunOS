# Building
# target toolchain
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := $(KERNEL_ELF).bin
SDCARD := /dev/sda
APPS := ../user/src/bin/*
SUBMIT_BIN := ../os.bin

# PLATFORM qemu or k210
PLATFORM ?= qemu
# SBI opensbi or rustsbi
SBI ?= opensbi
# SBI_IMG
SBI_IMG :=../bootloader/$(SBI)-img/$(SBI)-$(PLATFORM).bin
# BOOTLOADER
BOOTLOADER := $(SBI_IMG)
# LOG
LOG ?= DEBUG

# FAT32
ifeq ($(PLATFORM), k210)
	FAT32_IMG := $(SDCARD)
else
	FAT32_IMG := ../fat32.img
endif

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

# CPUS
CPUS ?= 2

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -all -S

switch-check:
ifeq ($(PLATFORM), qemu)
	@(which last-qemu) || (rm -f last-k210 && touch last-qemu && make clean)
else ifeq ($(PLATFORM), k210)
	@(which last-k210) || (rm -f last-qemu && touch last-k210 && make clean)
endif

# env:
# 	@rustup target add riscv64gc-unknown-none-elf
# @cargo install cargo-binutils --vers =0.3.3
# @rustup component add rust-src
# @rustup component add llvm-tools-preview

build: switch-check $(KERNEL_BIN) # env

# fat32-img:
# 	@cd ../fat32-pack && ./createfs.sh
# 	@cd ../user && ./addtestfile2fs.sh

# fat32-oscomp-img:
# 	cd ../fat32-pack && ./createfs.sh
# 	cd ../oscomp && ./addoscompfile2fs.sh

# sdcard: fat32-img
# 	@echo "Are you sure write to $(SDCARD) ? [y/N] " && read ans && [ $${ans:-N} = y ]
# 	@sudo dd if=/dev/zero of=$(SDCARD) bs=1048576 count=32
# 	@sudo dd if=$(FS_IMG) of=$(SDCARD)

kernel:
	@cp src/linker-$(PLATFORM).ld src/linker.ld
	@cargo build --$(MODE) --features "platform-$(PLATFORM)" --features "$(SBI)" #--offline
	@rm src/linker.ld

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

run: build
ifeq ($(PLATFORM),qemu)
ifeq ($(SBI),opensbi)
	@qemu-system-riscv64 \
		-m 1G \
    	-cpu rv64 \
		-machine virt \
		-nographic \
		-bios $(SBI_IMG) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FAT32_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp $(CPUS)
else
	@qemu-system-riscv64 \
		-m 1G \
        -cpu rv64 \
		-machine virt \
		-nographic \
		-bios $(SBI_IMG) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FAT32_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp $(CPUS)
endif
else
	@(which $(K210-BURNER)) || (cd .. && git clone https://github.com/sipeed/kflash.py.git && mv kflash.py tools)
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	@dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(K210_BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@sudo chmod 777 $(K210-SERIALPORT)
	python3 $(K210-BURNER) -p $(K210-SERIALPORT) -b 1500000 $(KERNEL_BIN)
	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(K210-SERIALPORT) 115200
endif

debug: build
	@qemu-system-riscv64 \
		-m 512M \
        -cpu rv64 \
    	-machine virt \
    	-nographic \
    	-bios $(SBI_IMG) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FAT32_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-s -S \
		-smp $(CPUS)

all: build
	@rm -f $(SUBMIT_BIN)
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	@dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(K210_BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@cp $(KERNEL_BIN) $(SUBMIT_BIN)

disasm: kernel
	@ rm -f ../disasm.txt
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less >> ../disasm.txt

gdb:
	@gdb-multiarch \
    	-ex 'file $(KERNEL_ELF)' \
    	-ex 'set arch riscv:rv64' \
    	-ex 'target remote localhost:1234'

clean:
	@cargo clean
	@rm -f $(SUBMIT_BIN)

.PHONY: switch-check build fs-img sdcard kernel run debug disasm gdb clean all
