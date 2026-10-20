#!/bin/bash
set -euo pipefail

BACKUP_DIR="${BACKUP_DIR:-/opt/sahels/backups}"
RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-30}"
mkdir -p "$BACKUP_DIR"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
FILE="$BACKUP_DIR/sahels_${TIMESTAMP}.sql.gz"

docker compose exec -T postgres pg_dump -U sahels sahels | gzip > "$FILE"

find "$BACKUP_DIR" -name "sahels_*.sql.gz" -mtime +"$RETENTION_DAYS" -delete

echo "✅ Backup created: ${FILE}"
