fix the umrs-selinux::fs_encrypt module. Given a mount point determine if the filesystem is
  encrypted or if the device is LUKS encrypted. Fix the two functions. replace
  fs::read_strings function to use any of our more secure options we wrote. EncryptionSource
  enum (should indicate) type of encryption: none, LUKS Device, from the filesystem like
  encryptfs?. the secure_dirent module and associated struct should have has_encryption or
  something. Lastly, fix umrs-ls to use the checks from secure_dirent and remove old code from
   umrs-ls. Ask me any questions. Give me the option ot save this as a plan to resume later in
   case I reach my usage limit.

