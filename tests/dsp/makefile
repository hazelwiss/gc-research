include ../../common/build.mk

build: fuzzer.dol

fuzzer.dol: fuzzer.elf
fuzzer.elf: fuzzer.o

clean:
	@rm -f *.elf *.dol *.o *.map *.d
