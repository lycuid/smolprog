NAME=smolprog
BIN=./target/release/$(NAME)
PREFIX=/usr/local
BINPREFIX=$(PREFIX)/bin

.PHONY: install
install: $(BIN)
	mkdir -p $(DESTDIR)$(BINPREFIX)
	strip $(BIN)
	cp -f $(BIN) $(DESTDIR)$(BINPREFIX)/$(NAME)
	chmod 755 $(DESTDIR)$(BINPREFIX)/$(NAME)

.PHONY: uninstall
uninstall:
	rm $(DESTDIR)$(BINPREFIX)/$(NAME)
