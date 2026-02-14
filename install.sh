#!/usr/bin/env bash
set -euo pipefail

REPO_URL="https://The-HaiKaw-Pr0tocol.github.io/RushX"
KEY_URL="${REPO_URL}/public.gpg"
KEYRING="/usr/share/keyrings/rushx-archive-keyring.gpg"
LIST_FILE="/etc/apt/sources.list.d/rushx.list"

if [[ ${EUID} -ne 0 ]]; then
  echo "Please run as root (use sudo)." >&2
  exit 1
fi

if ! command -v curl >/dev/null 2>&1; then
  apt-get update
  apt-get install -y curl
fi

if ! command -v gpg >/dev/null 2>&1; then
  apt-get update
  apt-get install -y gpg
fi

install -m 0755 -d /usr/share/keyrings
curl -fsSL "${KEY_URL}" | gpg --dearmor -o "${KEYRING}"

echo "deb [signed-by=${KEYRING}] ${REPO_URL}/ stable main" > "${LIST_FILE}"
apt-get update