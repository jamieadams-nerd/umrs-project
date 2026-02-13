############################################
# UMRS SELinux Policy Build
############################################

POLICY_MODULES := \
    umrs \
    umrs_cui \
    cui_lei

############################################
# Default build
############################################

all: $(POLICY_MODULES:=.pp)

############################################
# Delegate to SELinux dev kit
############################################

%.pp:
	make -f /usr/share/selinux/devel/Makefile $@

############################################
# Install
############################################

install: all
	@for m in $(POLICY_MODULES); do \
	    echo "[INSTALL] $$m.pp"; \
	    semodule -i $$m.pp; \
	done

############################################
# Clean
############################################

clean:
	rm -f *.pp *.mod *.mod.fc
	rm -rf tmp

