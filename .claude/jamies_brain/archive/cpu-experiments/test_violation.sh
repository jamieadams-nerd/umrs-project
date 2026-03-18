#!/usr/bin/sh
#
#

sudo sh -c 'echo "unauthorized change" >> /usr/lib/os-release'

cat /usr/lib/os-release
echo $?

sudo ausearch -m INTEGRITY_DATA -i

# Look for invalid-signature
#
#
