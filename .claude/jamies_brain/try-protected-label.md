
# Canadian Protected A 
- I have loaded s0:c200 in setrans.conf and restarted mcstrans
- chcat -L comamdn reveals that is loaded.
- NOTE: chcon won't work inside of our project directories because it is on a mount that isn't ext4.
- NOTE: What is loaded in the kernel is not the corrected label structure KNox made.
  - I wasn't sure which one was the corrected one. There were to in the umrs-uname crate.
  - I will laod the correct one. KNox should retire or archive the old wrong one or at least rename.
    I typically like the ALL caps version one in data/
- Test it in a safe place:
  - sudo -u umrs-agent -i
  - Create test files and chcon them to soemthing from chcat -L
  - Use ls -Z to see two test files already created.
  - touch somefile
  - chcon -l s0:c200 somefile
  - use exit when you're done.

