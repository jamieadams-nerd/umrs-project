# Red Hat Enterprise Linux 10 Basic Setup


## Planning

* Storage Capcity 64gb
  * File system layout
  * Mounting options
* Network interface 2

## Installation

   * Start with FIPS enabled
   * Disable root and enable sudo
   * Setting timezone
   * Choose two network interfaces
     * Set hostname (don't leave as generic)
   * Creating a user with admin (wheel)
    * Unique accounts and generics (secadmin) are discouraged.
   * is this going to be a development platform?

   * ADd other packages:
     * mcstrans


## Post-Installation

Red Hat Enterprise Linux stuff:
* Ensure that gpgcheck is true on all repository defintions
* Install updates: sudo yum update
* Fix mounting options
* Reboot

For development systems:
  * Install rust: sudo dnf install rust  rust-toolset
  * Configure git client to include ssh keys for communicatig with project repository


  
## Controlled Unclassified Labels
Now, we can install the translation strings for the sensitivity level and categories assigned
to files and directories. In targeted policy, this have no impact to security. No enforcement 
is done based on the assignments. They are simply installed to idnetify the CUI assigned to the
file or directory.

In the future, when operating in MLS policy, they will be related to enforcement. But for now, do
not worry.

First make a copy of the system's translation configuration, by copying the 
/etc/selinux/targeted/setrans.conf file to some place safe.

Next, copy the rhel10/components/targeted/setrans.conf-TARGETED to the 
/etc/selinux/targeted/setrans.conf. Now, restart the mcstrans service.

```
sudo systemctl restart mcstrans
sudo systemctl status mcstrans
```

Now, context levels and categories can be assigned to files and directories. 




  * Update the setrans.conf
  * Install the umrs-mls-labels.json
  * Install the umrs-mls-state.json
 
     
  * 

  

