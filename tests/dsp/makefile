include ../../common/build.mk


build: disasm.o
	@$(MAKE) -C apps -f build-wii
	@$(MAKE) -C apps -f build-gc

disasm.o: disasm.c

package: build
	@$(MAKE) -C apps -f build-gc package
