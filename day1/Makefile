
.PHONY: all build clean fmt fmt-ci

all: fmt clean build


build:
	@echo building...day1.rs
	@rustc day1.rs

clean:
	@echo cleaning...
	@rm -f day1

fmt:
	@echo "Formatting..."
	@rustfmt day1.rs

fmt-ci:
	@echo "Formatting..."
	@rustfmt --check day1.rs

