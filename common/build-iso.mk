MKFILEROOT := $(abspath $(dir $(word 2, $(MAKEFILE_LIST))))

%.iso:
	@(cd $(MKFILEROOT)/../tools && cargo run --bin iso -- $(abspath $@) $(abspath $<) $(MKFILEROOT)/../misc/gbi.hdr $(abspath $(wordlist 2, $(words $^), $^)))
	@echo packing iso ['$@'] with executable ['$<'] and fst ['$(wordlist 2, $(words $^), $^)']
