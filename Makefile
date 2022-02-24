DIR := $(shell pwd)
# USER_DIR := $(DIR)/user
OS_DIR := $(DIR)/os

all:
#	make build -C $(USER_DIR)
	make run -C $(OS_DIR)

debug:
	make debug -C $(OS_DIR)