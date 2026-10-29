#!/bin/sh
set -eu
mkdir -p /app/.resuma
chown -R 10001:10001 /app/.resuma
exec setpriv --reuid=10001 --regid=10001 --clear-groups /app/website
