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


  
* UMRS MLS
  * Update the setrans.conf
  * Install the umrs-mls-labels.json
  * Install the umrs-mls-state.json
 
     
  * 

  

