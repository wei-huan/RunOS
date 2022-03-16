DIR := $(shell pwd)
PACK_IMG_DIR := $(DIR)/myfs-pack
USER_DIR := $(DIR)/user
IMG_DIR := $(USER_DIR)/target/riscv64gc-unknown-none-elf/release/
OS_DIR := $(DIR)/os

user:
	@make build -C $(USER_DIR)

fs-img: user
	@cd $(PACK_IMG_DIR) && cargo run --release -- -s $(USER_DIR)/src/bin/ -t $(IMG_DIR)

os: fs-img
	@make run -C $(OS_DIR)

debug:
	@make debug -C $(OS_DIR)

gdb:
	@make gdb -C $(OS_DIR)

.PHONY: user fs-img os debug gdb
