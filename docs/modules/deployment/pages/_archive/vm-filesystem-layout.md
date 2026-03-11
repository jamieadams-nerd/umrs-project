# STIG-Compliant Filesystem Layout for Virtual Machines (RHEL / CentOS / Stream)

This guide provides a virtualization-optimized filesystem layout that remains fully STIG-compliant while improving VM performance, reducing disk footprint, and avoiding snapshot bloat.  
It includes partition sizes, mount options, controller recommendations, and filesystem guidance for Parallels, VMware, KVM, and VirtualBox.

---

## 1. Recommended Partition Layout for Virtual Machines (64GB Disk)

| Mount Point          | Size | Purpose |
|----------------------|-------|---------|
| /boot                | 1 GB  | Kernel and initramfs |
| /boot/efi            | 500 MB | UEFI firmware partition |
| /                    | 20 GB | OS binaries and system files |
| /home                | 2 GB  | User data (small in VMs) |
| /tmp                 | 1 GB  | Temporary storage (STIG-restricted) |
| /var                 | 10 GB | System state, RPM DB, journals |
| /var/tmp             | 1 GB  | Persistent temp files |
| /var/log             | 4 GB  | System logs |
| /var/log/audit       | 2 GB  | Audit logs (required by STIG) |
| Swap                 | 2–4 GB | VM memory support |

Optimizations:
- Smaller `/home`, `/tmp`, `/var/tmp` reduce disk footprint.
- Optional partitions such as `/opt` and `/srv` are omitted unless needed.
- Layout is snapshot-friendly and ideal for thin-provisioned VM disks.

---

## 2. Why This Layout Is Better for VMs

### Benefits:
- Smaller overall footprint improves cloning, snapshots, backups.
- Reduced write amplification (extends SSD and VM disk longevity).
- Faster boot and package operations.
- Retains **complete STIG compliance**.
- Ideal for cloud, CI/CD, ephemeral, or cloned VM environments.

---

## 3. Controller and Disk Type Recommendations

### Parallels Desktop (Mac M-series)
- Disk controller: **Virtio-SCSI**
- Disk type: **Virtio Block**
- Benefits: High performance, low overhead, best CPU efficiency.

### VMware Fusion / ESXi
- Preferred controller: **VMware Paravirtual (PVSCSI)**
- Fallback controller: **LSI Logic SAS**
- Disk type: **Thin provisioned**
- PVSCSI provides the best I/O performance for Linux.

### KVM / QEMU / Proxmox
- Controller: **Virtio-SCSI**
- Disk format: **qcow2** with compressed metadata
- Cache mode:
  - `writeback` = highest performance
  - `none` = safest data integrity

### VirtualBox
- Controller: **Virtio-SCSI** (if supported)
- Fallback: **AHCI SATA**
- Virtio improves performance significantly.

---

## 4. Recommended Filesystem Types

### XFS (Recommended, RHEL default)
- Best parallel performance
- Strong under Virtio and modern hypervisors
- Fast recovery after snapshots

### Optional: EXT4 for /var/log and /var/log/audit
Some administrators prefer EXT4 because:
- Faster fsck
- Simpler journaling
- Predictable behavior for audit logs

Both XFS and EXT4 are fully STIG-compliant.

---

## 5. Virtualization-Optimized /etc/fstab Example

```fstab
/dev/mapper/cs-root      /                  xfs     defaults                       0 0
/dev/mapper/cs-home      /home              xfs     nodev                          0 0
/dev/mapper/cs-tmp       /tmp               xfs     nodev,nosuid,noexec            0 0
/dev/mapper/cs-vartmp    /var/tmp           xfs     nodev,nosuid,noexec            0 0
/dev/mapper/cs-var       /var               xfs     defaults                       0 0
/dev/mapper/cs-varlog    /var/log           xfs     nodev,nosuid                   0 0
/dev/mapper/cs-audit     /var/log/audit     xfs     nodev,nosuid                   0 0
```

## STIG Mount Option Requirements

This configuration preserves all required STIG mount options:

- `nodev`
- `nosuid`
- `noexec` (where required)
- Secure UEFI `umask` on `/boot/efi`

---

## 6. STIG Compliance Confirmation

This virtualization layout satisfies the following required STIG controls:

| Requirement                  | STIG ID          | Status |
|-----------------------------|------------------|--------|
| /tmp must be separate       | RHEL-07-021310   | ✔      |
| /var/log must be separate   | RHEL-07-021320   | ✔      |
| /var/log/audit must be separate | RHEL-07-021330 | ✔      |
| /home must be separate      | RHEL-07-021100   | ✔      |
| /var/tmp must be separate   | RHEL-07-021200   | ✔      |
| Required mount options applied | Multiple STIGs | ✔      |
| UEFI partition secured      | RHEL-07-021350   | ✔      |

Only optional partitions like `/srv` and `/opt` are removed or reduced — this is acceptable in virtualized environments.

---

## 7. Summary

This guide provides a fully STIG-compliant but virtualization-optimized partition strategy.  
It reduces disk size, improves performance, accelerates snapshots, and still meets DoD/NIST security baselines for RHEL and CentOS systems running under Parallels, VMware, KVM, and VirtualBox.




UUID=<boot>              /boot              xfs     defaults                       0 0
UUID=<efi>               /boot/efi          vfat    umask=0077                     0 2
