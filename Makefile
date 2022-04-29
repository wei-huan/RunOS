TARGET := riscv64gc-unknown-none-elf
MODE := release
DIR := $(shell pwd)
PACK_IMG_DIR := $(DIR)/myfs-pack
FAT32_PACK_DIR := $(DIR)/fat32-pack
USER_DIR := $(DIR)/user
USER_TAR_DIR := $(USER_DIR)/target/$(TARGET)/$(MODE)
OSCOMP_DIR := $(DIR)/oscomp
OSCOMP_TAR_DIR := $(OSCOMP_DIR)/build/riscv64
OS_DIR := $(DIR)/kernel
FS_IMG := $(IMG_DIR)/fs.img

PLATFORM ?= qemu

build:
	@make build -C $(OS_DIR)

sdcard:
	@make sdcard -C $(OS_DIR)

fat32:
	@make fat32-img -C $(OS_DIR)

user:
	@make build -C $(USER_DIR)

# fs: user
# 	@rm -f $(FS_IMG)
# 	@cd $(PACK_IMG_DIR) && cargo run --release -- -s $(USER_DIR)/src/bin/ -t $(IMG_DIR)/

fat32-oscomp: user
ifeq ($(PLATFORM), qemu)
	cd fat32-pack && ./createfs.sh
	cd oscomp && ./addoscompfile2fs.sh qemu
else
	cd oscomp && ./addoscompfile2fs.sh k210
endif

run:
	@make run -C $(OS_DIR)

debug: fat32-oscomp
	@make debug -C $(OS_DIR)

gdb:
	@make gdb -C $(OS_DIR)

disasm:
	@make disasm -C $(OS_DIR)

.PHONY: build sdcard user fs run debug gdb disasm fat32-oscomp fat32
