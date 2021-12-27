NAME=smolprog
BIN=./target/release/$(NAME)
PREFIX=/usr/local
BINPREFIX=$(PREFIX)/bin

.PHONY: build-all
build-all: fmt clean
	cargo build --release --all-features

.PHONY: build-default
build-default: fmt clean
	cargo build --release

.PHONY: build-no-default
build-no-default: fmt clean
	cargo build --release --no-default-features

.PHONY: build-with-args
build-with-args: fmt clean
	cargo build --release $(ARGS)

.PHONY: run
run: fmt
	cargo run $(ARGS)

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean

.PHONY: install
install: $(BIN)
	mkdir -p $(DESTDIR)$(BINPREFIX)
	strip $(BIN)
	cp $(BIN) $(DESTDIR)$(BINPREFIX)/$(NAME)
	chmod 755 $(DESTDIR)$(BINPREFIX)/$(NAME)

.PHONY: uninstall
uninstall:
	rm $(DESTDIR)$(BINPREFIX)/$(NAME)
