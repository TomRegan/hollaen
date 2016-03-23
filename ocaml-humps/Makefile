PROG_BIN := bonjour fibonacci fizzbuzz duckquack sq
PROG_OBJ := $(wildcard *.o)
PROG_OBJ2 := $(wildcard *.cm*)
CC := ocamlbuild -no-links

.PHONY: all
all: $(PROG_BIN) copy-files

bonjour: src/bonjour.ml
	$(CC) src/bonjour.native src/bonjour.byte

fibonacci: src/fibonacci.ml
	$(CC) src/fibonacci.native src/fibonacci.byte

fizzbuzz: src/fizzbuzz.ml
	$(CC) src/fizzbuzz.native src/fizzbuzz.byte

duckquack: src/duckquack.ml
	$(CC) -use-ocamlfind -package unix src/duckquack.native src/duckquack.byte

sq: src/sq.ml
	$(CC) src/sq.native src/sq.byte

.PHONY: copy-files
copy-files:
	test -d bin/ || mkdir bin
	cp _build/src/*.native bin/

.PHONY: test
test: all
	ocamlbuild -use-ocamlfind -package oUnit src/test_fizzbuzz.native

.PHONY: clean
clean:
	ocamlbuild -clean
	test ! -d bin/ || $(RM) -rf bin/
