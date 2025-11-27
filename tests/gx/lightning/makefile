include ../../../common/build.mk

build: light-spotlight.dol light-diffuse.dol

light-spotlight.dol: light-spotlight.elf
light-spotlight.elf: light-spotlight.o

light-diffuse.dol: light-diffuse.elf
light-diffuse.elf: light-diffuse.o sphere.o

clean:
	@rm -f *.elf *.dol *.o *.map *.d
