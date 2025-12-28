build:
	@$(MAKE) -C .gen -f build-gc
	@$(MAKE) -C .gen -f build-wii

package: build
	@$(MAKE) -C .gen -f build-gc package
