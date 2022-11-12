IDIR:=src
ODIR:=.build
# order matters (should be in load order).
SRCS:=$(IDIR)/utils.scm                 \
      $(IDIR)/logger/battery.scm        \
      $(IDIR)/logger/cpu.scm            \
      $(IDIR)/logger/date.scm           \
      $(IDIR)/logger/memory.scm         \
      $(IDIR)/logger/network.scm        \
      $(IDIR)/logger/sessions.scm       \
      $(IDIR)/logger/volume.scm         \
      $(IDIR)/main.scm
OBJS:=$(SRCS:$(IDIR)/%=$(ODIR)/%.go)
LOAD_COMPILED:=(primitive-load-path "%")
GUILE_FLAGS:=-O3 -Wunbound-variable -Wformat -Warity-mismatch

build: $(OBJS)

run: build
	@guile -e '(smolprog)' -C . -c '$(OBJS:%=$(LOAD_COMPILED))'

$(ODIR)/%.scm.go: $(IDIR)/%.scm
	guild compile $(GUILE_FLAGS) -L $(IDIR) -o $@ $^

.PHONY: install
install:
	cp -Tfr $(ODIR) $(GUILE_LOAD_COMPILED_PATH)/smolprog

.PHONY: uninstall
uninstall:
	rm -rf $(GUILE_LOAD_COMPILED_PATH)/smolprog

.PHONY: clean run
clean: ; rm -rf $(ODIR)
