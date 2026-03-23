#!/usr/bin/bash
# Run umrs-uname with French Canadian locale for translation testing.
# Compiles .mo from .po, then launches with UMRS_LOCALEDIR override.

(cd ../../../ && make i18n-compile-umrs-uname)

export UMRS_LOCALEDIR=$(/usr/bin/realpath ../../../resources/i18n/umrs-uname)
export LANG=fr_CA.UTF-8
export LANGUAGE=fr_CA

cargo run -p umrs-uname

