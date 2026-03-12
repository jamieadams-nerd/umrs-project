Color configuration is not in setrans.conf.

It lives in:

/etc/selinux/mls/setrans.conf
/etc/selinux/mls/setrans.d/*.conf


and optionally:

/etc/selinux/mls/color.conf


On most modern systems:

color.conf is missing

or empty

or not shipped at all

When that happens, mcstransd logs exactly what you saw.
