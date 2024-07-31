AX_ROOT ?= $(PWD)/.arceos

all: build

ax_root:
	@./scripts/set_ax_root.sh $(AX_ROOT)

user_apps:
	@cd ./apps && make ARCH=$(ARCH) build

build run justrun debug disasm clean:user_apps ax_root
	@make -C $(AX_ROOT) A=$(PWD) $@

.PHONY: all ax_root build run justrun debug disasm clean
