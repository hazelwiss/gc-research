.PHONY: build
build:
	@$(MAKE) -C lib
	@$(MAKE) -C tests

.PHONY: clean
clean:
	@$(MAKE) -C lib clean
	@$(MAKE) -C tests clean
