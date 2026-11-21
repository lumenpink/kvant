# Especificação do Ticket #18: Delegar Entregas ao Kombinat via Generic Packages

## 1. Contexto e Motivação
O workflow `.forgejo/workflows/ci.yml` publicava diretamente no Cloudflare (R2/Pages/Workers) e/ou clonava a Banca para rotacionar manifestos. Com o Kombinat orquestrando entregas a partir dos hooks da Gambi, a CI deve publicar artefatos e sites exclusivamente nos Generic Packages da forja via `cccp package upload`.

## 2. Requisitos
1. Remover chamadas diretas a `wrangler`, `CLOUDFLARE_API_TOKEN` e `git clone ... banca.git` de `.forgejo/workflows/ci.yml`.
2. Publicar artefatos compilados e `site/` via `cccp package upload` no Generic Package Registry da Gambi.
3. Validar `cccp check --strict` com 100% de conformidade.

## 3. Árvore de Decisões (para a Natasha)
- **Zero segredos externos na CI:** Credenciais Cloudflare/R2 ficam confinadas ao Kombinat; a CI usa apenas o token da forja (`GAMBI_2LP_IN_TOKEN`).
