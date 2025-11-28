include ../../../common/build.mk

build: light-spotlight.dol light-diffuse.dol light-specular.dol light-ambient.dol

light-spotlight.dol: light-spotlight.elf
light-spotlight.elf: light-spotlight.o

light-diffuse.dol: light-diffuse.elf
light-diffuse.elf: light-diffuse.o models.o

light-specular.dol: light-specular.elf
light-specular.elf: light-specular.o models.o

light-ambient.dol: light-ambient.elf
light-ambient.elf: light-ambient.o models.o

clean:
	@rm -f *.elf *.dol *.o *.map *.d
