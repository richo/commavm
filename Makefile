thing: src/main.rs $(wildcard src/*.rs)
	rustc -o ./$@ $<
