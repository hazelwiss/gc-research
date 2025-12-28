MAKEFILES := $(shell find . -mindepth 2 -name makefile)
MAKEFILES := $(MAKEFILES) $(shell find . -mindepth 2 -name "*.mk")

build:
	@for i in $(MAKEFILES); do $(MAKE) -C `dirname $$i` -f `basename $$i` || exit 1; done;

package:
	@for i in $(MAKEFILES); do $(MAKE) -C `dirname $$i` -f `basename $$i` package || exit 1; done;
