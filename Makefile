build:
	cargo build --release 
	install_name_tool -add_rpath $$(rustc --print sysroot)/lib target/release/pes
	cp target/release/pes ~/bin/.

clean:
	rm target/release/pes && rm target/release/librepo_finder.dylib
