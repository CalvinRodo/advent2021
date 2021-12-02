#!/bin/bash

DAY=$1

mkdir -p "$DAY"

touch "$DAY/$DAY.rs"

cat << EOF > "$DAY/Makefile"

.PHONY: all build clean fmt fmt-ci

all: fmt clean build

build:
	@echo building...$DAY.rs
	@rustc $DAY.rs

clean:
	@echo cleaning...
	@rm $DAY

fmt:
	@echo "Formatting..."
	@rustfmt $DAY.rs

fmt-ci:
	@echo "Formatting..."
	@rustfmt --check $DAY.rs

EOF

printf "%s\n" "$DAY1" >> .modules