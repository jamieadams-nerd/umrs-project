#!/usr/bin/env bash
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
set -euo pipefail

if [[ ${EUID:-$(id -u)} -eq 0 ]]; then
  echo "Do not run as root."
  exit 1
fi

if ! command -v sudo >/dev/null 2>&1; then
  echo "sudo not found."
  exit 1
fi

# Prefer chrony; remove timesyncd if present
sudo systemctl stop systemd-timesyncd || true
sudo systemctl disable systemd-timesyncd || true
sudo apt remove -y systemd-timesyncd || true

sudo apt update

# Preseed mail-free fail2ban config to prevent Postfix pull-in
sudo mkdir -p /etc/fail2ban
sudo tee /etc/fail2ban/jail.local >/dev/null <<'EOF'
[DEFAULT]
banaction = nftables-multiport
action = %(action_)s
destemail =
sender =
mta =
EOF

# UMRS ops baseline packages (no Postfix)
sudo apt install -y auditd audispd-plugins rsyslog logrotate chrony dbus apparmor apparmor-utils fail2ban sudo coreutils util-linux procps psmisc net-tools iproute2 tcpdump lsof openssl libssl-dev gnutls-bin gnupg ca-certificates haveged rng-tools5 tpm2-tools tpm2-abrmd softhsm2 opensc aide attr acl inotify-tools xattr btrfs-progs e2fsprogs ufw nftables iptables conntrack traceroute nmap systemd-coredump libcap2-bin seccomp cgroup-tools dbus-user-session selinux-utils selinux-basics checkpolicy setools

# Fix Ubuntu AIDE packaging bug (_aide user missing)
sudo useradd --system --no-create-home --shell /usr/sbin/nologin _aide || true
sudo systemd-tmpfiles --create /usr/lib/tmpfiles.d/aide-common.conf || true

# Enable core services
sudo systemctl enable --now auditd
sudo systemctl enable --now rsyslog
sudo systemctl enable --now chrony
sudo systemctl enable --now fail2ban
sudo systemctl enable --now tpm2-abrmd || true

# Restart for clean state
sudo systemctl restart auditd
sudo systemctl restart rsyslog
sudo systemctl restart chrony
sudo systemctl restart fail2ban

echo "OK: UMRS ops baseline packages installed."
echo "OK: chrony selected as time daemon."
echo "OK: AIDE system user initialized."
echo "OK: fail2ban installed without mail dependency."
echo "NOTE: TPM daemon may be inactive on VMs (expected)."
echo "NOTE: Postfix was not installed or modified."
echo "NOTE: Ubuntu has no real FIPS mode without Ubuntu Pro."

