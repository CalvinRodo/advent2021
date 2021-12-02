
.PHONY: all build clean fmt fmt-ci

all: fmt clean build

build:
	@echo building...day2
	@rustc day2_part1.rs
	@rustc day2_part2.rs

clean:
	@echo cleaning...
	@rm -f day2_part1 day2_part2

fmt:
	@echo "Formatting..."
	@rustfmt day2_part1.rs day2_part2.rs

fmt-ci:
	@echo "Formatting..."
	@rustfmt --check day2_part1.rs day2_part2.rs

