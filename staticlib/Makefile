all: build

build: lib bin

lib:
	rustc --crate-type=staticlib lib.rs

bin:
	cc -lpthread main.c liblib.a

run: build
	./a.out
