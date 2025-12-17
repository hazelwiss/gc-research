MAKEFILES := $(shell find . -mindepth 2 -name makefile)
MAKEFILES := $(MAKEFILES) $(shell find . -mindepth 2 -name "*.mk")

build:
	@for i in $(MAKEFILES); do $(MAKE) -C `dirname $$i` || exit 1; done;

clean:
	@for i in $(MAKEFILES); do $(MAKE) -C `dirname $$i` clean || exit 1; done;

