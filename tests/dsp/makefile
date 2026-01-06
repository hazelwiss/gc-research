build:
	@$(MAKE) -C apps -f build-wii
	@$(MAKE) -C apps -f build-gc

package: build
	@$(MAKE) -C apps -f build-gc package
