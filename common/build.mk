include $(DEVKITPPC)/gamecube_rules

MKFILEROOT := $(abspath $(dir $(word 2, $(MAKEFILE_LIST))))

COPT		= -O3
CFLAGS		:= $(COPT) -g -Wall $(MACHDEP) $(INCLUDE) -I$(MKFILEROOT)/../include -I$(LIBOGC_INC)
CXXFLAGS	:= $(CFLAGS)
LDFLAGS		= -g $(MACHDEP) -Wl,-Map,$(notdir $@).map
LIBS		= -logc -ldspemit -lm

export DEPSDIR	:= $(CURDIR)/$(BUILD)
export LD		:= $(CC)
export LIBPATHS	:= -L$(LIBOGC_LIB) -L$(MKFILEROOT)/../lib/dsp
