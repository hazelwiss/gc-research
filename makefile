.PHONY: build
build: setup
	@$(MAKE) -C lib
	@$(MAKE) -C tests

.PHONY: clean
clean:
	@$(MAKE) -C lib clean
	@$(MAKE) -C tests clean

.PHONY: setup
setup:
	@(cd tools && cargo run --bin fuzzer-gen-tests)
