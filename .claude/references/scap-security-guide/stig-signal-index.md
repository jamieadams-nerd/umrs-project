# RHEL 10 STIG — Signal Index

**Source:** `rhel10-playbook-stig.yml` (SCAP Security Guide)
**Profile:** xccdf_org.ssgproject.content_profile_stig
**Signals:** 451

| Signal Name | CCE | NIST Controls | Severity | Description | Check Method |
|---|---|---|---|---|---|
| `account_disable_post_pw_expiration` | CCE-88966-7 | AC-2(3), CM-6(a), IA-4(e) | medium | Set Account Expiration Following Inactivity | other |
| `account_password_pam_faillock_password_auth` | CCE-87657-3 | AC-7 (a) | medium | Configure the Use of the pam_faillock.so Module in the /etc/pam.d/password-auth | other |
| `account_password_pam_faillock_system_auth` | CCE-88810-7 | AC-7 (a) | medium | Configure the Use of the pam_faillock.so Module in the /etc/pam.d/system-auth | other |
| `account_password_selinux_faillock_dir` | CCE-90568-7 | AC-7 (a) | medium | An SELinux Context must be configured for the pam_faillock.so records directory | other |
| `accounts_have_homedir_login_defs` | CCE-88604-4 |  | medium | Insert correct line to /etc/login.defs | other |
| `accounts_logon_fail_delay` | CCE-86822-4 | AC-7(b), CM-6(a) | medium | Set accounts logon fail delay | other |
| `accounts_max_concurrent_login_sessions` | CCE-90751-9 | AC-10, CM-6(a) | low | Find /etc/security/limits.d files containing maxlogins configuration | other |
| `accounts_maximum_age_login_defs` | CCE-87961-9 | CM-6(a), IA-5(1)(d), IA-5(f) | medium | Set Password Maximum Age | other |
| `accounts_minimum_age_login_defs` | CCE-89307-3 | CM-6(a), IA-5(1)(d), IA-5(f) | medium | Set Password Minimum Age | other |
| `accounts_no_uid_except_zero` | CCE-87552-6 | AC-6(5), IA-2, IA-4(b) | high | Get all /etc/passwd file entries | other |
| `accounts_password_pam_dcredit` | CCE-89089-7 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Minimum Digit Characters - Find | other |
| `accounts_password_pam_dictcheck` | CCE-88171-4 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Prevent the Use of Dictionary | other |
| `accounts_password_pam_difok` | CCE-90363-3 | CM-6(a), IA-5(1)(b), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Minimum Different Characters | other |
| `accounts_password_pam_enforce_root` | CCE-90134-8 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Enforce for root User | other |
| `accounts_password_pam_lcredit` | CCE-90276-7 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Minimum Lowercase Characters | other |
| `accounts_password_pam_maxclassrepeat` | CCE-88844-6 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Maximum Consecutive Repeating | other |
| `accounts_password_pam_maxrepeat` | CCE-88015-3 | CM-6(a), IA-5(4), IA-5(c) | medium | Set Password Maximum Consecutive Repeating Characters - Find pwquality.conf.d | other |
| `accounts_password_pam_minclass` | CCE-87289-5 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Minimum Different Categories | other |
| `accounts_password_pam_minlen` | CCE-87852-0 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Minimum Length - Find pwquality.conf | other |
| `accounts_password_pam_ocredit` | CCE-89297-6 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Minimum Special Characters - | other |
| `accounts_password_pam_pwquality_password_auth` | CCE-89505-2 |  | medium | Ensure PAM password complexity module is enabled in password-auth - Check | other |
| `accounts_password_pam_pwquality_retry` | CCE-90663-6 |  | medium | Ensure PAM Enforces Password Requirements - Authentication Retry Prompts | other |
| `accounts_password_pam_pwquality_system_auth` | CCE-89362-8 |  | medium | Ensure PAM password complexity module is enabled in system-auth - Check | other |
| `accounts_password_pam_ucredit` | CCE-89959-1 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | medium | Ensure PAM Enforces Password Requirements - Minimum Uppercase Characters | other |
| `accounts_password_set_max_life_existing` | CCE-87137-6 | CM-6(a), IA-5(1)(d), IA-5(f) | medium | Collect users with not correct maximum time period between password changes | other |
| `accounts_password_set_min_life_existing` | CCE-87953-6 | CM-6(a), IA-5(1)(d), IA-5(f) | medium | Collect users with not correct minimum time period between password changes | other |
| `accounts_passwords_pam_faillock_audit` | CCE-90730-3 | AC-7 (a) | medium | Account Lockouts Must Be Logged - Check if system relies on authselect tool | other |
| `accounts_passwords_pam_faillock_deny` | CCE-87388-5 | AC-7(a), CM-6(a) | medium | Lock Accounts After Failed Password Attempts - Check if system relies on | other |
| `accounts_passwords_pam_faillock_deny_root` | CCE-87975-9 | AC-7(b), CM-6(a), IA-5(c) | medium | Configure the root Account for Failed Password Attempts - Check if system | other |
| `accounts_passwords_pam_faillock_dir` | CCE-90182-7 | AC-7(a), AC-7(b), AC-7.1(ii) | medium | Lock Accounts Must Persist - Ensure necessary SELinux packages are installed | other |
| `accounts_passwords_pam_faillock_interval` | CCE-86672-3 | AC-7(a), CM-6(a) | medium | Set Interval For Counting Failed Password Attempts - Check if system relies | other |
| `accounts_passwords_pam_faillock_unlock_time` | CCE-89250-5 | AC-7(b), CM-6(a) | medium | Set Lockout Time for Failed Password Attempts - Check if system relies on | other |
| `accounts_tmout` | CCE-88163-1 | AC-12, AC-2(5), CM-6(a), SC-10 | medium | Correct any occurrence of TMOUT in /etc/profile | other |
| `accounts_umask_etc_bashrc` | CCE-88580-6 | AC-6(1), CM-6(a) | medium | Check if umask in /etc/bashrc is already set | other |
| `accounts_umask_etc_csh_cshrc` | CCE-90597-6 | AC-6(1), CM-6(a) | medium | Check if umask in /etc/csh.cshrc is already set | other |
| `accounts_umask_etc_login_defs` | CCE-89314-9 | AC-6(1), CM-6(a) | medium | Check if UMASK is already set | other |
| `accounts_umask_etc_profile` | CCE-87651-6 | AC-6(1), CM-6(a) | medium | Ensure the Default Umask is Set Correctly in /etc/profile - Locate Profile | other |
| `accounts_umask_interactive_users` | CCE-87122-8 |  | medium | Ensure the Default Umask is Set Correctly For Interactive Users - Get interactiv | other |
| `accounts_user_dot_no_world_writable_programs` | CCE-90449-0 |  | medium | User Initialization Files Must Not Run World-Writable Programs - Initialize | other |
| `accounts_user_interactive_home_directory_defined` | CCE-89933-6 |  | medium | Get all local users from /etc/passwd | other |
| `accounts_user_interactive_home_directory_exists` | CCE-86659-0 |  | medium | Get all local users from /etc/passwd | other |
| `aide_build_database` | CCE-86942-0 | CM-6(a) | medium | Build and Test AIDE Database - Ensure AIDE Is Installed | other |
| `aide_check_audit_tools` | CCE-86441-3 | AU-9(3), AU-9(3).1 | medium | Ensure aide is installed | other |
| `aide_periodic_cron_checking` | CCE-86738-2 | CM-6(a), SI-7, SI-7(1) | medium | Ensure AIDE is installed | other |
| `aide_scan_notification` | CCE-90177-7 | CM-3(5), CM-6(a) | medium | Ensure AIDE is installed | other |
| `aide_use_fips_hashes` | CCE-90260-1 | CM-6(a), SI-7, SI-7(1) | medium | Configure AIDE to Use FIPS 140-2 for Validating Hashes - Ensure aide is | other |
| `aide_verify_acls` | CCE-89640-7 | CM-6(a), SI-7, SI-7(1) | low | Get rules groups | other |
| `aide_verify_ext_attributes` | CCE-89625-8 | CM-6(a), SI-7, SI-7(1) | low | Get rules groups | other |
| `audit_privileged_commands_init` | CCE-88214-2 | AU-12(c) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - init | other |
| `audit_privileged_commands_poweroff` | CCE-86744-0 | AU-12(c) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - poweroff | other |
| `audit_privileged_commands_reboot` | CCE-88843-8 | AU-12(c) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - reboot | other |
| `audit_privileged_commands_shutdown` | CCE-88922-0 | AU-12(c) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - shutdown | other |
| `audit_rules_dac_modification_chmod` | CCE-90466-4 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit chmod tasks | audit-rule |
| `audit_rules_dac_modification_chown` | CCE-89540-9 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit chown tasks | audit-rule |
| `audit_rules_dac_modification_fchmod` | CCE-88200-1 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit fchmod tasks | audit-rule |
| `audit_rules_dac_modification_fchmodat` | CCE-89356-0 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit fchmodat tasks | audit-rule |
| `audit_rules_dac_modification_fchmodat2` | CCE-86535-2 |  | medium | Set architecture for audit fchmodat2 tasks | audit-rule |
| `audit_rules_dac_modification_fchown` | CCE-90685-9 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit fchown tasks | audit-rule |
| `audit_rules_dac_modification_fchownat` | CCE-90651-1 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit fchownat tasks | audit-rule |
| `audit_rules_dac_modification_fremovexattr` | CCE-88352-0 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit fremovexattr tasks | audit-rule |
| `audit_rules_dac_modification_fsetxattr` | CCE-89370-1 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit fsetxattr tasks | audit-rule |
| `audit_rules_dac_modification_lchown` | CCE-88243-1 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit lchown tasks | audit-rule |
| `audit_rules_dac_modification_lremovexattr` | CCE-90100-9 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit lremovexattr tasks | audit-rule |
| `audit_rules_dac_modification_lsetxattr` | CCE-88052-6 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit lsetxattr tasks | audit-rule |
| `audit_rules_dac_modification_removexattr` | CCE-89677-9 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit removexattr tasks | audit-rule |
| `audit_rules_dac_modification_setxattr` | CCE-89571-4 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit setxattr tasks | audit-rule |
| `audit_rules_dac_modification_umount` | CCE-87601-1 |  | medium | Add the audit rule to {{ audit_file }} | audit-rule |
| `audit_rules_dac_modification_umount2` | CCE-89822-1 |  | medium | Set architecture for audit umount2 tasks | audit-rule |
| `audit_rules_execution_chacl` | CCE-88467-6 |  | medium | Record Any Attempts to Run chacl - Set architecture for audit /usr/bin/chacl | audit-rule |
| `audit_rules_execution_chcon` | CCE-87762-1 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Any Attempts to Run chcon - Set architecture for audit /usr/bin/chcon | audit-rule |
| `audit_rules_execution_semanage` | CCE-89541-7 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Any Attempts to Run semanage - Set architecture for audit /usr/sbin/seman | audit-rule |
| `audit_rules_execution_setfacl` | CCE-87662-3 |  | medium | Record Any Attempts to Run setfacl - Set architecture for audit /usr/bin/setfacl | audit-rule |
| `audit_rules_execution_setfiles` | CCE-88818-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Any Attempts to Run setfiles - Set architecture for audit /usr/sbin/setfi | audit-rule |
| `audit_rules_execution_setsebool` | CCE-87741-5 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Any Attempts to Run setsebool - Set architecture for audit /usr/sbin/sets | audit-rule |
| `audit_rules_file_deletion_events_rename` | CCE-90733-7 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit rename tasks | audit-rule |
| `audit_rules_file_deletion_events_renameat` | CCE-90237-9 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit renameat tasks | audit-rule |
| `audit_rules_file_deletion_events_renameat2` | CCE-86188-0 |  | medium | Set architecture for audit renameat2 tasks | audit-rule |
| `audit_rules_file_deletion_events_rmdir` | CCE-88762-0 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit rmdir tasks | audit-rule |
| `audit_rules_file_deletion_events_unlink` | CCE-86737-4 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit unlink tasks | audit-rule |
| `audit_rules_file_deletion_events_unlinkat` | CCE-87813-2 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit unlinkat tasks | audit-rule |
| `audit_rules_immutable` | CCE-89816-3 | AC-6(9), CM-6(a) | medium | Make the auditd Configuration Immutable - Collect all files from /etc/audit/rule | audit-rule |
| `audit_rules_kernel_module_loading_delete` | CCE-89982-3 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on Kernel Module Unloading - delete_module | audit-rule |
| `audit_rules_kernel_module_loading_finit` | CCE-88638-2 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on Kernel Module Loading and Unloading | audit-rule |
| `audit_rules_kernel_module_loading_init` | CCE-90172-8 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on Kernel Module Loading - init_module | audit-rule |
| `audit_rules_login_events_faillock` | CCE-89479-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Attempts to Alter Logon and Logout Events - faillock - Check if watch | audit-rule |
| `audit_rules_login_events_lastlog` | CCE-88938-6 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Attempts to Alter Logon and Logout Events - lastlog - Check if watch | audit-rule |
| `audit_rules_login_events_tallylog` | CCE-88948-5 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Attempts to Alter Logon and Logout Events - tallylog - Check if watch | audit-rule |
| `audit_rules_media_export` | CCE-86590-7 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit mount tasks | audit-rule |
| `audit_rules_privileged_commands_chage` | CCE-90143-9 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - chage | audit-rule |
| `audit_rules_privileged_commands_chsh` | CCE-89551-6 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - chsh | audit-rule |
| `audit_rules_privileged_commands_crontab` | CCE-89029-3 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - crontab | audit-rule |
| `audit_rules_privileged_commands_gpasswd` | CCE-89403-0 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - gpasswd | audit-rule |
| `audit_rules_privileged_commands_kmod` | CCE-86727-5 | AU-12(a), AU-12.1(ii), AU-12.1(iv)AU-12(c), AU-3, AU-3.1, MA-4(1)(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - kmod | audit-rule |
| `audit_rules_privileged_commands_modprobe` | CCE-89893-2 | AU-12(a), AU-12(c), AU-12.1(ii), AU-12.1(iv), AU-3, AU-3.1, MA-4(1)(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - modprobe | audit-rule |
| `audit_rules_privileged_commands_mount` | CCE-87814-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - mount | audit-rule |
| `audit_rules_privileged_commands_newgrp` | CCE-88752-1 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - newgrp | audit-rule |
| `audit_rules_privileged_commands_pam_timestamp_check` | CCE-89521-9 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - pam_times | audit-rule |
| `audit_rules_privileged_commands_passwd` | CCE-89215-8 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - passwd | audit-rule |
| `audit_rules_privileged_commands_pkexec` | CCE-89134-1 |  | medium | Ensure auditd Collects Information on the Use of Privileged Commands - pkexec | audit-rule |
| `audit_rules_privileged_commands_postdrop` | CCE-89394-1 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - postdrop | audit-rule |
| `audit_rules_privileged_commands_postqueue` | CCE-87927-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - postqueue | audit-rule |
| `audit_rules_privileged_commands_rmmod` | CCE-88804-0 |  | medium | Ensure auditd Collects Information on the Use of Privileged Commands - rmmod | audit-rule |
| `audit_rules_privileged_commands_ssh_agent` | CCE-90081-1 |  | medium | Record Any Attempts to Run ssh-agent - Set architecture for audit /usr/bin/ssh-a | audit-rule |
| `audit_rules_privileged_commands_ssh_keysign` | CCE-88874-3 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - ssh-keysi | audit-rule |
| `audit_rules_privileged_commands_su` | CCE-89587-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - su | audit-rule |
| `audit_rules_privileged_commands_sudo` | CCE-89698-5 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - sudo | audit-rule |
| `audit_rules_privileged_commands_sudoedit` | CCE-89601-9 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - sudoedit | audit-rule |
| `audit_rules_privileged_commands_umount` | CCE-86962-8 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - umount | audit-rule |
| `audit_rules_privileged_commands_unix_chkpwd` | CCE-89529-2 | AC-2(4), AC-6(9), AU-12(a), AU-12(c), AU-12.1(ii), AU-12.1(iv), AU-2(d), AU-3, AU-3.1, CM-6(a), MA-4(1)(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - unix_chkp | audit-rule |
| `audit_rules_privileged_commands_unix_update` | CCE-86620-2 |  | medium | Ensure auditd Collects Information on the Use of Privileged Commands - unix_upda | audit-rule |
| `audit_rules_privileged_commands_userhelper` | CCE-90652-9 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Ensure auditd Collects Information on the Use of Privileged Commands - userhelpe | audit-rule |
| `audit_rules_privileged_commands_usermod` | CCE-87659-9 |  | medium | Ensure auditd Collects Information on the Use of Privileged Commands - usermod | audit-rule |
| `audit_rules_sudoers` | CCE-88688-7 |  | medium | Ensure auditd Collects System Administrator Actions - /etc/sudoers - Check | audit-rule |
| `audit_rules_sudoers_d` | CCE-89020-2 |  | medium | Ensure auditd Collects System Administrator Actions - /etc/sudoers.d/ - | audit-rule |
| `audit_rules_suid_privilege_function` | CCE-88933-7 | AC-6(9), AU-12(3), AU-7(a), AU-7(b), AU-8(b), CM-5(1) | medium | Set suid_audit_rules fact | audit-rule |
| `audit_rules_system_shutdown` | CCE-87352-1 | AU-5(b), CM-6(a), SC-24 | medium | Collect all files from /etc/audit/rules.d with .rules extension | audit-rule |
| `audit_rules_unsuccessful_file_modification_creat` | CCE-87052-7 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit creat tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_ftruncate` | CCE-86729-1 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit ftruncate tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_open` | CCE-87349-7 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit open tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_open_by_handle_at` | CCE-90251-0 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit open_by_handle_at tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_openat` | CCE-89291-9 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit openat tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_rename` | CCE-89713-2 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit rename tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_renameat` | CCE-88132-6 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit renameat tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_truncate` | CCE-89869-2 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit truncate tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_unlink` | CCE-88520-2 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit unlink tasks | audit-rule |
| `audit_rules_unsuccessful_file_modification_unlinkat` | CCE-89972-4 | AU-12(c), AU-2(d), CM-6(a) | medium | Set architecture for audit unlinkat tasks | audit-rule |
| `audit_rules_usergroup_modification_group` | CCE-87111-1 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Events that Modify User/Group Information - /etc/group - Check if | audit-rule |
| `audit_rules_usergroup_modification_gshadow` | CCE-87736-5 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Events that Modify User/Group Information - /etc/gshadow - Check | audit-rule |
| `audit_rules_usergroup_modification_opasswd` | CCE-90664-4 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Events that Modify User/Group Information - /etc/security/opasswd | audit-rule |
| `audit_rules_usergroup_modification_passwd` | CCE-88286-0 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Events that Modify User/Group Information - /etc/passwd - Check if | audit-rule |
| `audit_rules_usergroup_modification_shadow` | CCE-88637-4 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | medium | Record Events that Modify User/Group Information - /etc/shadow - Check if | audit-rule |
| `auditd_data_retention_action_mail_acct` | CCE-89081-4 | AU-5(2), AU-5(a), CM-6(a), IA-5(1) | medium | Configure auditd mail_acct Action on Low Disk Space - Configure auditd mail_acct | other |
| `auditd_data_retention_admin_space_left_action` | CCE-89040-0 | AU-5(1), AU-5(2), AU-5(4), AU-5(b), CM-6(a) | medium | Configure auditd admin_space_left Action on Low Disk Space | other |
| `auditd_data_retention_admin_space_left_percentage` | CCE-88585-5 | AU-5(1), AU-5(2), AU-5(4), AU-5(b), CM-6(a) | medium | Configure auditd admin_space_left on Low Disk Space | other |
| `auditd_data_retention_space_left_action` | CCE-88897-4 | AU-5(1), AU-5(2), AU-5(4), AU-5(b), CM-6(a) | medium | Configure auditd space_left Action on Low Disk Space | other |
| `auditd_data_retention_space_left_percentage` | CCE-88619-2 | AU-5(1), AU-5(2), AU-5(4), AU-5(b), CM-6(a) | medium | Configure auditd space_left on Low Disk Space | other |
| `auditd_freq` | CCE-87482-6 | CM-6 | medium | Insert correct line to /etc/audit/auditd.conf | other |
| `auditd_local_events` | CCE-88064-1 | CM-6 | medium | Insert correct line to /etc/audit/auditd.conf | other |
| `auditd_log_format` | CCE-88921-2 | AU-3, CM-6 | low | Insert correct line to /etc/audit/auditd.conf | other |
| `auditd_name_format` | CCE-87429-7 | AU-3, CM-6 | medium | Set type of computer node name logging in audit logs - Define Value to Be | other |
| `auditd_overflow_action` | CCE-87003-0 | AU-4(1) | medium | Insert correct line to /etc/audit/auditd.conf | other |
| `auditd_write_logs` | CCE-88724-0 | CM-6 | medium | Insert correct line to /etc/audit/auditd.conf | other |
| `banner_etc_issue` | CCE-88261-3 | AC-8(a), AC-8(c) | medium | Modify the System Login Banner - Ensure Correct Banner | other |
| `chrony_set_nts` | CCE-86471-0 |  | medium | Configure Time Service to use NTS - Check That /etc/ntp.conf Exist | other |
| `chronyd_client_only` | CCE-89002-0 | AU-12(1), AU-8(1) | low | Insert correct line to /etc/chrony.conf | other |
| `chronyd_no_chronyc_network` | CCE-87066-7 | CM-7(1) | low | Insert correct line to /etc/chrony.conf | other |
| `chronyd_or_ntpd_set_maxpoll` | CCE-88549-1 | AU-12(1), AU-8(1)(b), CM-6(a) | medium | Configure Time Service Maxpoll Interval - Check That /etc/ntp.conf Exist | other |
| `chronyd_specify_remote_server` | CCE-86811-7 | AU-8(1)(a), CM-6(a) | medium | Detect if chrony is already configured with pools or servers | other |
| `clean_components_post_updating` | CCE-88515-2 | CM-11(a), CM-11(b), CM-6(a), SI-2(6) | low | Ensure dnf Removes Previous Package Versions - Ensure DNF Removes Previous | other |
| `configure_bind_crypto_policy` | CCE-86874-5 | SC-12(2), SC-12(3), SC-13 | high | Configure BIND to use System Crypto Policy - Check BIND configuration file | other |
| `configure_crypto_policy` | CCE-89085-5 | AC-17(2), AC-17(a), CM-6(a), MA-4(6), SC-12(2), SC-12(3), SC-13 | high | Configure System Cryptography Policy - Check current crypto policy (runtime) | other |
| `configure_kerberos_crypto_policy` | CCE-88640-8 | SC-12(2), SC-12(3), SC-13 | high | Configure Kerberos to use System Crypto Policy | other |
| `configure_libreswan_crypto_policy` | CCE-88687-9 | CM-6(a), MA-4(6), SC-12(2), SC-12(3), SC-13 | high | Configure Libreswan to use System Crypto Policy | other |
| `configure_opensc_card_drivers` | CCE-90065-4 | CM-6(a), IA-2(1), IA-2(11), IA-2(2), IA-2(3), IA-2(4), IA-2(6), IA-2(7) | medium | Check existence of opensc conf | other |
| `configure_usbguard_auditbackend` | CCE-87152-5 | AU-2, CM-8(3), IA-3 | low | Insert correct line to /etc/usbguard/usbguard-daemon.conf | other |
| `coredump_disable_backtraces` | CCE-88825-5 | CM-6 | medium | Disable core dump backtraces - Search for a section in files | other |
| `coredump_disable_storage` | CCE-88732-3 | CM-6 | medium | Disable storing core dump - Search for a section in files | other |
| `dconf_db_up_to_date` | CCE-86609-5 |  | high | Make sure that the dconf databases are up-to-date with regards to respective | other |
| `dconf_gnome_banner_enabled` | CCE-87417-2 | AC-8(a), AC-8(b), AC-8(c) | medium | Enable GNOME3 Login Warning Banner | other |
| `dconf_gnome_disable_automount_open` | CCE-86628-5 | CM-6(a), CM-7(a), CM-7(b) | medium | Disable GNOME3 Automounting - automount-open | other |
| `dconf_gnome_disable_autorun` | CCE-87588-0 | CM-6(a), CM-7(a), CM-7(b) | low | Disable GNOME3 Automounting - autorun-never | other |
| `dconf_gnome_disable_ctrlaltdel_reboot` | CCE-90658-6 | AC-6(1), CM-6(a), CM-7(b) | high | Disable Ctrl-Alt-Del Reboot Key Sequence in GNOME3 | other |
| `dconf_gnome_disable_restart_shutdown` | CCE-87837-1 | AC-6(1), CM-6(a), CM-7(b) | high | Disable the GNOME3 Login Restart and Shutdown Buttons | other |
| `dconf_gnome_disable_user_list` | CCE-87918-9 | AC-23, CM-6(a) | medium | Disable the GNOME3 Login User List | other |
| `dconf_gnome_lock_screen_on_smartcard_removal` | CCE-87751-4 |  | medium | Detect if removal-action can be found on /etc/dconf/db/local.d/ | other |
| `dconf_gnome_login_banner_text` | CCE-88901-4 | AC-8(a), AC-8(c) | medium | Set the GNOME3 Login Warning Banner Text | other |
| `dconf_gnome_screensaver_idle_delay` | CCE-87170-7 | AC-11(a), CM-6(a) | medium | Set GNOME3 Screensaver Inactivity Timeout | other |
| `dconf_gnome_screensaver_lock_delay` | CCE-88417-1 | AC-11(a), CM-6(a) | medium | Set GNOME3 Screensaver Lock Delay After Activation Period | other |
| `dconf_gnome_screensaver_lock_enabled` | CCE-89684-5 | CM-6(a) | medium | Enable GNOME3 Screensaver Lock After Idle Period - Enable GNOME3 Screensaver | other |
| `dconf_gnome_screensaver_lock_locked` | CCE-87356-2 | CM-6(a) | medium | Prevent user modification of GNOME Screensaver lock-enabled | other |
| `dconf_gnome_screensaver_mode_blank` | CCE-88476-7 | AC-11(1), AC-11(1).1, CM-6(a) | medium | Implement Blank Screensaver | other |
| `dconf_gnome_screensaver_user_locks` | CCE-88349-6 | CM-6(a) | medium | Prevent user modification of GNOME lock-delay | other |
| `dconf_gnome_session_idle_user_locks` | CCE-88587-1 | CM-6(a) | medium | Prevent user modification of GNOME Session idle-delay | other |
| `dir_group_ownership_library_dirs` | CCE-88290-2 | CM-5(6), CM-5(6).1 | medium | Set the dir_group_ownership_library_dirs_newgroup variable if represented | file-check |
| `dir_ownership_library_dirs` | CCE-89745-4 | CM-5(6), CM-5(6).1 | medium | Set the dir_ownership_library_dirs_newown variable if represented by uid | file-check |
| `dir_permissions_library_dirs` | CCE-87731-6 | CM-5, CM-5(6), CM-5(6).1 | medium | Find /lib/ file(s) recursively | file-check |
| `dir_perms_world_writable_root_owned` | CCE-89514-4 |  | medium | Ensure All World-Writable Directories Are Owned by root User - Define Excluded | file-check |
| `dir_perms_world_writable_sticky_bits` | CCE-88397-5 | AC-6(1), CM-6(a) | medium | Verify that All World-Writable Directories Have Sticky Bits Set - Define | file-check |
| `directory_group_ownership_var_log_audit` | CCE-88841-2 | AC-6(1), AU-9(4), CM-6(a) | medium | System Audit Directories Must Be Group Owned By Root - Register Audit Configurat | other |
| `directory_groupowner_sshd_config_d` | CCE-89991-4 | AC-17(a), AC-6(1), CM-6(a) | medium | Set the directory_groupowner_sshd_config_d_newgroup variable if represented | other |
| `directory_owner_sshd_config_d` | CCE-89747-0 | AC-17(a), AC-6(1), CM-6(a) | medium | Set the directory_owner_sshd_config_d_newown variable if represented by | other |
| `directory_ownership_var_log_audit` | CCE-86731-7 | AC-6(1), AU-9(4), CM-6(a) | medium | System Audit Directories Must Be Owned By Root - Register Audit Configuration | other |
| `directory_permissions_sshd_config_d` | CCE-89963-3 | AC-17(a), AC-6(1), CM-6(a) | medium | Find /etc/ssh/sshd_config.d/ file(s) | other |
| `disable_ctrlaltdel_burstaction` | CCE-87627-6 | AC-6(1), CM-6(a) | high | Disable Ctrl-Alt-Del Burst Action | other |
| `disable_ctrlaltdel_reboot` | CCE-90035-7 | AC-6(1), CM-6(a) | high | Disable Ctrl-Alt-Del Reboot Activation | other |
| `disable_host_auth` | CCE-88057-5 | AC-17(a), AC-3, CM-6(a), CM-7(a), CM-7(b) | medium | Disable Host-Based Authentication - Check if the parameter HostbasedAuthenticati | other |
| `disable_users_coredumps` | CCE-88330-6 | CM-6, SC-7(10) | medium | Disable Core Dumps for All Users - Set dirs, files and regex variables | other |
| `disallow_bypass_password_sudo` | CCE-89298-4 | IA-11 | medium | Check for pam_succeed_if entry | other |
| `display_login_attempts` | CCE-88650-7 | AC-9, AC-9(1) | low | Ensure PAM Displays Last Logon/Access Notification - Check if system relies | other |
| `dnf-automatic_apply_updates` | CCE-86671-5 | CM-6(a), SI-2(5), SI-2(c) | medium | Configure dnf-automatic to Install Available Updates Automatically | other |
| `ensure_gpgcheck_globally_activated` | CCE-88404-9 | CM-11(a), CM-11(b), CM-5(3), CM-6(a), SA-12, SA-12(10), SC-12, SC-12(3), SI-7 | high | Ensure GPG check is globally activated | other |
| `ensure_gpgcheck_local_packages` | CCE-89409-7 | CM-11(a), CM-11(b), CM-5(3), CM-6(a), SA-12, SA-12(10) | high | Ensure GPG check Enabled for Local Packages (dnf) | other |
| `ensure_gpgcheck_never_disabled` | CCE-88176-3 | CM-11(a), CM-11(b), CM-5(3), CM-6(a), SA-12, SA-12(10), SC-12, SC-12(3), SI-7 | high | Grep for dnf repo section names | other |
| `ensure_redhat_gpgkey_installed` | CCE-88256-3 | CM-5(3), CM-6(a), SC-12, SC-12(3), SI-7 | high | Read permission of GPG key directory | other |
| `fapolicy_default_deny` | CCE-90343-5 | CM-6 b, CM-7 (2), CM-7 (5) (b) | medium | Configure Fapolicy Module to Employ a Deny-all, Permit-by-exception Policy | other |
| `file_audit_tools_group_ownership` | CCE-86839-8 | AU-9 | medium | Set the file_audit_tools_group_ownership_newgroup variable if represented | file-check |
| `file_audit_tools_ownership` | CCE-87874-4 | AU-9 | medium | Set the file_audit_tools_ownership_newown variable if represented by uid | file-check |
| `file_audit_tools_permissions` | CCE-86578-2 | AU-9 | medium | Test for existence /sbin/auditctl | file-check |
| `file_groupowner_backup_etc_group` | CCE-89477-4 | AC-6 (1) | medium | Set the file_groupowner_backup_etc_group_newgroup variable if represented | file-check |
| `file_groupowner_backup_etc_gshadow` | CCE-88453-6 | AC-6 (1) | medium | Set the file_groupowner_backup_etc_gshadow_newgroup variable if represented | file-check |
| `file_groupowner_backup_etc_passwd` | CCE-89914-6 | AC-6 (1) | medium | Set the file_groupowner_backup_etc_passwd_newgroup variable if represented | file-check |
| `file_groupowner_backup_etc_shadow` | CCE-88235-7 |  | medium | Set the file_groupowner_backup_etc_shadow_newgroup variable if represented | file-check |
| `file_groupowner_cron_allow` | CCE-90094-4 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_cron_allow_newgroup variable if represented by gid | file-check |
| `file_groupowner_cron_d` | CCE-89321-4 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_cron_d_newgroup variable if represented by gid | file-check |
| `file_groupowner_cron_daily` | CCE-90342-7 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_cron_daily_newgroup variable if represented by gid | file-check |
| `file_groupowner_cron_deny` | CCE-88060-9 | CM-6 b | medium | Set the file_groupowner_cron_deny_newgroup variable if represented by gid | file-check |
| `file_groupowner_cron_hourly` | CCE-88140-9 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_cron_hourly_newgroup variable if represented by | file-check |
| `file_groupowner_cron_monthly` | CCE-88986-5 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_cron_monthly_newgroup variable if represented by | file-check |
| `file_groupowner_cron_weekly` | CCE-89080-6 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_cron_weekly_newgroup variable if represented by | file-check |
| `file_groupowner_crontab` | CCE-89062-4 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_crontab_newgroup variable if represented by gid | file-check |
| `file_groupowner_etc_group` | CCE-90261-9 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_etc_group_newgroup variable if represented by gid | file-check |
| `file_groupowner_etc_gshadow` | CCE-90043-1 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_etc_gshadow_newgroup variable if represented by | file-check |
| `file_groupowner_etc_passwd` | CCE-89210-9 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_etc_passwd_newgroup variable if represented by gid | file-check |
| `file_groupowner_etc_shadow` | CCE-87579-9 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_etc_shadow_newgroup variable if represented by gid | file-check |
| `file_groupowner_grub2_cfg` | CCE-88691-1 | AC-6(1), CM-6(a) | medium | Set the file_groupowner_grub2_cfg_newgroup variable if represented by gid | file-check |
| `file_groupowner_sshd_config` | CCE-86992-5 | AC-17(a), AC-6(1), CM-6(a) | medium | Set the file_groupowner_sshd_config_newgroup variable if represented by | file-check |
| `file_groupowner_sshd_drop_in_config` | CCE-86254-0 | AC-17(a), AC-6(1), CM-6(a) | medium | Set the file_groupowner_sshd_drop_in_config_newgroup variable if represented | file-check |
| `file_groupowner_var_log` | CCE-89035-0 |  | medium | Set the file_groupowner_var_log_newgroup variable if represented by gid | file-check |
| `file_groupowner_var_log_messages` | CCE-86924-8 |  | medium | Set the file_groupowner_var_log_messages_newgroup variable if represented | file-check |
| `file_groupownership_audit_configuration` | CCE-88238-1 |  | medium | Set the file_groupownership_audit_configuration_newgroup variable if represented | file-check |
| `file_groupownership_home_directories` | CCE-87946-0 |  | medium | Get all local users from /etc/passwd | file-check |
| `file_groupownership_system_commands_dirs` | CCE-89800-7 | CM-5(6), CM-5(6).1 | medium | Verify that system commands files are group owned by root or a system account | file-check |
| `file_owner_backup_etc_group` | CCE-89017-8 | AC-6 (1) | medium | Set the file_owner_backup_etc_group_newown variable if represented by uid | file-check |
| `file_owner_backup_etc_gshadow` | CCE-86957-8 | AC-6 (1) | medium | Set the file_owner_backup_etc_gshadow_newown variable if represented by | file-check |
| `file_owner_backup_etc_passwd` | CCE-90377-3 | AC-6 (1) | medium | Set the file_owner_backup_etc_passwd_newown variable if represented by uid | file-check |
| `file_owner_backup_etc_shadow` | CCE-87502-1 | AC-6 (1) | medium | Set the file_owner_backup_etc_shadow_newown variable if represented by uid | file-check |
| `file_owner_cron_allow` | CCE-88914-7 | AC-6(1), CM-6(a) | medium | Set the file_owner_cron_allow_newown variable if represented by uid | file-check |
| `file_owner_cron_d` | CCE-88741-4 | AC-6(1), CM-6(a) | medium | Set the file_owner_cron_d_newown variable if represented by uid | file-check |
| `file_owner_cron_daily` | CCE-87499-0 | AC-6(1), CM-6(a) | medium | Set the file_owner_cron_daily_newown variable if represented by uid | file-check |
| `file_owner_cron_deny` | CCE-86823-2 | CM-6 b | medium | Set the file_owner_cron_deny_newown variable if represented by uid | file-check |
| `file_owner_cron_hourly` | CCE-89705-8 | AC-6(1), CM-6(a) | medium | Set the file_owner_cron_hourly_newown variable if represented by uid | file-check |
| `file_owner_cron_monthly` | CCE-90753-5 | AC-6(1), CM-6(a) | medium | Set the file_owner_cron_monthly_newown variable if represented by uid | file-check |
| `file_owner_cron_weekly` | CCE-88943-6 | AC-6(1), CM-6(a) | medium | Set the file_owner_cron_weekly_newown variable if represented by uid | file-check |
| `file_owner_crontab` | CCE-87294-5 | AC-6(1), CM-6(a) | medium | Set the file_owner_crontab_newown variable if represented by uid | file-check |
| `file_owner_etc_group` | CCE-86870-3 | AC-6(1), CM-6(a) | medium | Set the file_owner_etc_group_newown variable if represented by uid | file-check |
| `file_owner_etc_gshadow` | CCE-87701-9 | AC-6(1), CM-6(a) | medium | Set the file_owner_etc_gshadow_newown variable if represented by uid | file-check |
| `file_owner_etc_passwd` | CCE-87827-2 | AC-6(1), CM-6(a) | medium | Set the file_owner_etc_passwd_newown variable if represented by uid | file-check |
| `file_owner_etc_shadow` | CCE-86857-0 | AC-6(1), CM-6(a) | medium | Set the file_owner_etc_shadow_newown variable if represented by uid | file-check |
| `file_owner_grub2_cfg` | CCE-89438-6 | AC-6(1), CM-6(a) | medium | Set the file_owner_grub2_cfg_newown variable if represented by uid | file-check |
| `file_owner_sshd_config` | CCE-89829-6 | AC-17(a), AC-6(1), CM-6(a) | medium | Set the file_owner_sshd_config_newown variable if represented by uid | file-check |
| `file_owner_sshd_drop_in_config` | CCE-86268-0 | AC-17(a), AC-6(1), CM-6(a) | medium | Set the file_owner_sshd_drop_in_config_newown variable if represented by | file-check |
| `file_owner_var_log` | CCE-86705-1 |  | medium | Set the file_owner_var_log_newown variable if represented by uid | file-check |
| `file_owner_var_log_messages` | CCE-89093-9 |  | medium | Set the file_owner_var_log_messages_newown variable if represented by uid | file-check |
| `file_ownership_audit_configuration` | CCE-88877-6 |  | medium | Set the file_ownership_audit_configuration_newown variable if represented | file-check |
| `file_ownership_binary_dirs` | CCE-89620-9 | AC-6(1), CM-5(6), CM-5(6).1, CM-6(a) | medium | Read list of system executables without root ownership | file-check |
| `file_ownership_library_dirs` | CCE-87988-2 | AC-6(1), CM-5(6), CM-5(6).1, CM-6(a) | medium | Set the file_ownership_library_dirs_newown variable if represented by uid | file-check |
| `file_permission_user_init_files` | CCE-87771-2 |  | medium | Ensure All User Initialization Files Have Mode 0740 Or Less Permissive - | file-check |
| `file_permission_user_init_files_root` | CCE-89585-4 |  | medium | Ensure All User Initialization Files Have Mode 0740 Or Less Permissive - | file-check |
| `file_permissions_audit_configuration` | CCE-88067-4 | AU-12 b | medium | Find /etc/audit/ file(s) | file-check |
| `file_permissions_backup_etc_group` | CCE-86579-0 | AC-6 (1) | medium | Test for existence /etc/group- | file-check |
| `file_permissions_backup_etc_gshadow` | CCE-89056-6 | AC-6 (1) | medium | Test for existence /etc/gshadow- | file-check |
| `file_permissions_backup_etc_passwd` | CCE-86854-7 | AC-6 (1) | medium | Test for existence /etc/passwd- | file-check |
| `file_permissions_backup_etc_shadow` | CCE-87423-0 | AC-6 (1) | medium | Test for existence /etc/shadow- | file-check |
| `file_permissions_binary_dirs` | CCE-86978-4 | AC-6(1), CM-5(6), CM-5(6).1, CM-6(a) | medium | Read list of world and group writable system executables | file-check |
| `file_permissions_cron_allow` | CCE-89121-8 |  | medium | Test for existence /etc/cron.allow | file-check |
| `file_permissions_cron_d` | CCE-86651-7 | AC-6(1), CM-6(a) | medium | Find /etc/cron.d/ file(s) | file-check |
| `file_permissions_cron_daily` | CCE-88919-6 | AC-6(1), CM-6(a) | medium | Find /etc/cron.daily/ file(s) | file-check |
| `file_permissions_cron_hourly` | CCE-88664-8 | AC-6(1), CM-6(a) | medium | Find /etc/cron.hourly/ file(s) | file-check |
| `file_permissions_cron_monthly` | CCE-86632-7 | AC-6(1), CM-6(a) | medium | Find /etc/cron.monthly/ file(s) | file-check |
| `file_permissions_cron_weekly` | CCE-89733-0 | AC-6(1), CM-6(a) | medium | Find /etc/cron.weekly/ file(s) | file-check |
| `file_permissions_crontab` | CCE-90078-7 | AC-6(1), CM-6(a) | medium | Test for existence /etc/crontab | file-check |
| `file_permissions_etc_audit_auditd` | CCE-89306-5 | AU-12(b) | medium | Test for existence /etc/audit/auditd.conf | file-check |
| `file_permissions_etc_audit_rulesd` | CCE-89313-1 | AU-12(b) | medium | Find /etc/audit/rules.d/ file(s) | file-check |
| `file_permissions_etc_group` | CCE-88868-5 | AC-6(1), CM-6(a) | medium | Test for existence /etc/group | file-check |
| `file_permissions_etc_gshadow` | CCE-86975-0 | AC-6(1), CM-6(a) | medium | Test for existence /etc/gshadow | file-check |
| `file_permissions_etc_passwd` | CCE-90644-6 | AC-6(1), CM-6(a) | medium | Test for existence /etc/passwd | file-check |
| `file_permissions_etc_shadow` | CCE-88433-8 | AC-6(1), CM-6(a) | medium | Test for existence /etc/shadow | file-check |
| `file_permissions_home_directories` | CCE-86605-3 |  | medium | Get all local users from /etc/passwd | file-check |
| `file_permissions_library_dirs` | CCE-88771-1 | AC-6(1), CM-5(6), CM-5(6).1, CM-6(a) | medium | Find /lib/ file(s) recursively | file-check |
| `file_permissions_sshd_config` | CCE-86264-9 | AC-17(a), AC-6(1), CM-6(a) | medium | Test for existence /etc/ssh/sshd_config | file-check |
| `file_permissions_sshd_drop_in_config` | CCE-86442-1 | AC-17(a), AC-6(1), CM-6(a) | medium | Find /etc/ssh/sshd_config.d/ file(s) | file-check |
| `file_permissions_sshd_private_key` | CCE-88018-7 | AC-17(a), AC-6(1), CM-6(a) | medium | Find root:root-owned keys | file-check |
| `file_permissions_sshd_pub_key` | CCE-87454-5 | AC-17(a), AC-6(1), CM-6(a) | medium | Find /etc/ssh/ file(s) | file-check |
| `file_permissions_var_log` | CCE-89801-5 |  | medium | Find /var/log/ file(s) | file-check |
| `file_permissions_var_log_audit` | CCE-90129-8 | AC-6(1), AU-9(4), CM-6(a) | medium | Get audit log files | file-check |
| `file_permissions_var_log_messages` | CCE-89397-4 |  | medium | Test for existence /var/log/messages | file-check |
| `firewalld-backend` | CCE-87099-8 | SC-5 | medium | Insert correct line to /etc/firewalld/firewalld.conf | other |
| `firewalld_sshd_port_enabled` | CCE-89799-1 | AC-17(a), CM-6(b), CM-7(a), CM-7(b) | medium | Enable SSH Server firewalld Firewall Exception - Ensure firewalld and NetworkMan | other |
| `gnome_gdm_disable_automatic_login` | CCE-87057-6 | AC-6(1), CM-6(a), CM-7(b) | high | Disable GDM Automatic Login | other |
| `grub2_audit_argument` | CCE-88376-9 | AC-17(1), AU-10, AU-14(1), CM-6(a), IR-5(1) | low | Check if audit argument is already present in /etc/default/grub | cmdline |
| `grub2_audit_backlog_limit_argument` | CCE-88192-0 | CM-6(a) | low | Check if audit_backlog_limit argument is already present in /etc/default/grub | cmdline |
| `grub2_disable_interactive_boot` | CCE-89661-3 | CM-6(a), SC-2(1) | medium | Verify that Interactive Boot is Disabled - Verify GRUB_DISABLE_RECOVERY=true | cmdline |
| `grub2_init_on_free` | CCE-90140-5 | SC-3 | medium | Check if init_on_free argument is already present in /etc/default/grub | cmdline |
| `grub2_page_poison_argument` | CCE-89086-3 | CM-6(a) | medium | Check if page_poison argument is already present in /etc/default/grub | cmdline |
| `grub2_pti_argument` | CCE-88971-7 | SI-16 | low | Check if pti argument is already present in /etc/default/grub | cmdline |
| `grub2_vsyscall_argument` | CCE-87153-3 | CM-7(a) | medium | Check if vsyscall argument is already present in /etc/default/grub | cmdline |
| `install_smartcard_packages` | CCE-86642-6 | CM-6(a) | medium | Ensure pkcs11-provider is installed | other |
| `kernel_module_bluetooth_disabled` | CCE-87455-2 | AC-18(3), AC-18(a), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | Ensure kernel module 'bluetooth' is disabled | other |
| `kernel_module_can_disabled` | CCE-89282-8 | AC-18 | medium | Ensure kernel module 'can' is disabled | other |
| `kernel_module_sctp_disabled` | CCE-90489-6 | CM-6(a), CM-7(a), CM-7(b) | medium | Ensure kernel module 'sctp' is disabled | other |
| `kernel_module_tipc_disabled` | CCE-86569-1 | CM-6(a), CM-7(a), CM-7(b) | low | Ensure kernel module 'tipc' is disabled | other |
| `kernel_module_usb-storage_disabled` | CCE-89301-6 | CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | Ensure kernel module 'usb-storage' is disabled | other |
| `logind_session_timeout` | CCE-88334-8 | AC-12, AC-17(a), AC-2(5), CM-6(a), SC-10 | medium | Set 'StopIdleSessionSec' to '{{ var_logind_session_timeout }}' in the [Login] | other |
| `mount_option_boot_nodev` | CCE-90132-2 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nodev Option to /boot: Check information associated to mountpoint' | file-check |
| `mount_option_boot_nosuid` | CCE-88881-8 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nosuid Option to /boot: Check information associated to mountpoint' | file-check |
| `mount_option_dev_shm_nodev` | CCE-86783-8 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nodev Option to /dev/shm: Check information associated to mountpoint' | file-check |
| `mount_option_dev_shm_noexec` | CCE-86775-4 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add noexec Option to /dev/shm: Check information associated to mountpoint' | file-check |
| `mount_option_dev_shm_nosuid` | CCE-88358-7 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nosuid Option to /dev/shm: Check information associated to mountpoint' | file-check |
| `mount_option_home_nodev` | CCE-87344-8 |  | unknown | 'Add nodev Option to /home: Check information associated to mountpoint' | file-check |
| `mount_option_home_noexec` | CCE-87810-8 | CM-6(b) | medium | 'Add noexec Option to /home: Check information associated to mountpoint' | file-check |
| `mount_option_home_nosuid` | CCE-88987-3 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nosuid Option to /home: Check information associated to mountpoint' | file-check |
| `mount_option_krb_sec_remote_filesystems` | CCE-87249-9 | AC-17(a), CM-6(a), CM-7(a), CM-7(b), IA-2, IA-2(8), IA-2(9) | medium | Get nfs and nfs4 mount points, that don't have sec=krb5:krb5i:krb5p | file-check |
| `mount_option_nodev_nonroot_local_partitions` | CCE-88981-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nodev Option to Non-Root Local Partitions: Refresh facts' | file-check |
| `mount_option_nodev_remote_filesystems` | CCE-88013-8 | CM-6(a), MP-2 | medium | Get nfs and nfs4 mount points, that don't have nodev | file-check |
| `mount_option_nodev_removable_partitions` | CCE-90154-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | Ensure permission nodev are set on var_removable_partition | file-check |
| `mount_option_noexec_remote_filesystems` | CCE-86463-7 | AC-6, AC-6(10), AC-6(8), CM-6(a) | medium | Get nfs and nfs4 mount points, that don't have noexec | file-check |
| `mount_option_noexec_removable_partitions` | CCE-90378-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | Ensure permission noexec are set on var_removable_partition | file-check |
| `mount_option_nosuid_remote_filesystems` | CCE-90504-2 | AC-6, AC-6(1), CM6(a) | medium | Get nfs and nfs4 mount points, that don't have nosuid | file-check |
| `mount_option_nosuid_removable_partitions` | CCE-88078-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | Ensure permission nosuid are set on var_removable_partition | file-check |
| `mount_option_tmp_nodev` | CCE-90522-4 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nodev Option to /tmp: Check information associated to mountpoint' | file-check |
| `mount_option_tmp_noexec` | CCE-87095-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add noexec Option to /tmp: Check information associated to mountpoint' | file-check |
| `mount_option_tmp_nosuid` | CCE-87318-2 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nosuid Option to /tmp: Check information associated to mountpoint' | file-check |
| `mount_option_var_log_audit_nodev` | CCE-87220-0 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nodev Option to /var/log/audit: Check information associated to mountpoint' | file-check |
| `mount_option_var_log_audit_noexec` | CCE-88957-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add noexec Option to /var/log/audit: Check information associated to mountpoint | file-check |
| `mount_option_var_log_audit_nosuid` | CCE-90694-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nosuid Option to /var/log/audit: Check information associated to mountpoint | file-check |
| `mount_option_var_log_nodev` | CCE-89389-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nodev Option to /var/log: Check information associated to mountpoint' | file-check |
| `mount_option_var_log_noexec` | CCE-89129-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add noexec Option to /var/log: Check information associated to mountpoint' | file-check |
| `mount_option_var_log_nosuid` | CCE-90639-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nosuid Option to /var/log: Check information associated to mountpoint' | file-check |
| `mount_option_var_nodev` | CCE-87070-9 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | 'Add nodev Option to /var: Check information associated to mountpoint' | file-check |
| `mount_option_var_tmp_nodev` | CCE-89441-0 |  | medium | 'Add nodev Option to /var/tmp: Check information associated to mountpoint' | file-check |
| `mount_option_var_tmp_noexec` | CCE-87347-1 |  | medium | 'Add noexec Option to /var/tmp: Check information associated to mountpoint' | file-check |
| `mount_option_var_tmp_nosuid` | CCE-87892-6 |  | medium | 'Add nosuid Option to /var/tmp: Check information associated to mountpoint' | file-check |
| `network_sniffer_disabled` | CCE-88985-7 | CM-6(a), CM-7(2), CM-7(a), CM-7(b), MA-3 | medium | Ensure System is Not Acting as a Network Sniffer - Gather network interfaces | other |
| `networkmanager_dns_mode` | CCE-90712-1 | CM-6(b) | medium | NetworkManager DNS Mode Must Be Must Configured - Search for a section in | other |
| `no_empty_passwords` | CCE-86640-0 | CM-6(a), IA-5(1)(a), IA-5(c) | high | Prevent Login to Accounts With Empty Password - Check if system relies on | other |
| `no_empty_passwords_etc_shadow` | CCE-90491-2 | CM-6(b), CM-6.1(iv) | high | Collect users with no password | other |
| `no_host_based_files` | CCE-89350-3 |  | high | Remove Host-Based Authentication Files - Define Excluded (Non-Local) File | other |
| `no_shelllogin_for_systemaccounts` | CCE-87448-7 | AC-6, CM-6(a), CM-6(b), CM-6.1(iv) | medium | Ensure that System Accounts Do Not Run a Shell Upon Login - Get All Local | other |
| `no_user_host_based_files` | CCE-89341-2 |  | high | Remove User Host-Based Authentication Files - Define Excluded (Non-Local) | other |
| `package_aide_installed` | CCE-90477-1 | CM-6(a) | medium | Ensure aide is installed | package-check |
| `package_audispd-plugins_installed` | CCE-88547-5 |  | medium | Ensure audispd-plugins is installed | package-check |
| `package_audit_installed` | CCE-88240-7 | AC-7(a), AU-12(2), AU-14, AU-2(a), AU-7(1), AU-7(2), CM-6(a) | medium | Ensure audit is installed | package-check |
| `package_chrony_installed` | CCE-89591-2 |  | medium | Ensure chrony is installed | package-check |
| `package_cron_installed` | CCE-86619-4 | CM-6(a) | medium | Ensure cronie is installed | package-check |
| `package_crypto-policies_installed` | CCE-89668-8 |  | medium | Ensure crypto-policies is installed | package-check |
| `package_fapolicyd_installed` | CCE-89813-0 | CM-6(a), SI-4(22) | medium | Ensure fapolicyd is installed | package-check |
| `package_firewalld_installed` | CCE-88164-9 | CM-6(a) | medium | Ensure firewalld is installed | package-check |
| `package_gdm_removed` | CCE-88880-0 | CM-6(a), CM-7(a), CM-7(b) | medium | 'Remove the GDM Package Group: Ensure gdm is removed' | package-check |
| `package_gnutls-utils_installed` | CCE-90403-7 |  | medium | Ensure gnutls-utils is installed | package-check |
| `package_gssproxy_removed` | CCE-87596-3 |  | medium | 'Uninstall gssproxy Package: Ensure gssproxy is removed' | package-check |
| `package_libreswan_installed` | CCE-87497-4 | CM-6(a) | medium | Ensure libreswan is installed | package-check |
| `package_nfs-utils_removed` | CCE-88270-4 |  | low | 'Uninstall nfs-utils Package: Ensure nfs-utils is removed' | package-check |
| `package_nss-tools_installed` | CCE-87829-8 |  | medium | Ensure nss-tools is installed | package-check |
| `package_opensc_installed` | CCE-86898-4 | CM-6(a) | medium | Ensure opensc is installed | package-check |
| `package_openssh-clients_installed` | CCE-86852-1 |  | medium | Ensure openssh-clients is installed | package-check |
| `package_openssh-server_installed` | CCE-89241-4 | CM-6(a) | medium | Ensure openssh-server is installed | package-check |
| `package_pcsc-lite-ccid_installed` | CCE-86250-8 | CM-6(a) | medium | Ensure pcsc-lite-ccid is installed | package-check |
| `package_pcsc-lite_installed` | CCE-88682-0 | CM-6(a) | medium | Ensure pcsc-lite is installed | package-check |
| `package_policycoreutils-python-utils_installed` | CCE-87004-8 |  | medium | Ensure policycoreutils-python-utils is installed | package-check |
| `package_policycoreutils_installed` | CCE-88996-4 |  | low | Ensure policycoreutils is installed | package-check |
| `package_rsyslog-gnutls_installed` | CCE-89106-9 |  | medium | Ensure rsyslog-gnutls is installed | package-check |
| `package_rsyslog_installed` | CCE-90353-4 | CM-6(a) | medium | Ensure rsyslog is installed | package-check |
| `package_s-nail_installed` | CCE-89346-1 | CM-3(5) | medium | Ensure s-nail is installed | package-check |
| `package_sssd_installed` | CCE-88372-8 | CM-6(a) | medium | Ensure sssd is installed | package-check |
| `package_subscription-manager_installed` | CCE-88542-6 |  | medium | Ensure subscription-manager is installed | package-check |
| `package_sudo_installed` | CCE-87100-4 | CM-6(a) | medium | Ensure sudo is installed | package-check |
| `package_telnet-server_removed` | CCE-88105-2 | CM-6(a), CM-7(a), CM-7(b) | high | 'Uninstall telnet-server Package: Ensure telnet-server is removed' | package-check |
| `package_tftp-server_removed` | CCE-89287-7 | CM-6(a), CM-7(a), CM-7(b) | high | 'Uninstall tftp-server Package: Ensure tftp-server is removed' | package-check |
| `package_tftp_removed` | CCE-88586-3 |  | low | 'Remove tftp Daemon: Ensure tftp is removed' | package-check |
| `package_tuned_removed` | CCE-87654-0 |  | medium | 'Uninstall tuned Package: Ensure tuned is removed' | package-check |
| `package_unbound_removed` | CCE-86181-5 | CM-6(a), CM-7(a), CM-7(b) | low | 'Uninstall unbound Package: Ensure unbound is removed' | package-check |
| `package_usbguard_installed` | CCE-87756-3 | CM-8(3), IA-3 | medium | Ensure usbguard is installed | package-check |
| `package_vsftpd_removed` | CCE-88674-7 | CM-6(a), CM-7, CM-7(a), CM-7(b), CM-7.1(ii), IA-5(1)(c), IA-5(1).1(v) | high | 'Uninstall vsftpd Package: Ensure vsftpd is removed' | package-check |
| `postfix_client_configure_mail_alias` | CCE-87937-9 | CM-6(a) | medium | Configure System to Forward All Mail For The Root Account - Make sure that | other |
| `postfix_client_configure_mail_alias_postmaster` | CCE-89448-5 | AU-5(a), AU-5.1(ii) | medium | Insert correct line to /etc/aliases | other |
| `postfix_prevent_unrestricted_relay` | CCE-87792-8 |  | medium | Insert correct line to /etc/postfix/main.cf | other |
| `require_singleuser_auth` | CCE-90014-2 | AC-3, CM-6(a), IA-2 | medium | Require Authentication for Single User Mode - find files which already override | other |
| `root_permissions_syslibrary_files` | CCE-86440-5 | CM-5(6), CM-5(6).1 | medium | Set the root_permissions_syslibrary_files_newgroup variable if represented | other |
| `rootfiles_configured` | CCE-86476-9 |  | medium | Ensure rootfiles tmpfile.d is Configured Correctly - Find configuration | other |
| `rsyslog_cron_logging` | CCE-90383-1 | CM-6(a) | medium | Ensure cron Is Logging To Rsyslog - Ensure /etc/rsyslog.conf exists | other |
| `rsyslog_encrypt_offload_actionsendstreamdriverauthmode` | CCE-88521-0 | AU-4(1) | medium | Ensure Rsyslog Authenticates Off-Loaded Audit Records - Ensure /etc/rsyslog.conf | other |
| `rsyslog_encrypt_offload_actionsendstreamdrivermode` | CCE-88359-5 | AU-4(1) | medium | Ensure Rsyslog Encrypts Off-Loaded Audit Records - Ensure /etc/rsyslog.conf | other |
| `rsyslog_encrypt_offload_defaultnetstreamdriver` | CCE-89018-6 | AU-4(1) | medium | Ensure Rsyslog Encrypts Off-Loaded Audit Records - Ensure /etc/rsyslog.conf | other |
| `rsyslog_nolisten` | CCE-89374-3 | CM-6(a), CM-7(a), CM-7(b) | medium | Ensure rsyslog Does Not Accept Remote Messages Unless Acting As Log Server | other |
| `rsyslog_remote_access_monitoring` | CCE-87341-4 | AC-17(1) | medium | 'Ensure remote access methods are monitored in Rsyslog: Set facts' | other |
| `rsyslog_remote_loghost` | CCE-90372-4 | AU-4(1), AU-9(2), CM-6(a) | medium | Set rsyslog remote loghost | other |
| `selinux_policytype` | CCE-88366-0 | AC-3, AC-3(3)(a), AU-9, SC-7(21) | medium | Insert correct line to /etc/selinux/config | other |
| `selinux_state` | CCE-89386-7 | AC-3, AC-3(3)(a), AU-9, SC-7(21) | high | Ensure SELinux State is Enforcing - Check current SELinux state | other |
| `service_systemd-coredump_disabled` | CCE-90438-3 | SC-7(10) | medium | Disable acquiring, saving, and processing core dumps - Collect systemd Socket | service-check |
| `set_password_hashing_algorithm_libuserconf` | CCE-90325-2 | CM-6(a), IA-5(1)(c), IA-5(c) | medium | Set Password Hashing Algorithm in /etc/libuser.conf - Set Password Hashing | other |
| `set_password_hashing_algorithm_logindefs` | CCE-89508-6 | CM-6(a), IA-5(1)(c), IA-5(c) | medium | Set Password Hashing Algorithm in /etc/login.defs | other |
| `set_password_hashing_algorithm_passwordauth` | CCE-88661-4 | CM-6(a), IA-5(1)(c), IA-5(c) | medium | Set PAM's Password Hashing Algorithm - password-auth - Check if /etc/pam.d/passw | other |
| `set_password_hashing_algorithm_systemauth` | CCE-88697-8 | CM-6(a), IA-5(1)(c), IA-5(c) | medium | Set PAM's Password Hashing Algorithm - Check if /etc/pam.d/system-auth file | other |
| `set_password_hashing_min_rounds_logindefs` | CCE-90508-3 |  | medium | Set Password Hashing Rounds in /etc/login.defs - extract contents of the | other |
| `special_service_block` | CCE-90212-2 | AC-2(g), AC-4, AC-6(9), AU-10, AU-12(c), AU-14(1), AU-2(d), AU-3, AU-4(1), CA-3(5), CM-6, CM-6(a), CM-7(a), CM-7(b), CM-8(3)(a), IA-2(1), IA-2(11), IA-2(2), IA-2(3), IA-2(4), IA-2(6), IA-2(7), IA-3, IA-5(10), MP-7, SC-24, SC-7(21), SC-8, SC-8(1), SC-8(2), SC-8(3), SC-8(4), SI-4(22), SI-4(23) | medium | Disable debug-shell SystemD Service - Disable Socket debug-shell | other |
| `ssh_client_rekey_limit` | CCE-89510-2 |  | medium | Ensure RekeyLimit is not configured in /etc/ssh/ssh_config | other |
| `sshd_disable_compression` | CCE-90051-4 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | medium | Disable Compression Or Set Compression to delayed - Check if the parameter | other |
| `sshd_disable_empty_passwords` | CCE-86753-1 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | high | Disable SSH Access via Empty Passwords - Check if the parameter PermitEmptyPassw | other |
| `sshd_disable_gssapi_auth` | CCE-89145-7 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | medium | Disable GSSAPI Authentication - Check if the parameter GSSAPIAuthentication | other |
| `sshd_disable_kerb_auth` | CCE-90591-9 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | medium | Disable Kerberos Authentication - Check if the parameter KerberosAuthentication | other |
| `sshd_disable_rhosts` | CCE-87777-9 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | medium | Disable SSH Support for .rhosts Files - Check if the parameter IgnoreRhosts | other |
| `sshd_disable_root_login` | CCE-89730-6 | AC-17(a), AC-6(2), CM-6(a), CM-7(a), CM-7(b), IA-2, IA-2(5) | medium | Disable SSH Root Login - Check if the parameter PermitRootLogin is configured | other |
| `sshd_disable_user_known_hosts` | CCE-87313-3 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | medium | Disable SSH Support for User Known Hosts - Check if the parameter IgnoreUserKnow | other |
| `sshd_disable_x11_forwarding` | CCE-89476-6 | CM-6(b) | medium | Disable X11 Forwarding - Check if the parameter X11Forwarding is configured | other |
| `sshd_do_not_permit_user_env` | CCE-87395-0 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | medium | Do Not Allow SSH Environment Options - Check if the parameter PermitUserEnvironm | other |
| `sshd_enable_pam` | CCE-87045-1 |  | medium | Enable PAM - Check if the parameter UsePAM is configured | other |
| `sshd_enable_pubkey_auth` | CCE-90625-5 |  | medium | Enable Public Key Authentication - Check if the parameter PubkeyAuthentication | other |
| `sshd_enable_strictmodes` | CCE-88037-7 | AC-17(a), AC-6, CM-6(a) | medium | Enable Use of Strict Mode Checking - Check if the parameter StrictModes | other |
| `sshd_enable_warning_banner` | CCE-86539-4 | AC-17(a), AC-8(a), AC-8(c), CM-6(a) | medium | Enable SSH Warning Banner - Check if the parameter Banner is configured | other |
| `sshd_print_last_log` | CCE-88362-9 | AC-9, AC-9(1) | medium | Enable SSH Print Last Log - Check if the parameter PrintLastLog is configured | other |
| `sshd_rekey_limit` | CCE-88356-1 |  | medium | Force frequent session key renegotiation - Check if the parameter RekeyLimit | other |
| `sshd_set_idle_timeout` | CCE-90362-5 | AC-12, AC-17(a), AC-2(5), CM-6(a), SC-10 | medium | Set SSH Client Alive Interval - Check if the parameter ClientAliveInterval | other |
| `sshd_set_keepalive` | CCE-86794-5 | AC-12, AC-17(a), AC-2(5), CM-6(a), SC-10 | medium | Set SSH Client Alive Count Max - Check if the parameter ClientAliveCountMax | other |
| `sshd_set_loglevel_verbose` | CCE-86241-7 | AC-17(1), AC-17(a), CM-6(a) | medium | Set SSH Daemon LogLevel to VERBOSE - Check if the parameter LogLevel is | other |
| `sshd_x11_use_localhost` | CCE-86528-7 | CM-6(b) | medium | Prevent remote hosts from connecting to the proxy display - Check if the | other |
| `sssd_certificate_verification` | CCE-86192-2 | IA-2(11) | medium | Ensure that "certificate_verification" is not set in /etc/sssd/sssd.conf | other |
| `sssd_enable_smartcards` | CCE-90275-9 |  | medium | Test for domain group | other |
| `sssd_offline_cred_expiration` | CCE-90741-0 | CM-6(a), IA-5(13) | medium | Test for domain group | other |
| `sudo_remove_no_authenticate` | CCE-88892-5 | CM-6(a), IA-11 | medium | Find /etc/sudoers.d/ files | other |
| `sudo_remove_nopasswd` | CCE-87015-4 | CM-6(a), IA-11 | medium | Find /etc/sudoers.d/ files | other |
| `sudo_require_reauthentication` | CCE-88136-7 | IA-11 | medium | Require Re-Authentication When Using the sudo Command - Find /etc/sudoers.d/* | other |
| `sudoers_validate_passwd` | CCE-88855-2 | CM-6(b), CM-6.1(iv) | medium | Find out if /etc/sudoers.d/* files contain Defaults targetpw to be deduplicated | other |
| `sysctl_fs_protected_hardlinks` | CCE-86689-7 | AC-6(1), CM-6(a) | medium | Enable Kernel Parameter to Enforce DAC on Hardlinks - Set fact for sysctl | sysctl |
| `sysctl_fs_protected_symlinks` | CCE-88796-8 | AC-6(1), CM-6(a) | medium | Enable Kernel Parameter to Enforce DAC on Symlinks - Set fact for sysctl | sysctl |
| `sysctl_kernel_core_pattern` | CCE-86714-3 | SC-7(10) | medium | Disable storing core dumps - Set fact for sysctl paths | sysctl |
| `sysctl_kernel_dmesg_restrict` | CCE-89000-4 | SI-11(a), SI-11(b) | low | Restrict Access to Kernel Message Buffer - Set fact for sysctl paths | sysctl |
| `sysctl_kernel_exec_shield` | CCE-89079-8 | CM-6(a), SC-39 | medium | Check if noexec argument is already present in /etc/default/grub | sysctl |
| `sysctl_kernel_kexec_load_disabled` | CCE-89232-3 | CM-6 | medium | Disable Kernel Image Loading - Set fact for sysctl paths | sysctl |
| `sysctl_kernel_kptr_restrict` | CCE-88686-1 | CM-6(a), SC-30, SC-30(2), SC-30(5) | medium | Restrict Exposed Kernel Pointer Addresses Access - Set fact for sysctl paths | sysctl |
| `sysctl_kernel_perf_event_paranoid` | CCE-90142-1 | AC-6 | low | Disallow kernel profiling by unprivileged users - Set fact for sysctl paths | sysctl |
| `sysctl_kernel_randomize_va_space` | CCE-87876-9 | CM-6(a), SC-30, SC-30(2) | medium | Enable Randomized Layout of Virtual Address Space - Set fact for sysctl | sysctl |
| `sysctl_kernel_unprivileged_bpf_disabled` | CCE-89405-5 | AC-6, SC-7(10) | medium | Disable Access to Network bpf() Syscall From Unprivileged Processes - Set | sysctl |
| `sysctl_kernel_yama_ptrace_scope` | CCE-88785-1 | SC-7(10) | medium | Restrict usage of ptrace to descendant processes - Set fact for sysctl paths | sysctl |
| `sysctl_net_core_bpf_jit_harden` | CCE-89631-6 | CM-6, SC-7(10) | medium | Harden the operation of the BPF just-in-time compiler - Set fact for sysctl | sysctl |
| `sysctl_net_ipv4_conf_all_accept_redirects` | CCE-90409-4 | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | medium | Disable Accepting ICMP Redirects for All IPv4 Interfaces - Set fact for | sysctl |
| `sysctl_net_ipv4_conf_all_accept_source_route` | CCE-90165-2 | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | medium | Disable Kernel Parameter for Accepting Source-Routed Packets on all IPv4 | sysctl |
| `sysctl_net_ipv4_conf_all_forwarding` | CCE-87420-6 | CM-6(b) | medium | Disable Kernel Parameter for IPv4 Forwarding on all IPv4 Interfaces - Set | sysctl |
| `sysctl_net_ipv4_conf_all_rp_filter` | CCE-88689-5 | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | medium | Enable Kernel Parameter to Use Reverse Path Filtering on all IPv4 Interfaces | sysctl |
| `sysctl_net_ipv4_conf_all_send_redirects` | CCE-88360-3 | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | medium | Disable Kernel Parameter for Sending ICMP Redirects on all IPv4 Interfaces | sysctl |
| `sysctl_net_ipv4_conf_default_accept_redirects` | CCE-86820-8 | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | medium | Disable Kernel Parameter for Accepting ICMP Redirects by Default on IPv4 | sysctl |
| `sysctl_net_ipv4_conf_default_accept_source_route` | CCE-88071-6 | CM-7(a), CM-7(b), SC-5, SC-7(a) | medium | Disable Kernel Parameter for Accepting Source-Routed Packets on IPv4 Interfaces | sysctl |
| `sysctl_net_ipv4_conf_default_rp_filter` | CCE-87424-8 | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | medium | Enable Kernel Parameter to Use Reverse Path Filtering on all IPv4 Interfaces | sysctl |
| `sysctl_net_ipv4_conf_default_send_redirects` | CCE-89177-0 | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | medium | Disable Kernel Parameter for Sending ICMP Redirects on all IPv4 Interfaces | sysctl |
| `sysctl_net_ipv4_icmp_echo_ignore_broadcasts` | CCE-86918-0 | CM-7(a), CM-7(b), SC-5 | medium | Enable Kernel Parameter to Ignore ICMP Broadcast Echo Requests on IPv4 Interface | sysctl |
| `sysctl_net_ipv4_icmp_ignore_bogus_error_responses` | CCE-87841-3 | CM-7(a), CM-7(b), SC-5 | unknown | Enable Kernel Parameter to Ignore Bogus ICMP Error Responses on IPv4 Interfaces | sysctl |
| `sysctl_net_ipv4_ip_forward` | CCE-87377-8 | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | medium | Disable Kernel Parameter for IP Forwarding on IPv4 Interfaces - Set fact | sysctl |
| `sysctl_net_ipv4_tcp_invalid_ratelimit` | CCE-86242-5 | SC-5 | medium | Configure Kernel to Rate Limit Sending of Duplicate TCP Acknowledgments | sysctl |
| `sysctl_net_ipv4_tcp_syncookies` | CCE-88084-9 | CM-6(a), CM-7(a), CM-7(b), SC-5(1), SC-5(2), SC-5(3)(a) | medium | Enable Kernel Parameter to Use TCP Syncookies on Network Interfaces - Set | sysctl |
| `sysctl_net_ipv6_conf_all_accept_ra` | CCE-88665-5 | CM-6(a), CM-7(a), CM-7(b) | medium | Configure Accepting Router Advertisements on All IPv6 Interfaces - Set fact | sysctl |
| `sysctl_net_ipv6_conf_all_accept_redirects` | CCE-90083-7 | CM-6(a), CM-6(b), CM-6.1(iv), CM-7(a), CM-7(b) | medium | Disable Accepting ICMP Redirects for All IPv6 Interfaces - Set fact for | sysctl |
| `sysctl_net_ipv6_conf_all_accept_source_route` | CCE-90450-8 | CM-6(a), CM-7(a), CM-7(b) | medium | Disable Kernel Parameter for Accepting Source-Routed Packets on all IPv6 | sysctl |
| `sysctl_net_ipv6_conf_all_forwarding` | CCE-86882-8 | CM-6(a), CM-6(b), CM-6.1(iv), CM-7(a), CM-7(b) | medium | Disable Kernel Parameter for IPv6 Forwarding - Set fact for sysctl paths | sysctl |
| `sysctl_net_ipv6_conf_default_accept_ra` | CCE-90557-0 | CM-6(a), CM-7(a), CM-7(b) | medium | Disable Accepting Router Advertisements on all IPv6 Interfaces by Default | sysctl |
| `sysctl_net_ipv6_conf_default_accept_redirects` | CCE-89486-5 | CM-6(a), CM-7(a), CM-7(b) | medium | Disable Kernel Parameter for Accepting ICMP Redirects by Default on IPv6 | sysctl |
| `sysctl_net_ipv6_conf_default_accept_source_route` | CCE-89135-8 | CM-6(a), CM-6(b), CM-6.1(iv), CM-7(a), CM-7(b) | medium | Disable Kernel Parameter for Accepting Source-Routed Packets on IPv6 Interfaces | sysctl |
| `tftp_uses_secure_mode_systemd` | CCE-86495-9 | IA-5 (1) (c) | medium | Ensure tftp systemd Service Uses Secure Mode - Find valid drop-ins | other |
| `usbguard_generate_policy` | CCE-88632-5 | CM-8(3)(a), IA-3 | medium | Enable service usbguard | other |
| `use_kerberos_security_all_exports` | CCE-90391-4 | AC-17(a), CM-6(a), CM-7(a), CM-7(b), IA-2, IA-2(8), IA-2(9) | medium | Drop any security clause for every export | other |
| `use_pam_wheel_for_su` | CCE-90595-0 |  | medium | Restrict usage of su command only to members of wheel group | other |
| `wireless_disable_interfaces` | CCE-88576-4 | AC-18(3), AC-18(a), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium | Ensure NetworkManager is installed | other |
| `xwindows_runlevel_target` | CCE-89266-1 | CM-6(a), CM-7(a), CM-7(b) | medium | Switch to multi-user runlevel | other |
