include ../../common/build.mk

build: fuzzer-main.dol

fuzzer-main.dol: fuzzer-main.elf
fuzzer-main.elf: fuzzer-main.o fuzzer.o inputsgen.o

clean:
	@rm -f *.elf *.dol *.o *.map *.d
