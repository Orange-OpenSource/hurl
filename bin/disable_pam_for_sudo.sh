#!/bin/bash
set -Eeuo pipefail

echo "----- Disable PAM for sudo -----"
{
echo "auth sufficient pam_permit.so"
echo "account sufficient pam_permit.so"
echo "session sufficient pam_permit.so"
} > /etc/pam.d/sudo
cat /etc/pam.d/sudo

