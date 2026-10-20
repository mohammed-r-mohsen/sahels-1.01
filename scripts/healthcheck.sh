#!/bin/bash
set -euo pipefail

mkdir -p /opt/sahels/logs
chmod 700 /opt/sahels

echo "Sahels healthcheck started"

curl -fsS http://localhost:8080/health > /dev/null

echo "✅ API healthcheck passed"
