FILE     := .modules
RESOURCES :=$(file < $(FILE))

.PHONY: help fmt docs scaffold

help:
	@echo make [COMMAND]
	@echo
	@echo COMMANDS
	@echo "  build       -- Build days"
	@echo "  clean       -- Clean days"
	@echo "  fmt         -- Run formatters"
	@echo "  fmt-ci      -- Run formatters in CI"
	@echo "  scaffold    -- Run module scaffolding tool"

format: fmt

build: $(addsuffix .build,$(RESOURCES))

clean: $(addsuffix .clean,$(RESOURCES))

fmt: $(addsuffix .fmt,$(RESOURCES))

fmt-ci: $(addsuffix .fmt-ci,$(RESOURCES))

scaffold:
	@./bin/scaffold.sh

define make-rules

.PHONY: $1.build $1.clean $1.fmt $1.fmt-ci

$1.build:
	@echo "[🧼 Building$1]"
	@$(MAKE) -C $1 build --no-print-directory

$1.clean:
	@echo "[🧼 Cleaning$1]"
	@$(MAKE) -C $1 clean --no-print-directory

$1.fmt:
	@echo "[💄 Formatting$1]"
	@$(MAKE) -C $1 fmt --no-print-directory

$1.fmt-ci:
	@echo "[🔬 Checking Formatting$1]"
	@$(MAKE) -C $1 fmt --no-print-directory

endef
$(foreach component,$(RESOURCES), $(eval $(call make-rules, $(component))))