NAME=xdstatus
BUILDDIR=./bin

BIN=$(BUILDDIR)/$(NAME)
PREFIX=/usr/local
BINPREFIX=$(PREFIX)/bin

INCLUDE=./include
LIBS=-lX11 -lpthread -lm
CFLAGS=-pedantic -Wall
