NAME=xdstatus
BIN=./target/release/$(NAME)
PREFIX=/usr/local
BINPREFIX=$(PREFIX)/bin

build: fmt clean
	cargo build --release $(ARGS)

run: fmt
	cargo run $(ARGS)

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean

install: $(BIN)
	mkdir -p $(DESTDIR)$(BINPREFIX)
	strip $(BIN)
	cp $(BIN) $(DESTDIR)$(BINPREFIX)/$(NAME)
	chmod 755 $(DESTDIR)$(BINPREFIX)/$(NAME)

uninstall:
	rm $(DESTDIR)$(BINPREFIX)/$(NAME)
