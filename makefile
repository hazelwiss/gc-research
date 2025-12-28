.PHONY: build
build:
	@$(MAKE) -C lib/libogc2
	@$(MAKE) -C tests

.PHONY: clean
clean:
	@(find . \( -name "*.elf" -o -name "*.dol" -o -name "*.o" -o -name "*.map" -o -name "*.d" -o -name "*.a" \) -not -path "./lib/*" | xargs rm -f)

.PHONY: setup
setup:
	@(cd tools && cargo run --bin dsp-gen-tests)

.PHONY: clear
clear:
	@rm -rf tests/dsp/gen
