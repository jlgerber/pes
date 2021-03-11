BTYPE := release
LIBFIDDLE :=
ifeq ($(OS),Windows_NT)
	$(error "Windows_NT not supported")
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        EXTENSION := so
    endif
    ifeq ($(UNAME_S),Darwin)
        EXTENSION := dylib
		LIBFIDDLE := install_name_tool -add_rpath $$(rustc --print sysroot)/lib target/$(BTYPE)/pes
    endif
endif

build:
	cargo build --release 
	- $(LIBFIDDLE)
	cp target/$(BTYPE)/pes ~/bin/.

clean:
	rm  -f target/$(BTYPE)/libpes* && rm -f target/$(BTYPE)/librepo* && rm -f target/$(BTYPE)/pes*
