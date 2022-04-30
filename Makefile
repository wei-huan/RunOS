TARGET := riscv64gc-unknown-none-elf
MODE := release
DIR := $(shell pwd)
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

# sdcard:
# 	@make sdcard -C $(OS_DIR)

# fat32:
# 	@make fat32-img -C $(OS_DIR)

user:
	@make build -C $(USER_DIR)

fat32-oscomp: user
ifeq ($(PLATFORM), qemu)
	@./createfs.sh
	@cd oscomp && ./addoscompfile2fs.sh qemu
else
	@cd oscomp && ./addoscompfile2fs.sh k210
endif

run: user
	@make run -C $(OS_DIR)

debug: fat32-oscomp
	@make debug -C $(OS_DIR)

gdb:
	@make gdb -C $(OS_DIR)

disasm:
	@make disasm -C $(OS_DIR)

all: user
	@make all -C $(OS_DIR)

clean:
	@rm -f fat32.img
	@rm -rf ./fs
	@make clean -C $(USER_DIR)
	@make clean -C $(OS_DIR)

.PHONY: build sdcard user fs run debug gdb disasm fat32-oscomp fat32 clean
