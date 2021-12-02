#!/bin/bash

DAY=$1
P1=$1_part1
P2=$1_part2

mkdir -p "$DAY"

touch "$DAY/$P1.rs"
touch "$DAY/$P2.rs"

cat << EOF > "$DAY/Makefile"

.PHONY: all build clean fmt fmt-ci

all: fmt clean build

build:
	@echo building...$DAY
	@rustc $P1.rs
	@rustc $P2.rs

clean:
	@echo cleaning...
	@rm -f $P1 $P2

fmt:
	@echo "Formatting..."
	@rustfmt $P1.rs $P2.rs

fmt-ci:
	@echo "Formatting..."
	@rustfmt --check $P1.rs $P2.rs

EOF

printf "%s\n" "$DAY" >> .modules