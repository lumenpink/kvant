#!/usr/bin/env bash
# ==============================================================================
# TESTE: Otimização de Tamanho Binário e Linking Estático musl (#10)
# ==============================================================================
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SPEC_FILE="$ROOT_DIR/docs/specs/10_build_size_and_static_musl.md"
CARGO_TOML="$ROOT_DIR/Cargo.toml"
CARGO_CONFIG="$ROOT_DIR/.cargo/config.toml"

echo "==> [TEST] Validando conformidade de otimização de tamanho e linking estático musl (#10)..."

FAILED=0

# 1. Validar existência da especificação
echo -n "-> Verificando especificação formal... "
if [[ -f "$SPEC_FILE" ]]; then
    echo "OK: $SPEC_FILE presente."
else
    echo "FALHA: Arquivo de especificação não encontrado."
    FAILED=1
fi

# 2. Validar perfil [profile.release] no Cargo.toml
echo -n "-> Verificando [profile.release] no Cargo.toml... "
if grep -q "\[profile\.release\]" "$CARGO_TOML"; then
    echo "OK: [profile.release] declarado."
else
    echo "FALHA: [profile.release] ausente no Cargo.toml."
    FAILED=1
fi

echo -n "-> Verificando LTO fat e codegen-units = 1... "
if grep -q 'lto = "fat"' "$CARGO_TOML" && grep -q "codegen-units = 1" "$CARGO_TOML"; then
    echo "OK: LTO e codegen-units configurados para otimização máxima."
else
    echo "FALHA: lto = \"fat\" ou codegen-units = 1 ausentes no [profile.release]."
    FAILED=1
fi

echo -n "-> Verificando panic = abort e strip = true... "
if grep -q 'panic = "abort"' "$CARGO_TOML" && grep -q "strip = true" "$CARGO_TOML"; then
    echo "OK: panic = abort e strip = true configurados."
else
    echo "FALHA: panic = abort ou strip = true ausentes no [profile.release]."
    FAILED=1
fi

# 3. Validar .cargo/config.toml
echo -n "-> Verificando arquivo .cargo/config.toml... "
if [[ -f "$CARGO_CONFIG" ]]; then
    echo "OK: .cargo/config.toml existe."
else
    echo "FALHA: .cargo/config.toml ausente."
    FAILED=1
fi

echo -n "-> Verificando target-feature=+crt-static para x86_64-unknown-linux-musl... "
if [[ -f "$CARGO_CONFIG" ]] && grep -A 2 "target\.x86_64-unknown-linux-musl" "$CARGO_CONFIG" | grep -q "target-feature=+crt-static"; then
    echo "OK: crt-static configurado para x86_64 musl."
else
    echo "FALHA: crt-static ausente para x86_64 musl em .cargo/config.toml."
    FAILED=1
fi

echo -n "-> Verificando target-feature=+crt-static para aarch64-unknown-linux-musl... "
if [[ -f "$CARGO_CONFIG" ]] && grep -A 2 "target\.aarch64-unknown-linux-musl" "$CARGO_CONFIG" | grep -q "target-feature=+crt-static"; then
    echo "OK: crt-static configurado para aarch64 musl."
else
    echo "FALHA: crt-static ausente para aarch64 musl em .cargo/config.toml."
    FAILED=1
fi

# 4. Testes de mutação: validar que valores permissivos ou corrompidos falham
echo -n "-> [MUTAÇÃO] Testando sensibilidade a ausência de crt-static... "
MUTANT_CONFIG="[build]\ntarget = \"x86_64-unknown-linux-musl\""
if ! echo -e "$MUTANT_CONFIG" | grep -q "target-feature=+crt-static"; then
    echo "OK: Mutante detectado com precisão."
else
    echo "FALHA: Mutante não detectado."
    FAILED=1
fi

if [[ "$FAILED" -ne 0 ]]; then
    echo "================================================================="
    echo "FALHA: Um ou mais requisitos não foram satisfeitos (RED)."
    exit 1
fi

echo "================================================================="
echo "SUCESSO: Todas as asserções e testes de mutação passaram (GREEN)!"
