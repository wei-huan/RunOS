DIR := $(shell pwd)
# USER_DIR := $(DIR)/user
OS_DIR := $(DIR)/os

all:
#	make build -C $(USER_DIR)
	make run -C $(OS_DIR)

debug:
	make debug -C $(OS_DIR)

gdb:
	make gdb -C $(OS_DIR)