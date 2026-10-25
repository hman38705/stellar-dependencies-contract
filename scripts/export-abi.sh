#!/usr/bin/env bash
#
# export-abi.sh
#
# Builds the contracts with `forge build` and exports the ABI of
# ShortlistRegistry to a stable, versioned path: abi/ShortlistRegistry.json
#
# This file is intended to be attached to releases so that backend and
# frontend consumers can integrate against a stable contract interface.
#
# Usage:
#   ./scripts/export-abi.sh
#
# Requirements:
#   - forge (Foundry)
#   - jq
#
set -euo pipefail

# Resolve the repository root regardless of where the script is invoked from.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

CONTRACT_NAME="ShortlistRegistry"
ARTIFACT_PATH="${REPO_ROOT}/out/${CONTRACT_NAME}.sol/${CONTRACT_NAME}.json"
ABI_DIR="${REPO_ROOT}/abi"
ABI_PATH="${ABI_DIR}/${CONTRACT_NAME}.json"

# Fail loudly if required tooling is unavailable.
if ! command -v forge >/dev/null 2>&1; then
  echo "error: 'forge' not found in PATH. Install Foundry: https://getfoundry.sh" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "error: 'jq' not found in PATH. Install jq to export the ABI." >&2
  exit 1
fi

echo "==> Building contracts with forge build"
(
  cd "${REPO_ROOT}"
  forge build
)

# Fail loudly if the compiled artifact is missing.
if [[ ! -f "${ARTIFACT_PATH}" ]]; then
  echo "error: compiled artifact not found at ${ARTIFACT_PATH}" >&2
  echo "       Did 'forge build' succeed and does ${CONTRACT_NAME}.sol exist?" >&2
  exit 1
fi

# Ensure the output directory exists.
mkdir -p "${ABI_DIR}"

echo "==> Extracting ABI from ${ARTIFACT_PATH}"
if ! jq '.abi' "${ARTIFACT_PATH}" > "${ABI_PATH}"; then
  echo "error: failed to extract ABI from ${ARTIFACT_PATH}" >&2
  exit 1
fi

# Sanity check: the ABI must be a non-empty JSON array.
if ! jq -e 'type == "array" and length > 0' "${ABI_PATH}" >/dev/null 2>&1; then
  echo "error: extracted ABI at ${ABI_PATH} is not a non-empty JSON array" >&2
  exit 1
fi

echo "==> ABI exported to ${ABI_PATH}"