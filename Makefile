IDIR:=src
ODIR:=.build
MAIN:=$(ODIR)/run
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
LOAD_OBJS:=$(OBJS:$(ODIR)/%=(primitive-load-path "%"))

$(MAIN): $(OBJS)
	@echo "[+] writing shell script to load and exec compiled guile object files."
	@echo "#!/bin/sh"                                            >  $@
	@echo "exec guile -e '(smolprog)' -C \$$(dirname \$$0) -c '" >> $@
	@echo '$(LOAD_OBJS)' | sed -rn 's/\)\s/\)\n/pg'              >> $@
	@echo "'"                                                    >> $@
	@chmod 755 $@

$(ODIR)/%.scm.go: $(IDIR)/%.scm
	guild compile -O3 -L $(IDIR) -o $@ $^

.PHONY: install
install:
	cp -Tfr $(ODIR) $$SCRIPTS/smolprog

.PHONY: uninstall
uninstall:
	rm -rf $$SCRIPTS/smolprog

.PHONY: clean run
clean: ; rm -rf $(ODIR)
run: $(MAIN); $(MAIN)
