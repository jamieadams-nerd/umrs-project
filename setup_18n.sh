mkdir -p resources/i18n/umrs-tester/{locale/en_US/LC_MESSAGES,locale/fr_FR/LC_MESSAGES}
touch resources/i18n/umrs-tester/umrs-tester.pot
touch resources/i18n/umrs-tester/en_US.po
touch resources/i18n/umrs-tester/fr_FR.po

# Check Makefile
#
#msgfmt -o resources/i18n/umrs-tester/locale/en_US/LC_MESSAGES/umrs-tester.mo resources/i18n/umrs-tester/en_US.po
#msgfmt -o resources/i18n/umrs-tester/locale/fr_FR/LC_MESSAGES/umrs-tester.mo resources/i18n/umrs-tester/fr_FR.po

