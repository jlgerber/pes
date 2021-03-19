BINROOT := ~
LIBROOT := ~
BTYPE := release
LIBFIDDLE :=
ifeq ($(OS),Windows_NT)
	$(error "Windows_NT not supported")
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        EXTENSION := so
		LIBFIDDLE := patchelf --set-rpath '$$ORIGIN/../lib' $(BINROOT)/bin/pes &&  patchelf --set-rpath '$$ORIGIN' $(LIBROOT)/lib/librepo_finder.so \
		&& patchelf --set-rpath  '$$ORIGIN' $(LIBROOT)/lib/libmanifest_finder.so
    endif
    ifeq ($(UNAME_S),Darwin)
        EXTENSION := dylib
		LIBFIDDLE := install_name_tool -add_rpath $(LIBROOT)/lib $(BINROOT)/bin/pes
    endif
endif

build:
	cargo build --release
	
	cp target/$(BTYPE)/pes $(BINROOT)/bin/.
	cp target/$(BTYPE)/librepo_finder.$(EXTENSION) $(LIBROOT)/lib/.
	cp target/$(BTYPE)/libmanifest_finder.$(EXTENSION) $(LIBROOT)/lib/.
	cp $$(rustc --print sysroot)/lib/libstd-*.$(EXTENSION) $(LIBROOT)/lib/.
	- $(LIBFIDDLE)


clean:
	rm  -f target/$(BTYPE)/libpes* && rm -f target/$(BTYPE)/librepo* && rm -f target/$(BTYPE)/pes*

test:
	cargo test --release