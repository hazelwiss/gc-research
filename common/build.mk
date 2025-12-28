MKFILEROOT := $(abspath $(dir $(word 2, $(MAKEFILE_LIST))))

include $(MKFILEROOT)/../lib/libogc2/gamecube_rules
include $(MKFILEROOT)/build-iso.mk

COPT		= -O3
CFLAGS		:= $(COPT) -g -Wall $(MACHDEP) $(INCLUDE) -I$(MKFILEROOT)/../lib/libogc2/include
CXXFLAGS	:= $(CFLAGS)
LDFLAGS		= -g $(MACHDEP) -Wl,-Map,$(notdir $@).map
LIBS		= -logc -lm -liso9660 

export DEPSDIR	:= $(CURDIR)/$(BUILD)
export LD		:= $(CC)
export LIBPATHS	:= -L$(MKFILEROOT)/../lib/libogc2/lib/cube
