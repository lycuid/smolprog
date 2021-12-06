NAME=xdstatus
BUILDDIR=./bin

BIN=$(BUILDDIR)/$(NAME)
PREFIX=/usr/local
BINPREFIX=$(PREFIX)/bin

INCLUDE=./include
LIBS=-lX11 -lpthread -lm
CFLAGS=-pedantic -Wall

build: clean include.o
	mkdir -p $(BUILDDIR)
	$(CC) $(CFLAGS) $(LIBS) -I$(INCLUDE) -o $(BIN) $(NAME).c *.o

run: build
	$(BUILDDIR)/$(NAME)

include.o:
	$(CC) $(CFLAGS) -I$(INCLUDE) -c $(INCLUDE)/*.c $(INCLUDE)/**/*.c

.PHONY: clean
clean:
	rm -rf $(BUILDDIR) *.o

install: $(BIN)
	mkdir -p $(DESTDIR)$(BINPREFIX)
	strip $(BIN)
	cp $(BIN) $(DESTDIR)$(BINPREFIX)/$(NAME)
	chmod 755 $(DESTDIR)$(BINPREFIX)/$(NAME)

uninstall:
	rm $(DESTDIR)$(BINPREFIX)/$(NAME)
