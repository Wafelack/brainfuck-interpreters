CC=gcc
CLFAGS=-W -Wall -Werror -Wextra
CFILES=src/main.c 
CBUILD=bin/main.o 
EXEC=interpreter.exe

$(EXEC) : $(CBUILD)
	gcc $(CBUILD) -o $(EXEC)

$(CBUILD) : $(CFILES)
	gcc -c src/main.c -o bin/main.o
