
.PHONY: all build clean fmt fmt-ci

all: fmt clean build

build:
	@echo building...day4
	@rustc day4_part1.rs
	@rustc day4_part2.rs

clean:
	@echo cleaning...
	@rm -f day4_part1 day4_part2

fmt:
	@echo "Formatting..."
	@rustfmt day4_part1.rs day4_part2.rs

fmt-ci:
	@echo "Formatting..."
	@rustfmt --check day4_part1.rs day4_part2.rs

