#!/usr/bin/env bash
set -euo pipefail

# Local Windows packaging helper (Git Bash compatible)
# - Builds Tauri release
# - Renames Windows artifacts to unified names:
#   "<ProductName>_<version>_x64.msi" and "<ProductName>_<version>_x64.exe"

ROOT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

PRODUCT_NAME=$(node -e "console.log(require('./src-tauri/tauri.conf.json').productName)")
VERSION=$(node -e "console.log(require('./package.json').version)")

echo "Packaging $PRODUCT_NAME v$VERSION (Windows x64)"

if [[ "${SKIP_BUILD:-}" != "1" ]]; then
  npm run -s tauri build
fi

MSI_DIR="src-tauri/target/release/bundle/msi"
NSIS_DIR="src-tauri/target/release/bundle/nsis"

# Normalize MSI name (remove locale suffix like _en-US)
if compgen -G "$MSI_DIR/$PRODUCT_NAME_${VERSION}_x64_*.msi" > /dev/null; then
  MSI_SRC=$(ls "$MSI_DIR/$PRODUCT_NAME"_"${VERSION}"_x64_*.msi | head -n1)
  MSI_DST="$MSI_DIR/$PRODUCT_NAME"_"${VERSION}"_x64.msi
  echo "Renaming MSI: $(basename "$MSI_SRC") -> $(basename "$MSI_DST")"
  mv -f "$MSI_SRC" "$MSI_DST" || true
fi

# Normalize NSIS name (drop -setup suffix)
if [[ -f "$NSIS_DIR/$PRODUCT_NAME"_"${VERSION}"_x64-setup.exe ]]; then
  EXE_SRC="$NSIS_DIR/$PRODUCT_NAME"_"${VERSION}"_x64-setup.exe
  EXE_DST="$NSIS_DIR/$PRODUCT_NAME"_"${VERSION}"_x64.exe
  echo "Copying NSIS: $(basename "$EXE_SRC") -> $(basename "$EXE_DST")"
  cp -f "$EXE_SRC" "$EXE_DST"
fi

echo "Done. Artifacts:"
ls -l "$MSI_DIR" || true
ls -l "$NSIS_DIR" || true


