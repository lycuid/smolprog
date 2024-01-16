NAME:=smolprog
VERSION:=0.1.0
BIN:=$(NAME)
PREFIX:=/usr/local
BINPREFIX:=$(PREFIX)/bin

.PHONY: install uninstall
install:
	mkdir -p $(DESTDIR)$(BINPREFIX)
	strip $(BIN)
	cp -f $(BIN) $(DESTDIR)$(BINPREFIX)/$(NAME)

uninstall:
	$(RM) $(DESTDIR)$(BINPREFIX)/$(NAME)
