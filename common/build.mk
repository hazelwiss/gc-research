MKFILEROOT := $(abspath $(dir $(word 2, $(MAKEFILE_LIST))))

ifeq ($(origin Wii),undefined)
include $(MKFILEROOT)/build-gc.mk
else
include $(MKFILEROOT)/build-wii.mk
endif
