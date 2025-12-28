export PACKAGE_DIR = $(abspath .)/.pkg

.PHONY: build
build:
	@$(MAKE) -C lib/libogc2
	@$(MAKE) -C tests

.PHONY: setup
setup:
	@(cd tools && cargo run --bin dsp-gen-tests)

.PHONY: package
package:
	@rm -rf .pkg
	@mkdir .pkg
	@$(MAKE) -C tests package
	@tar czf packaged.tar.gz .pkg

.PHONY: clean
clean:
	@(find . \( -name "*.elf" -o -name "*.dol" -o -name "*.o" -o -name "*.map" -o -name "*.d" -o -name "*.a" \) -not -path "./lib/*" | xargs rm -f)
