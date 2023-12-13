NAME:=smolprog
VERSION:=0.1.0
BIN:=$(NAME)
PREFIX:=/usr/local
BINPREFIX:=$(PREFIX)/bin

$(BIN):
	go build

run: $(BIN)
	./$(BIN)

.PHONY: install uninstall
install: $(BIN)
	mkdir -p $(DESTDIR)$(BINPREFIX)
	strip $(BIN)
	cp -f $(BIN) $(DESTDIR)$(BINPREFIX)/$(NAME)

uninstall:
	$(RM) $(DESTDIR)$(BINPREFIX)/$(NAME)
