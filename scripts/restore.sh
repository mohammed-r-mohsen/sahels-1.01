#!/bin/bash
set -euo pipefail

BACKUP_DIR="${BACKUP_DIR:-/opt/sahels/backups}"
LATEST_FILE=$(find "$BACKUP_DIR" -name "sahels_*.sql.gz" | sort | tail -n 1)

if [ -z "$LATEST_FILE" ]; then
  echo "❌ No backup file found in $BACKUP_DIR"
  exit 1
fi

gunzip -c "$LATEST_FILE" | docker compose exec -T postgres psql -U sahels sahels

echo "✅ Restore completed from ${LATEST_FILE}"
