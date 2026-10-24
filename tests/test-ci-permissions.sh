#!/bin/bash
set -eo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKFLOWS_DIR="${ROOT_DIR}/.forgejo/workflows"

echo "=== [TEST] Auditoria de ausência de permissions no Forgejo CI (kvant) ==="

audit_workflow_file() {
    local file="$1"
    if grep -nE '^[[:space:]]*permissions:' "$file"; then
        return 1
    fi
    return 0
}

FAILED=0

for wf in "${WORKFLOWS_DIR}"/*.yml "${WORKFLOWS_DIR}"/*.yaml; do
    [ -f "$wf" ] || continue
    if ! audit_workflow_file "$wf"; then
        echo "  [FAIL] Campo 'permissions:' incompatível detectado em $(basename "$wf")"
        FAILED=1
    else
        echo "  [PASS] $(basename "$wf") livre de diretiva 'permissions:'"
    fi
done

if [ $FAILED -ne 0 ]; then
    echo "❌ [RED TEST] Auditoria falhou: encontrada diretiva permissions incompatível com Forgejo."
    exit 1
fi

echo "=== [MUTATION] Testes de Mutação (Sensibilidade a permissions) ==="
TMP_MUTANT=$(mktemp --suffix=.yml)
trap 'rm -f "$TMP_MUTANT"' EXIT

cat << 'EOF' > "$TMP_MUTANT"
name: Mutante Top
on: push
permissions:
  contents: write
jobs:
  test:
    runs-on: ubuntu-latest
EOF

if audit_workflow_file "$TMP_MUTANT" >/dev/null 2>&1; then
    echo "  [FAIL] Mutante 1 não foi detectado!"
    exit 1
else
    echo "  [PASS] Mutante 1 (permissions no topo) detectado com sucesso"
fi

cat << 'EOF' > "$TMP_MUTANT"
name: Mutante Job
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    permissions:
      contents: read
EOF

if audit_workflow_file "$TMP_MUTANT" >/dev/null 2>&1; then
    echo "  [FAIL] Mutante 2 não foi detectado!"
    exit 1
else
    echo "  [PASS] Mutante 2 (permissions em job) detectado com sucesso"
fi

echo "✔ [GREEN TEST] Todos os workflows Forgejo e testes de mutação aprovados."
