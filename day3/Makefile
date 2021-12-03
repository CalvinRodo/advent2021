
.PHONY: all build clean fmt fmt-ci

all: fmt clean build

build:
	@echo building...day3
	@rustc day3_part1.rs
	@rustc day3_part2.rs

clean:
	@echo cleaning...
	@rm -f day3_part1 day3_part2

fmt:
	@echo "Formatting..."
	@rustfmt day3_part1.rs day3_part2.rs

fmt-ci:
	@echo "Formatting..."
	@rustfmt --check day3_part1.rs day3_part2.rs

