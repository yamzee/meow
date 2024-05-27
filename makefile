.PHONY: cat catc rustycat gocat
all: cat catc rustycat gocat

cat:
	nasm -f elf64 cat.asm
	ld cat.o -o cat

catc:
	gcc cat.c -o catc

rustycat:
	rustc cat.rs -o rustycat

gocat:
	go build -o gocat cat.go

clean:
	rm cat cat.o catc rustycat gocat
