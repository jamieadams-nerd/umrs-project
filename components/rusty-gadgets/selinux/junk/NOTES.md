Yes — and this is exactly the right instinct.

You **should not** be hand-writing low-level permission sets like:

```
read getattr open
```

unless you are doing extremely surgical policy work.

SELinux ships a very rich macro/interface layer specifically to avoid that verbosity and reduce error surface.

---

# 1️⃣ File / Dir access macros (what you’re looking for)

These come primarily from:

```
/usr/share/selinux/devel/include/
```

Loaded automatically when you build policy modules.

---

## Read-only file access

Instead of:

```
allow lei_officer_t cui_lei_data_ro_t:file { read getattr open };
```

Use:

```
read_files_pattern(lei_officer_t, cui_lei_data_ro_t, cui_lei_data_ro_t)
```

or simpler:

```
allow lei_officer_t cui_lei_data_ro_t:file read_file_perms;
```

---

## Read-only directory access

Instead of:

```
allow lei_officer_t cui_lei_data_ro_t:dir { search getattr open };
```

Use:

```
allow lei_officer_t cui_lei_data_ro_t:dir read_dir_perms;
```

---

# 2️⃣ Common permission sets

These are the most frequently used:

| Macro               | Expands to              |
| ------------------- | ----------------------- |
| `read_file_perms`   | read, getattr, open     |
| `write_file_perms`  | write, append           |
| `create_file_perms` | create, write, setattr  |
| `rw_file_perms`     | read + write            |
| `read_dir_perms`    | search, getattr, open   |
| `rw_dir_perms`      | read + add/remove names |

So your RO custody becomes:

```
allow lei_officer_t cui_lei_data_ro_t:file read_file_perms;
allow lei_officer_t cui_lei_data_ro_t:dir  read_dir_perms;
```

Much cleaner.

---

# 3️⃣ Higher-level pattern macros

Even better — pattern macros bundle file + dir logic.

Example:

```
read_files_pattern(lei_officer_t, cui_lei_data_ro_t, cui_lei_data_ro_t)
```

This expands to:

* dir search
* file read
* getattr
* open

All in one.

---

## RW pattern

```
rw_files_pattern(lei_officer_t, cui_lei_data_rw_t, cui_lei_data_rw_t)
```

Covers:

* read
* write
* append
* create
* remove
* rename

---

# 4️⃣ When to use which

### Use permission sets when:

You want surgical control.

Example:

```
allow lei_officer_t cui_lei_data_ro_t:file { read getattr };
```

---

### Use pattern macros when:

You’re modeling storage custody.

Example vault access:

```
read_files_pattern(lei_officer_t, cui_lei_data_ro_t, cui_lei_data_ro_t)
rw_files_pattern(lei_officer_t, cui_lei_data_rw_t, cui_lei_data_rw_t)
```

This is the normal design.

---

# 5️⃣ File vs dir vs lnk vs sock

There are variants:

| Pattern                  | Scope          |
| ------------------------ | -------------- |
| `read_files_pattern`     | files + dirs   |
| `manage_files_pattern`   | full lifecycle |
| `create_files_pattern`   | creation only  |
| `read_lnk_files_pattern` | symlinks       |

So if vaults contain symlinks:

```
read_lnk_files_pattern(lei_officer_t, cui_lei_data_ro_t, cui_lei_data_ro_t)
```

---

# 6️⃣ Why macros matter (assurance angle)

Hand-written perms are brittle.

You forget one:

* getattr missing → tools fail
* open missing → weird denials
* search missing → directory invisible

Macros encode:

* Correct minimum perms
* Community-reviewed patterns
* Future compatibility

They are policy engineering guardrails.

---

# 7️⃣ Where they are defined

Look at:

```
/usr/share/selinux/devel/include/file_patterns.spt
/usr/share/selinux/devel/include/obj_perm_sets.spt
```

Examples inside:

```
define(`read_file_perms', `{ read getattr open }')
define(`read_dir_perms', `{ search getattr open }')
```

So your instinct was exactly right — they’re abstractions over those raw sets.

---

# 8️⃣ Your LEI example — clean policy

## cui_lei_access.te

```
policy_module(cui_lei_access, 1.0)

require {
    type lei_officer_t;
    type cui_lei_data_ro_t;
    type cui_lei_data_rw_t;
}

########################################
# Read-only custody
########################################

read_files_pattern(
    lei_officer_t,
    cui_lei_data_ro_t,
    cui_lei_data_ro_t
)

########################################
# Read / write custody
########################################

rw_files_pattern(
    lei_officer_t,
    cui_lei_data_rw_t,
    cui_lei_data_rw_t
)
```

Readable. Auditable. Minimal error surface.

---

# 9️⃣ Practical design guidance

Use this layering:

| Layer             | Macro style      |
| ----------------- | ---------------- |
| Platform          | pattern macros   |
| Compartments      | pattern macros   |
| Tools / daemons   | interface macros |
| Surgical controls | raw perms        |

High-assurance MLS systems almost never hand-write base perms.

---

If you want next step, we should:

* Define `lei_officer_r`
* Map it to `lei_officer_t`
* Add login mapping via `semanage login`

That completes the identity → custody enforcement chain.





Base custody (tooling layer)
umrs.te
umrs.if

Classification custody
umrs_cui.te
umrs_cui.if

Future compartment layers
cui_lei.te
cui_agr.te
cui_cti.te


All inheriting the attribute + interface model you just built.



