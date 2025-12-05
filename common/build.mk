include /opt/devkitpro/devkitPPC/gamecube_rules

MKFILEROOT := $(abspath $(dir $(word 2, $(MAKEFILE_LIST))))

COPT		= -O3
CFLAGS		:= $(COPT) -g -Wall $(MACHDEP) $(INCLUDE) -I$(MKFILEROOT)/../include -I/opt/devkitpro/libogc/include
CXXFLAGS	:= $(CFLAGS)
LDFLAGS		= -g $(MACHDEP) -Wl,-Map,$(notdir $@).map
LIBS		= -logc -lm

export DEPSDIR	:=	$(CURDIR)/$(BUILD)
export LD		:=	$(CC)
export LIBPATHS	:=	-L$(MKFILEROOT)/../lib/cube
export INCLUDE	:=	-I$(MKFILEROOT)/../include
