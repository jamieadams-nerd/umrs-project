############################################
# UMRS SELinux Policy Build (Phase A)
#
# Builds the `umrs` policy module (.pp) for installation via
# `semodule -i`. Operator commands (semodule, semanage, restorecon)
# are PRINTED, not executed -- Jamie runs sudo; this Makefile does not.
############################################

# Installation prefix. Substituted into umrs.fc at build time.
# Override with: make PREFIX=/usr/local/umrs
PREFIX ?= /opt/umrs

# Phase A ships a single policy module.
POLICY_MODULES := umrs

PP_FILES := $(POLICY_MODULES:=.pp)

DEVEL_MAKEFILE := /usr/share/selinux/devel/Makefile

############################################
# Default build
############################################

.PHONY: all
all: $(PP_FILES)

############################################
# Generate umrs.fc from umrs.fc.in
#
# Substitutes @PREFIX@ with $(PREFIX). This is the false-root
# mechanism: no path is baked into the .te source.
############################################

umrs.fc: umrs.fc.in
	@echo "[GEN ]  umrs.fc  (PREFIX=$(PREFIX))"
	@sed -e 's|@PREFIX@|$(PREFIX)|g' umrs.fc.in > umrs.fc

############################################
# Build .pp via the refpolicy devel kit
#
# The devel Makefile consumes <name>.te, <name>.if, and <name>.fc
# in the current directory.
############################################

umrs.pp: umrs.te umrs.if umrs.fc
	@echo "[BUILD] umrs.pp"
	$(MAKE) -f $(DEVEL_MAKEFILE) umrs.pp

############################################
# Clean
############################################

.PHONY: clean
clean:
	rm -f $(PP_FILES) *.mod *.mod.fc umrs.fc
	rm -rf tmp

############################################
# Install / reload / fcontext hints
#
# These targets PRINT the sudo commands for Jamie to run.
# They do not invoke sudo.
############################################

.PHONY: install
install: all
	@echo
	@echo "Run as root to install the built policy package:"
	@echo "    sudo semodule -i umrs.pp"
	@echo
	@echo "Then apply file contexts under $(PREFIX):"
	@echo "    sudo restorecon -RF $(PREFIX)"

.PHONY: reload
reload: all
	@echo
	@echo "Run as root to reload the UMRS policy module:"
	@echo "    sudo semodule -r umrs || true"
	@echo "    sudo semodule -i umrs.pp"
	@echo "    sudo restorecon -RF $(PREFIX)"

.PHONY: load-fcontext
load-fcontext: umrs.fc
	@echo
	@echo "umrs.fc is bundled into umrs.pp by semodule_package;"
	@echo "a separate semanage fcontext run is only needed for"
	@echo "prefix-equivalence or path overrides, e.g.:"
	@echo
	@echo "    sudo semanage fcontext -a -e /opt/umrs $(PREFIX)"
	@echo "    sudo restorecon -RF $(PREFIX)"
