# ☭ Normas e Diretrizes Compiladas do Repositório

> **Aviso de Integridade:**
> Este documento foi montado automaticamente pelo CCCP (`cccp sync` / `assemble`).
> A precedência é estrita de baixo para cima: cada camada subsequente sobrepõe as definições anteriores.

### 🌳 Árvore de Herança e Precedência Resolvida:
1. Camada 1: `https://kollontai.2lp.in/2.2.0/RULES.md`
2. Camada 2: `https://lilya.2lp.in/2.2.0/RULES.md`

---

<!-- ===================================================================== -->
<!-- CAMADA 1: https://kollontai.2lp.in/2.2.0/RULES.md -->
<!-- ===================================================================== -->

---
standard: "kollontai"
tier: "tactical"
version: "2.2.0"
locale: "pt-br"
authority: "Standards Committee"
published_at: "2026-11-08"
inherits_from:
  standard: "gosplan"
  tier: "strategic"
  version: "1.0.0"
  base_url: "https://gosplan.2lp.in/1.0.0/"
---

# Perfil de Implementação GOST: Kollontai

> **Sobre este Perfil:**  
> O perfil **Kollontai** (em homenagem à revolucionária soviética, feminista e diplomata Alexandra Kollontai) define o ferramental tático de engenharia, os contratos de interface do ecossistema de governança, a matriz de versões de compiladores, o protocolo de auto-atualização de binários e a arquitetura de distribuição para projetos aderentes ao padrão **GOST**.  
> Ele é agnóstico a domínios pessoais e identidades individuais, servindo como perfil base para coletivos, laboratórios ou perfis autorais derivados (como o perfil Lilya).

---

## 1. Contratos e Interfaces do Ecossistema Tático de Governança

No perfil Kollontai, a governança definida na norma GOST é decomposta em cinco interfaces táticas funcionais e universais:

### 1.1 Contrato da Ferramenta de Ciclo de Vida Local (Maestro Local)
Responsável exclusivo pela orquestração do ciclo de vida local do repositório, execução das 9 fases DD-TDD, interceptação de convenções de commit, higiene de código e integração segura de branches.

#### Contrato de Comandos do Maestro Local:
| Comando / Ação | Fase / Escopo | Contrato Funcional |
| :--- | :---: | :--- |
| `start <id> [--base <b>]` | **Fase 2** | Lê os metadados do ticket `#<id>`, atualiza a base (`main`, `vN` ou `epic/*`) e cria/alterna para a branch de trabalho `<tipo>/<id>-<slug>`. |
| `status` | **Fases 1–9** | Inspeciona o histórico da branch ativa, audita a fase alcançada (`ph3`..`ph9`) e indica o próximo passo obrigatório. |
| `commit "<resumo>"` | **Fases 3–8** | Deduz o ticket e a fase atual (`ph3`..`ph8`), valida as travas da fase e grava o commit convencional com `<tipo>: <resumo> (ref #<id> - ph<N>)`. |
| `mutate` | **Fase 7** | Executa a ferramenta de teste de mutação da linguagem restrita ao diff da branch. |
| `hygiene` / `fix` | **Fase 8** | Executa formatadores determinísticos e linters estáticos com tolerância zero a avisos (`-D warnings`). |
| `close "<resumo>"` | **Fase 9** | Exige o corpo com o **Laudo Textual de Resolução** (`[Resumo]:`, `[Solução]:`, `[Validação]:`) e grava o commit canônico de fechamento `(closes #<id> - ph9)`. |
| `merge` | **Pós-Fase 9** | Valida a presença da tag canônica `(closes #<id>)` e integra localmente a branch de trabalho na base (`main`), eliminando a branch local efêmera. |
| `push [--multidev]` | **Publicação** | Bloqueia push de branches efêmeras de ticket e envia branches persistentes (`main`, `vN`, `epic/*`) ao remote autorizado. |
| `release <patch|minor|major>` | **Release** | Executa auditoria pré-tag, consolida `CHANGELOG.md`, atualiza `VERSION` e sela a tag semântica. |

### 1.2 Contrato da Ferramenta de Governança de Forja e Abastecimento
Responsável pela interface com forjas remotas (API), ciclo de vida de issues/tickets, triagem assistida e catalogação multi-repositório.

#### Contrato de Comandos de Forja e Abastecimento:
| Comando / Ação | Finalidade | Contrato Funcional |
| :--- | :--- | :--- |
| `init` / `new` | **Dia Zero** | Inicializa repositório local e remoto na forja primária, gera anatomia inicial e registra no catálogo. |
| `ticket create` | **Abertura** | Cria issue na forja com tipo semântico, contexto e critérios de aceite. |
| `ticket list` | **Consulta** | Lista issues ativas com filtros por estado, marco ou responsável. |
| `ticket comment` | **Comentário** | Insere anotações ou laudos de progresso na issue remota. |
| `ticket close` | **Encerramento** | Fecha formalmente a issue na forja com mensagem conclusiva. |
| `divan` / `triage` | **Triagem Assistida** | Sessão interativa assistida por modelo de inteligência para refinamento e planejamento. |
| `catalog scan` / `list` | **Inventário** | Varre workspaces e cataloga repositórios e seus metadados (`.gost/metadata.json`). |
| `batch push` | **Operação em Lote** | Executa envio de atualizações sincronizadas através dos repositórios do catálogo. |

### 1.3 Contrato do Validador Normativo de Conformidade
Responsável pela avaliação de regras declarativas (`rules.toml`) e emissão de laudos padronizados de conformidade.

#### Contrato de Comandos do Validador:
| Comando / Ação | Contrato Funcional |
| :--- | :--- |
| `check [--strict] [--lang <l>] [--scope <s>] [--json]` | Avalia regras declaradas com filtros de escopo (`project`, `post-build`, etc.) e emite laudos em texto ou JSON. Retorna código 0 em aprovação e código 1 em falha (ou avisos em modo estrito). |
| `test` | Valida sintaxe e integridade das regras declarativas do catálogo local. |
| `eval <expressão>` | Executa script ou expressão isolada com acesso aos primitivos de runtime normativos. |

### 1.4 Contrato da Ferramenta de Entrega Contínua e Despacho de Artefatos (CD)
Responsável pelo pipeline de entrega contínua após a aprovação da integração contínua (CI), garantindo que nenhum binário de produção seja compilado ou empacotado em estações de trabalho não-reprodutíveis.

#### Contrato Funcional de Entrega Contínua:
| Operação / Capacidade | Finalidade | Contrato Funcional |
| :--- | :--- | :--- |
| `ingest` | **Ingestão de Staging** | Consome binários e pacotes compilados na esteira hermética de CI da forja e valida suas assinaturas e hashes criptográficos SHA-256. |
| `dispatch` | **Despacho Externo** | Publica os artefatos nos storages autorizados de distribuição e atualiza os canais de entrega semântica (`stable`, `dev`). |
| `purge` | **Higiene de Staging** | Remove imediatamente da forja primária os pacotes temporários de staging recém-despachados, prevenindo saturação de disco. |
| `deploy-stack` | **Deploy de Servidores** | Aciona remotamente a atualização de serviços conteinerizados via SSH na rede privada com verificação de healthcheck e rollback automático. |
| `cascade` | **Propagação em Cascata** | Propaga atualizações de versão a montante (*upstream*) para os manifestos de dependência dos módulos a jusante (*downstream*). |

### 1.5 Contrato do Controlador de Fronteira e Sincronia de Espelhos
Responsável pela proteção e telemetria nas fronteiras públicas do ecossistema, isolando contribuições não-confiáveis e gerando paridade entre espelhos.

#### Contrato Funcional de Fronteira:
| Operação / Capacidade | Finalidade | Contrato Funcional |
| :--- | :--- | :--- |
| `sync-infra` | **Sincronia de Espelhos** | Configura espelhos de envio passivo (`pushurl = no_push`), garantindo paridade de metadados, descrição e tópicos. |
| `reconcile` | **Reconciliação Anti-Drift** | Executa conciliação periódica de eventos de forja e webhooks perdidos através de buffers de lookback. |
| `inbox-quarantine` | **Quarentena de PRs** | Isola Pull Requests externos de espelhos públicos no namespace de quarentena `refs/heads/inbox/*` antes de qualquer execução de CI. |
| `inbox-clean` | **Higiene de Fronteira** | Remove branches locais `inbox/*` associadas a Pull Requests já encerrados ou rejeitados. |

---

## 2. Topologia de Forjas, Espelhos e Fronteira

### 2.1 Autoridade da Forja Primária e Espelhamento Passivo
1. **Unidirecionalidade Local (`Zero Dual Push`):** O ambiente local mantém o remote `origin` apontando exclusivamente para a forja primária definida no perfil final. A replicação de código para espelhos externos (Codeberg, GitHub) é responsabilidade exclusiva do *Push Mirror* automático da forja primária (`sync_on_commit: true`).
2. **Sinalização de Espelhos e Paridade de Metadados:**
   - O comando `cccp gost sync` mantém `description` (extraído de `summary`), `website` e `topics` (extraídos de `tags` no `.gost/metadata.json`) idênticos entre a forja primária e os espelhos.
   - Nos espelhos secundários, a descrição recebe automaticamente o prefixo canônico:
     `[MIRROR / BACKUP] {summary} — Repositório canônico: {primary_forge_url}`
3. **Configuração Obrigatória de Remotes para Ingestão de PRs:**
   Para permitir que o controlador de fronteira baixe Pull Requests externos sem risco de push acidental para os espelhos, o `.git/config` **DEVE** ser mantido pelo `cccp` no seguinte formato:
   ```ini
   [remote "origin"]
       url = <primary_ssh_url>
       fetch = +refs/heads/*:refs/remotes/origin/*

   [remote "codeberg"]
       url = https://codeberg.org/<mirror_owner>/<slug>.git
       fetch = +refs/pull/*/head:refs/remotes/codeberg/pr/*
       pushurl = no_push

   [remote "github"]
       url = https://github.com/<mirror_owner>/<slug>.git
       fetch = +refs/pull/*/head:refs/remotes/github/pr/*
       pushurl = no_push
   ```

---

## 3. Matriz de Toolchains Homologadas (*Toolchain Pinning*)

As regras arquiteturais (DDD/Hexagonal), estruturas de diretórios, templates de `Dockerfile` e comandos de cada linguagem residem nos módulos `specs/languages/<lang>.md` da norma GOST. O perfil Kollontai fixa as versões mínimas homologadas para execução local e nas imagens da esteira:

| Stack (`GOST_LANGUAGES`) | Versão Canônica / Runtime | Runner de Testes & Cobertura (≥90%) | Mutação (Fase 7) | Linter (`-D warnings`) & Formatador |
| :--- | :--- | :--- | :--- | :--- |
| **`rust`** | `rustc >= 1.100.0` (`edition = "2024"`, target estático `musl`) | `cargo test` + `cargo-llvm-cov` | `cargo-mutants` (`--in-diff`) | `clippy` + `rustfmt` + `cargo-audit` |
| **`php`** | `PHP >= 8.5.0` (compatibilidade reversa `>= 8.2`, `strict_types=1`) | `pest --coverage --min=90` | `infection` (`--git-diff-lines`) | `phpstan --level=max` (target 8.2) + `pint` + `composer audit` |
| **`shell`** | POSIX `/bin/sh` estrito (ou `Bash >= 5.2` justificado) | `shellspec --kcov` | Revisão de borda (Boundary/Subshells) | `shellcheck` + `shfmt -d -i 2 -ci -bn` |
| **`javascript`** | `Node.js >= 24.16.0 LTS` (ESM nativo) + `TypeScript >= 5.8` (`strict`) | `vitest` (v8 coverage) + `playwright` (E2E) | `stryker` | `biome check` + `npm audit` |
| **`r`** | `R >= 4.4.1` + `renv` (lockfile imutável `renv.lock`) | `testthat` (3e) + `covr` | Revisão de invariantes (`NA`/`NaN`/`Inf`) | `lintr` + `styler` + `roxygen2` |
| **`go`** | `Go >= 1.22+` (cgo desabilitado para binário estático puro) | `go test -race` + `go tool cover` | `gremlins` | `golangci-lint` + `gofmt` + `govulncheck` |
| **`python`** | `Python >= 3.12+` (tipagem estrita mypy, Poetry/uv) | `pytest --cov` | `mutatest` | `ruff check/format` + `mypy --strict` + `pip-audit` |

### 3.1 Framework Canônico de Interface Gráfica: Dioxus 0.8
Para aplicações gráficas multiplataforma em Rust (Web WASM, Desktop via Tao/Wry e Mobile Android/iOS), o perfil Kollontai adota o **Dioxus 0.8** (`dioxus = "0.8"`):
* **Ergonomia Mobile Obrigatória:** Respeito a `env(safe-area-inset-top)` / `env(safe-area-inset-bottom)`, alvos de toque mínimos de `48px` com `touch-action: manipulation` e navegação adaptativa (barra inferior em telas `< 640px` e barra lateral compacta em `640px–1024px`).

---

## 4. Protocolo de Auto-Atualização (XDG) e Manifesto de Distribuição

Todas as ferramentas de linha de comando (CLI) e aplicações desktop distribuídas sob o perfil Kollontai **DEVEM** implementar o protocolo de auto-atualização conectado ao catálogo de distribuição central (a **Banca**) e interoperável com a **Jornaleira**.

### 4.1 Caminhos XDG e Formato de Configuração (`config.env`)
Para manter conformidade com a regra anti-bloat da norma GOST (Seção 13.2), a configuração e o cache de estado residem estritamente em caminhos XDG:
1. **Configuração do Usuário:** `${XDG_CONFIG_HOME:-~/.config}/<slug>/config.env`  
   Formato plano canônico `CHAVE="VALOR"` (compatível com POSIX Shell, Rust, Go e C):
   ```sh
   UPDATE_CHANNEL="stable"           # Canal rastreado: "stable" ou "nightly"
   UPDATE_PINNED="false"             # Se "true", bloqueia atualizações e silencia alertas
   UPDATE_PIN_VERSION=""             # Versão fixada quando UPDATE_PINNED="true"
   UPDATE_AUTO_CHECK="true"          # Verificação periódica não-bloqueante em modo autônomo
   UPDATE_CHECK_INTERVAL_HOURS="12"  # Frequência padrão de consulta (12 horas)
   UPDATE_AUTO_UPDATE="false"        # Substituição atômica sem confirmação (opt-in estrito)
   UPDATE_MANAGED_BY_JORNALEIRA=""   # Override: "" (auto-detectar), "true" ou "false"
   ```
   *Precedência de Resolução:* Flags CLI (`--channel`, `--pin`) > Variáveis de ambiente (`<SLUG_UPPER>_UPDATE_*` / `UPDATE_*`) > `${XDG_CONFIG_HOME}/<slug>/config.env` > Padrões normativos Kollontai.

2. **Estado e Cache de Runtime:** `${XDG_STATE_HOME:-~/.local/state}/<slug>/update_check.json`  
   Registro estruturado compartilhado entre o aplicativo e a Jornaleira:
   ```json
   {
     "last_check_timestamp": 1729411200,
     "current_version": "0.1.0",
     "latest_remote_version": "0.1.2",
     "channel": "stable",
     "update_available": true,
     "last_check_status": "success",
     "managed_by": "jornaleira"
   }
   ```

### 4.2 A Jornaleira como Gestora Primária e Comportamento em Runtime
1. **Hegemonia da Jornaleira:** A Jornaleira atua como a gestora primária do ecossistema.  
   * **Detecção:** Se `UPDATE_MANAGED_BY_JORNALEIRA="true"` ou se `command -v jornaleira >/dev/null 2>&1` retornar sucesso (sem override explícito `"false"`):
     * O **auto-check nativo dos aplicativos no startup permanece sumariamente desativado**.
     * Nenhuma requisição de rede ou consulta DNS é disparada durante a invocação normal de comandos do aplicativo.
     * A consulta remota periódica e a substituição de binários passam a ser orquestradas exclusivamente pelos timers e rotinas centrais da Jornaleira.
2. **Modo Autônomo (Sem Jornaleira):**  
   * Se a Jornaleira não estiver presente (ou `UPDATE_MANAGED_BY_JORNALEIRA="false"`), o utilitário assume a responsabilidade de verificar atualizações de forma não-bloqueante no startup caso o intervalo de 12 horas tenha expirado, com timeout estrito de $\le 1\text{s}$ descartando falhas silenciosamente.
3. **Alerta Não-Intrusivo em `stderr`:**  
   Se o arquivo `${XDG_STATE_HOME}/<slug>/update_check.json` indicar `update_available: true`, emite alerta exclusivamente em `stderr` (preservando pipelines em `stdout`):
   * *Quando sob gestão da Jornaleira:*
     ```text
     💡 [<slug>] Nova versão disponível: vX.Y.Z (atual: vA.B.C). Execute 'jornaleira update <slug>' para atualizar.
     ```
   * *Quando em modo autônomo:*
     ```text
     💡 [<slug>] Nova versão disponível: vX.Y.Z (atual: vA.B.C). Execute '<slug> update' para atualizar.
     ```

### 4.3 Contrato de Subcomandos CLI e Persistência de Canais
Todo binário distribuído **DEVE** responder aos subcomandos:
* `<slug> check-update [--channel <stable|nightly>] [--stable] [--nightly]`  
  Realiza consulta remota síncrona não-bloqueante na Banca, atualiza o arquivo `${XDG_STATE_HOME}/<slug>/update_check.json` e informa o status.
* `<slug> update [--channel <stable|nightly>] [--stable] [--nightly] [--pin <ver>] [--force] [--dry-run]`  
  Efetua a substituição atômica pelo binário mais recente da Banca respeitando o canal solicitado.
* `<slug> config show | set-channel <c> | pin [<ver>] | unpin | set-auto-check <bool> | set-auto-update <bool>`  
  Exibe ou altera de forma persistente as opções em `${XDG_CONFIG_HOME}/<slug>/config.env`.

#### **Garantia de Persistência do Canal Dev / Nightly:**
* A preferência declarada em `UPDATE_CHANNEL` (em `config.env`) define a assinatura duradoura de atualizações do usuário (`stable` ou `dev`/`nightly`).
* O comando pontual `<slug> update --stable` (ou `--channel stable`) instala a compilação estável requisitada naquele momento, **mas NUNCA altera o valor de `UPDATE_CHANNEL`** no arquivo `config.env`.
* A reconfiguração persistente do canal exige comando explícito (`<slug> config set-channel stable` ou flag `--save-channel`).
* Enquanto `UPDATE_CHANNEL="dev"` (ou `"nightly"`), o verificador continuará consultando e alertando sobre novas compilações do canal `dev` assim que publicadas na Banca, mesmo que o executável local em execução seja momentaneamente uma versão estável.

### 4.4 Topologia de Armazenamento e Distribuição Desacoplada
A esteira de publicação opera sob o modelo **Produtor–Curador Desacoplado**:
* **Produtor (Repositório do Software):** Compila o código na esteira, calcula os hashes de integridade (SHA-256), envia os artefatos diretamente para o storage de distribuição, atualiza a cópia de alias e despacha um evento de notificação via API (`repository_dispatch`).
* **Curador (Repositório Central da Banca):** Ouve os eventos via esteira (`repository_dispatch`), executa auditoria estrita de existência e integridade dos arquivos novos e do histórico ativo, gerencia a retenção no storage conforme o perfil operacional (Lilya), atualiza o catálogo (`catalog.json`) e realiza o deploy.
* **Topologia do Storage (Bucket Único Particionado):**
  Todos os artefatos são mantidos sob o prefixo `banca-artifacts/{app}/{channel}/`:
  - `{app}-{versao}-{target}.{ext}` e respectivo `.sha256`
  - Cópias físicas de alias sobrescritas: `{app}-latest-{target}.{ext}` (em `stable`) ou `{app}-dev-latest-{target}.{ext}` (em `dev`), juntamente com seus arquivos `.sha256`.

### 4.5 Nomenclatura de Canais e Aliases Canônicos
A publicação e o consumo de artefatos de distribuição respeitam os canais canônicos:
1. **Canal Estável (`stable`):**
   - Rótulo na interface: *Versão Recomendada*.
   - Sufixo / padrão de versão: SemVer 2.0 (`x.y.z`).
   - Alias canônico: `*-latest.*` (ex.: `{app}-latest-{target}.{ext}`).
   - Possui **exclusividade absoluta** sobre o alias primário `latest` e versões desprovidas de sufixo.
2. **Canal de Desenvolvimento (`dev`):**
   - Rótulo na interface: *Versão Experimental*.
   - Sufixo / padrão de versão: `x.y.z-dev.{n}+{YYYYMMDD}.{commit}`.
   - **Semântica Canônica de `{n}` (Commit Count):** O contador `{n}` representa estritamente o número de commits desde a última tag estável (`git rev-list vX.Y.Z..HEAD --count`), mensurando a distância relativa do incremento em relação à versão estável base. É expressamente proibido utilizar a contagem acumulada absoluta de commits do repositório (`git rev-list --count HEAD`), prevenindo contaminações e saltos artificiais de numeração no SemVer. Quando o `HEAD` coincidir com a tag estável, `{n}` é `0`. Se o repositório não possuir tags estáveis anteriores (estado inaugural/gênese), `{n}` recai sobre a contagem a partir da gênese sobre a base `0.0.0-dev.{n}`.
   - Alias canônico: `*-dev-latest.*` (ex.: `{app}-dev-latest-{target}.{ext}`).
3. **Consumo de Versões Canônicas Recomendado:**
   - Em manifestos de build (`Cargo.toml`, etc.), a fixação em versões canônicas exatas (`=`) é formalmente **recomendada** (*SHOULD* / *DEVERIA*) para garantir reprodutibilidade hermética em cascata.
4. **Tolerância Zero a Links Quebrados:**
   - Se qualquer artefato declarado (novo ou histórico ativo) falhar na verificação de rede ou apresentar divergência de checksum, a publicação do catálogo é abortada imediatamente.

---

## 5. Esteira de CI/CD Hermética (`.forgejo/workflows/ci.yml`)

Em conformidade com a Seção 10 da norma GOST, todo repositório Kollontai possui um único arquivo `.forgejo/workflows/ci.yml` executando em containers locais pré-provisionados (sem downloads externos em runtime), dividido nas filas canônicas:

```yaml
name: "Gosplan Hermetic Pipeline"

on:
  push:
    branches: [main, "v*", "release/v*", "epic/*"]
  pull_request:

jobs:
  test:
    name: "Quality Gate (Lint, Tests, Coverage >= 90%, Audit)"
    runs-on: local-hermetic
    steps:
      - name: "Checkout Code"
        uses: actions/checkout@v4
      - name: "Execute GOST Compliance & Test Suite"
        run: |
          cccp gost check
          cccp test --coverage-min 90

  release:
    name: "Build Static Artifacts & Publish to Banca"
    runs-on: local-hermetic
    needs: [test]
    if: github.ref == 'refs/heads/main'
    steps:
      - name: "Checkout Code"
        uses: actions/checkout@v4
      - name: "Compile & Dispatch to R2 / Banca"
        run: cccp ci publish-artifacts

  deploy:
    name: "Deploy Documentation Site & Containers"
    runs-on: local-hermetic
    needs: [test]
    if: github.ref == 'refs/heads/main'
    steps:
      - name: "Checkout Code"
        uses: actions/checkout@v4
      - name: "Deploy Static Showcase & Orphan Pages"
        run: cccp ci deploy-site
```

* **Isolamento da Quarentena (`inbox/*`):** Para branches `refs/heads/inbox/*`, apenas o job `test` é executado em modo desprivilegiado (sem acesso a secrets de deploy ou chaves da Banca).

### 5.1 Contrato de Notificação via API (`repository_dispatch`)
O repositório produtor do software notifica a esteira central da Banca via chamada HTTP:
`POST /api/v1/repos/{owner}/banca/dispatches` com `event_type: "release-published"`.

#### Schema do Payload (`client_payload`):
```json
{
  "event_type": "release-published",
  "client_payload": {
    "app": "slug",
    "channel": "stable",
    "version": "x.y.z",
    "commit": "hash",
    "repo_slug": "owner/slug",
    "run_id": "1842",
    "assets": [
      {
        "filename": "app-0.1.0-linux-x86_64.tar.gz",
        "alias": "app-latest-linux-x86_64.tar.gz",
        "type": "archive",
        "format": "tar.gz",
        "target": "linux-x86_64",
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      }
    ]
  }
}
```

### 5.2 Idempotência na Publicação e Pre-Delete de Aliases Mutáveis
Para forjas de pacotes que rejeitam sobrescrita direta de arquivos com o mesmo nome sob uma versão (como os Generic Packages do Forgejo, gerando erros `HTTP 409 Conflict`), a esteira de CI/CD **DEVE** garantir idempotência:
* Antes de executar o envio (`PUT`) para aliases mutáveis (`latest-dev`, `${BASE_VERSION}-dev`, `latest`), a esteira deve executar uma limpeza prévia com deleção idempotente (`curl -s -X DELETE ... || true`).
* Essa prática previne falhas espúrias de publicação e assegura que a forja atualize o ponteiro do alias sem interrupções.

---

## 6. Vitrine Web, Internacionalização e Gerador de Banners

### 6.1 Roteador Estático Cliente (`site/index.html`) para `site_type: doc`
A página raiz atua como roteador leve para os 10 subdiretórios de idioma (`en`, `pt`, `es`, `fr`, `de`, `it`, `ru`, `zh`, `ja`, `ar`), respeitando `localStorage.getItem('doc_lang')`, `navigator.language` e o fallback primário do `.cccprc`:

```html
<script>
  (function() {
    var supported = ['en', 'pt', 'es', 'fr', 'de', 'it', 'ru', 'zh', 'ja', 'ar'];
    var saved = localStorage.getItem('doc_lang');
    if (saved && supported.indexOf(saved) !== -1) {
      window.location.replace(saved + '/');
      return;
    }
    var nav = (navigator.language || 'pt').toLowerCase().substring(0, 2);
    window.location.replace((supported.indexOf(nav) !== -1 ? nav : 'pt') + '/');
  })();
</script>
```

### 6.2 Geração Determinística do Banner Minimalista
O banner `site/assets/og_preview.png` (`1200 × 630 px`, Safe Zone 4:3 de `720px–840px` × `500px`) é gerado deterministicamente a partir do `.gost/metadata.json`:
* Renderiza o fundo com `theme.bg_color` e detalhes de contraste com `theme.accent_color`.
* Posiciona o `icon` oficial e imprime exclusivamente a chamada curta **`{title} — {tagline}`** ($\le 65$ caracteres), sem textos secundários.

### 6.3 Template Estrutural do Rodapé Canônico em Duas Linhas
O perfil Kollontai estabelece o contrato de renderização do rodapé nas 10 línguas oficiais, parametrizando as variáveis de identidade (`{author_affirmation_i18n}`, `{author_name}`, `{author_url}`, `{country_i18n}`) que são preenchidas pelo perfil final:

```html
<footer class="gost-footer">
  <p>{author_affirmation_i18n}</p>
  <p>{title} v{version_full} • <a href="{author_url}" target="_blank" rel="noopener">{author_name}</a> • {country_i18n} • <a href="LICENSE">{license_spdx}</a></p>
</footer>
```

---

## 7. Catálogo Local de Descoberta (`~/src/catalog.json`)

Todo repositório inicializado ou sincronizado pelo ferramental de governança registra seus metadados extraídos do `.gost/metadata.json` no índice central da estação de trabalho (`~/src/catalog.json`):

```json
{
  "slug": "<slug>",
  "title": "<title>",
  "tagline": "<tagline>",
  "summary": "<summary>",
  "tags": ["<tag1>", "<tag2>", "<tag3>"],
  "license": "<spdx>",
  "site_type": "doc",
  "primary_forge": "<primary_forge_url>",
  "website": "<public_site_url>"
}
```

---

## 8. Padrões Táticos de Conteinerização Obrigatória de Serviços

Em cumprimento à diretriz estratégica do GOSPLAN, todo serviço de backend, microsserviço ou daemon distribuído no perfil Kollontai DEVE atender aos seguintes padrões de conteinerização:

### 8.1 Anatomia de Imagem (`Dockerfile` / `Containerfile`)
1. **Multi-Stage Hermético:** O estágio de build compila o software hermeticamente; o estágio final copia apenas o binário compilado e artefatos estritamente necessários para uma imagem base minimalista (`alpine` ou `scratch`).
2. **Execução Desprivilegiada Mandatória:** O container DEVE executar sob usuário não-root explicitamente (`user: "1000:1000"`). É proibida a execução contínua como `root`.
3. **Imutabilidade e Efemeridade:** O sistema de arquivos do container é considerado volátil, delegando persistência a volumes nomeados ou bind-mounts explicitados no compose.

### 8.2 Orquestração Declarativa Local (`compose.yaml`)
1. **Extensão Canônica:** A declaração de orquestração utiliza estritamente o nome `compose.yaml` (sendo expressamente banido o uso da extensão obsoleta `.yml`).
2. **Política de Reinício e Healthcheck:** Todo serviço em produção declara `restart: unless-stopped` e uma probe ativa de integridade (`healthcheck` com `interval`, `timeout` e `retries`).
3. **Isolamento de Rede:** Serviços internos operam em redes privadas OCI (`networks: default: internal: true` ou isoladas). Portas só devem ser expostas se o serviço for explicitamente um ponto de ingresso público.

---

## 9. Padrão Tático de Sanidade Pós-Compilação (*Smoke Tests*)

Todo artefato executável gerado no perfil Kollontai está sujeito a validação no escopo canônico `post-build`:

1. **Execução Dinâmica Rápida:** O binário gerado é executado com argumentos de inspeção inofensivos em tempo limite estrito ($\le 3$ segundos).
2. **Prevenção de Falhas Críticas:** O teste detecta e bloqueia falhas de segmentação (`SIGSEGV` / sinal 11), *panics* imediatos de runtime ou ausência de dependências dinâmicas compartilhadas (*shared libraries* faltantes no ambiente).
3. **Isolamento e Limpeza:** A execução não deve deixar processos zumbis ou arquivos temporários não tratados no host.


---

## 10. Decretos Regulamentadores de Engenharia (KOLLONTAI-001 a KOLLONTAI-018)

O catálogo tático do perfil Kollontai estabelece parâmetros objetivos de engenharia e conformidade técnica para todos os projetos subordinados:

| ID | Nome Canônico | Escopo | Descrição Sumária |
| :--- | :--- | :--- | :--- |
| **KOLLONTAI-001** | `README-MAX-LINES` | `project` | Teto rigoroso de 50 linhas para o arquivo `README.md`. |
| **KOLLONTAI-002** | `BANNER-DIMENSIONS-AND-RATIO` | `project` | Banner institucional exatamente em 1200x630 (proporção 1.91:1) com safe zone. |
| **KOLLONTAI-003** | `DDD-DOMAIN-PURITY` | `project` | Pureza de domínio: `src/domain/` com zero I/O, rede ou dependências de frameworks. |
| **KOLLONTAI-004** | `STATIC-ANALYSIS-ZERO-WARNINGS` | `ci` | Linters e análise estática executando com zero advertências (`-D warnings` ou `--strict`). |
| **KOLLONTAI-005** | `FORMAT-CHECK-DETERMINISTIC` | `ci` | Verificação determinística de formatação de código (`fmt --check`) na esteira. |
| **KOLLONTAI-006** | `SITE-SECURITY-HEADERS` | `project` | Cabeçalhos de segurança HTTP em `site/_headers` (anti-sniffing e anti-framing). |
| **KOLLONTAI-007** | `COMPOSE-NON-ROOT-USER` | `project` | Usuário desprivilegiado `user: "1000:1000"` obrigatório em manifestos compose. |
| **KOLLONTAI-008** | `COMPOSE-HEALTHCHECK-REQUIRED` | `project` | Sonda de integridade ativa (`healthcheck:`) obrigatória em manifestos compose. |
| **KOLLONTAI-009** | `DOCKERFILE-NON-ROOT-USER` | `project` | Usuário desprivilegiado `USER 1000:1000` obrigatório em Dockerfiles. |
| **KOLLONTAI-010** | `API-CONTRACT-OPENAPI` | `project` | Contratos de API em `docs/openapi.yaml` ou `docs/external/` em padrão OpenAPI 3.x. |
| **KOLLONTAI-011** | `FORBIDDEN-TERMS` | `docs` | Tabela curada de termos proibidos (sem etiqueta imperialista na norma). |
| **KOLLONTAI-012** | `SMOKE-TEST-SANITY-POST-BUILD` | `post-build` | Smoke test pós-compilação atestando ausência de SIGSEGV (139) e panics. |
| **KOLLONTAI-013** | `INTL-KEY-PARITY` | `project` | Paridade genérica de chaves entre dicionários das línguas listadas em `LANGUAGES`. |
| **KOLLONTAI-014** | `RTL-LOGICAL-PROPERTIES-AUDIT` | `project` | Auditoria de propriedades lógicas CSS e atributo `dir="rtl"` para idiomas RTL. |
| **KOLLONTAI-015** | `I18N-SITE-FALLBACK-ENGLISH` | `project` | Roteador cliente do site adota inglês (`en`) como fallback na ausência do idioma do visitante. |
| **KOLLONTAI-016** | `FORBIDDEN-PATH-DEPENDENCIES` | `project` | Proibição de dependências locais via caminho (path = "...") em manifestos de pacotes. |
| **KOLLONTAI-017** | `LICENSING-MATRIX-COMPLIANCE` | `project` | Conformidade da matriz de licenciamento (GPL-3.0 padrão em software e satélites modulares). |
| **KOLLONTAI-018** | `GIT-HOOKS-MAESTRO-PARITY` | `project` | Hooks essenciais do Git instalados e ativamente gerenciados pelo maestro `cccp`. |

---

<!-- ===================================================================== -->
<!-- CAMADA 2: https://lilya.2lp.in/2.2.0/RULES.md -->
<!-- ===================================================================== -->

---
standard: "lilya"
tier: "operational"
version: "2.2.0"
locale: "pt-br"
authority: "Standards Committee"
published_at: "2026-11-08"
inherits_from:
  standard: "kollontai"
  tier: "tactical"
  version: "2.2.0"
  base_url: "https://kollontai.2lp.in/2.2.0/"
---

# Perfil de Implementação GOST: Lilya

> **Sobre este Perfil:**  
> O perfil **Lilya** é a especialização autoral, pessoal e de infraestrutura do padrão **[GOST](https://gost.2lp.in)**.  
> Ele estende diretamente o perfil base **[Kollontai](https://kollontai.2lp.in)** (`extends: https://kollontai.2lp.in/profile.md`), herdando a autoridade do Maestro **`cccp`**, a matriz de toolchains homologadas e os protocolos de distribuição, aplicando precedência estrita (*deep override*) para a forja canônica em `gambi.2lp.in`, publicação na borda (`*.2lp.in`), catálogo de binários (`banca.2lp.in`), afirmação identitária transfeminista no rodapé e entrega contínua nos servidores `obizero`, `obione` e `obitwo`.

---

## 1. Amarração Operacional de Binários Concretos da Fábrica Tulipa

Em estrito alinhamento com a separação em camadas estabelecida pelo **GOST** (estratégico) e formalizada nos contratos abstratos de engenharia do **Kollontai** (tático), o perfil **Lilya** amarra operacionalmente os binários executáveis concretos da infraestrutura:

| Papel Normativo Abstrato | Contrato Tático (Kollontai) | Binário Concreto Homologado (Lilya) | Repositório / Endpoint Canônico |
| :--- | :--- | :--- | :--- |
| **Maestro Local** | Ciclo DD-TDD, Branches, Commits, Fast-Hygiene, Merge e Push | **`cccp`** | `https://git.2lp.in/lumen/cccp` (`cccp.2lp.in`) |
| **Autoridade de Forja e Abastecimento** | Tickets remotos, Triagem assistida (IA), Scaffolding e Catálogo Multi-repo | **`gossnab`** | `https://git.2lp.in/lumen/gossnab` (`gossnab.2lp.in/api/v1`) |
| **Motor de Avaliação Normativa** | Auditoria de regras `rules.toml`, Laudos JSON/Texto e Avaliação sintática | **`lunokhod`** (`lk`) | `https://git.2lp.in/lumen/lunokhod` (`lunokhod.2lp.in`) |
| **Agente de Entrega e Orquestração Contínua (CD)** | Despacho de artefatos, Purga de staging, Deploy de stacks e Cascata | **`kombinat`** | `https://git.2lp.in/lumen/kombinat` (`kombinat.2lp.in`) |
| **Controlador de Fronteira e Sincronia** | Quarentena de PRs, Sincronia de espelhos e Ingestão de webhooks | **`kontur`** (`kt`) | `https://git.2lp.in/lumen/kontur` (`kontur.2lp.in`) |

### 1.1 Tabela Operacional de Comandos Concretos
A rotina de desenvolvimento na Fábrica Tulipa opera através das seguintes chamadas homologadas:
* **Ciclo de Vida Local:** `cccp start <id>`, `cccp commit "<resumo>"`, `cccp mutate`, `cccp hygiene`, `cccp close "<resumo>"`, `cccp merge`, `cccp push`, `cccp release`.
* **Forjas, Tickets e Catálogo:** `gossnab ticket <create|list|comment|close|sync>`, `gossnab divan` (triagem assistida via Gemini ou Ollama), `gossnab new <slug>`, `gossnab catalog scan/list`, `gossnab batch push`.
* **Auditoria Normativa:** `lunokhod check --strict --lang pt-br [--scope <scope>] [--json]`, `lunokhod test`, `lunokhod eval "<expr>"`.
* **Entrega Contínua e Deploy:** `kombinat` (daemon de CD / container em `stacks/kombinat/compose.yaml`).
* **Telemetria e Fronteira:** `kt daemon`, `kt infra sync`, `kt reconcile`, `kt queue status/flush`, `kt inbox clean`.

---

## 2. Identidade, Forja Canônica (`gambi.2lp.in`) e Espelhos

### 1.1 Parâmetros Canônicos do Maestro (`.cccprc`)
No perfil Lilya, o Maestro `cccp` opera com os seguintes parâmetros canônicos de infraestrutura:

```ini
agent_var_lang = portuguese

# Forja Primária Canônica (Forgejo em gambi.2lp.in)
primary_forge_type = forgejo
primary_forge_domain = gambi.2lp.in
primary_owner = lumen
primary_repo = <slug>
primary_ssh_url = git@gambi.2lp.in:lumen/<slug>.git

# Espelhos Passivos e Ingestão de Fronteira
mirrors = codeberg,github
mirror_owner = lumenpink
```

### 1.2 Topologia de Repositórios e Push Mirrors Duplos
1. **Forja Primária Canônica:** Hospedada em `https://gambi.2lp.in/lumen/<slug>` (acesso SSH via porta 22 padrão em `git@gambi.2lp.in:lumen/<slug>.git`).
2. **Push Mirrors Automatizados (`sync_on_commit: true`):** A instância `gambi.2lp.in` replica imediatamente todo push recebido na branch `main` (e tags/branches persistentes) para dois espelhos externos de vitrine e preservação:
   - **Espelho FOSS:** Codeberg (`https://codeberg.org/lumenpink/<slug>`)
   - **Espelho Comunitário:** GitHub (`https://github.com/lumenpink/<slug>`)
3. **Remotes Locais e Ingestão de PRs:**
   - O desenvolvedor e os agentes de IA executam `cccp push` exclusivamente para o `origin` (`git@gambi.2lp.in:lumen/<slug>.git`).
   - Os remotes `codeberg` e `github` são mantidos pelo `cccp` com `pushurl = no_push` e refspec `+refs/pull/*/head:refs/remotes/<remote>/pr/*` para ingestão segura na quarentena `inbox/*`.

### 1.3 Allowlist Estrita de Rede na Esteira (`ci.yml`)
Na auditoria da regra `GOST-10-CI-HERMETIC-SINGLE-FILE`, a esteira `.forgejo/workflows/ci.yml` nos runners do ecossistema Lilya **SÓ PODE** referenciar recursos e espelhos internos sob os domínios:
* **`gambi.2lp.in`** (Actions espelhadas internamente ou relativas sem prefixo de servidor).
* **`crates.2lp.in`** (Registry interno e espelho de pacotes).
* **`banca.2lp.in`** (Despacho de artefatos compilados na fila `release`).
* Chamadas a outros domínios de rede pública quebram o `cccp check --strict` imediatamente.

---

## 3. Rodapé Canônico em Duas Linhas (Afirmação Identitária & Autonomia)

O perfil Lilya preenche o template estrutural de rodapé herdado do perfil Kollontai com a afirmação identitária transfeminista sul-americana nas 10 línguas oficiais da norma GOST.

### 2.1 Estrutura HTML Canônica
Todo aplicativo com interface gráfica, TUI/CLI com rodapé formatado, vitrine pública ou documentação estática (`./site`) **DEVE** renderizar as duas linhas rigorosamente nesta estrutura:

```html
<footer class="footer">
  <p>Orgulhosamente produzido por uma <a href="https://travesti.2lp.in/pt/" target="_blank" rel="noopener">travesti latina</a> 🏳️‍⚧️ :)</p>
  <p>{title} v{version_full} • por <a href="https://lumen.pink" target="_blank" rel="noopener">Lumen Pink</a> • Brasil 🇧🇷 • <a href="LICENSE">{license_spdx}</a></p>
</footer>
```

### 2.2 Regras Invioláveis de Renderização do Rodapé
1. **Intraduzibilidade Política do Termo `travesti`:** O substantivo *travesti* permanece rigorosamente sem tradução nos 10 idiomas oficiais, preservando sua singularidade histórica e geopolítica latino-americana. O adjetivo concorda gramaticalmente com o idioma ativo e aponta para `https://travesti.2lp.in/<lang>/`.
2. **Versão Short vs. Versão Long:**
   - Cabeçalhos, títulos de janela e badges no topo da página exibem estritamente a **Versão Short** (`v{MAJOR}.{MINOR}.{PATCH}`, truncada antes do `+`).
   - A Linha 2 do rodapé exibe obrigatoriamente a **Versão Long** (string integral do arquivo `VERSION`, incluindo metadados de build `+YYYYMMDD`, ex: `v0.1.0+20261026`).

### 2.3 Matriz Oficial de Tradução do Rodapé (10 Línguas)

| ISO | Linha 1 (Afirmação Identitária Transfeminista) | Linha 2 (Metadados de Autonomia e Licença) |
|:---:|---|---|
| **`pt`** | Orgulhosamente produzido por uma [travesti latina](https://travesti.2lp.in/pt/) 🏳️‍⚧️ :) | {title} v{ver} • por [Lumen Pink](https://lumen.pink) • Brasil 🇧🇷 • [{lic}](LICENSE) |
| **`en`** | Proudly produced by a [Latin American travesti](https://travesti.2lp.in/en/) 🏳️‍⚧️ :) | {title} v{ver} • by [Lumen Pink](https://lumen.pink) • Brazil 🇧🇷 • [{lic}](LICENSE) |
| **`es`** | Orgullosamente producido por una [travesti latinoamericana](https://travesti.2lp.in/es/) 🏳️‍⚧️ :) | {title} v{ver} • por [Lumen Pink](https://lumen.pink) • Brasil 🇧🇷 • [{lic}](LICENSE) |
| **`fr`** | Fièrement produit par une [travesti latino-américaine](https://travesti.2lp.in/fr/) 🏳️‍⚧️ :) | {title} v{ver} • par [Lumen Pink](https://lumen.pink) • Brésil 🇧🇷 • [{lic}](LICENSE) |
| **`de`** | Mit Stolz entwickelt von einer [lateinamerikanischen Travesti](https://travesti.2lp.in/de/) 🏳️‍⚧️ :) | {title} v{ver} • von [Lumen Pink](https://lumen.pink) • Brasilien 🇧🇷 • [{lic}](LICENSE) |
| **`it`** | Orgogliosamente prodotto da una [travesti latinoamericana](https://travesti.2lp.in/it/) 🏳️‍⚧️ :) | {title} v{ver} • da [Lumen Pink](https://lumen.pink) • Brasile 🇧🇷 • [{lic}](LICENSE) |
| **`ru`** | С гордостью создано [латиноамериканской travesti](https://travesti.2lp.in/ru/) 🏳️‍⚧️ :) | {title} v{ver} • от [Lumen Pink](https://lumen.pink) • Бразилия 🇧🇷 • [{lic}](LICENSE) |
| **`zh`** | 由一位[拉美 travesti](https://travesti.2lp.in/zh/) 自豪制作 🏳️‍⚧️ :) | {title} v{ver} • 来自 [Lumen Pink](https://lumen.pink) • 巴西 🇧🇷 • [{lic}](LICENSE) |
| **`ja`** | [ラテンアメリカの travesti](https://travesti.2lp.in/ja/) によって誇りを持って制作されました 🏳️‍⚧️ :) | {title} v{ver} • 制作: [Lumen Pink](https://lumen.pink) • ブラジル 🇧🇷 • [{lic}](LICENSE) |
| **`ar`** | صُنع بكل فخر بواسطة [travesti أمريكية لاتينية](https://travesti.2lp.in/ar/) 🏳️‍⚧️ :) | {title} v{ver} • بواسطة [Lumen Pink](https://lumen.pink) • البرازيل 🇧🇷 • [{lic}](LICENSE) |

---

## 4. Publicação Web na Borda (`*.2lp.in`) e Empacotamento Dioxus

### 3.1 Hospedagem Estática via Cloudflare Workers (`wrangler.jsonc`)
Toda vitrine pública (`./site`) é publicada sob o domínio `<slug>.2lp.in` através de `wrangler.jsonc` na raiz:

```jsonc
{
  "$schema": "node_modules/wrangler/config-schema.json",
  "name": "<slug>",
  "compatibility_date": "2026-01-01",
  "routes": [
    {
      "pattern": "<slug>.2lp.in",
      "custom_domain": true
    }
  ],
  "assets": {
    "directory": "./site"
  }
}
```

* **Redundância Estática:** A fila `deploy` da esteira publica simultaneamente o snapshot em branch órfã stateless `pages` para o Codeberg Pages (`https://lumenpink.codeberg.page/<slug>/`).

### 3.2 Identificador de Bundle Canônico (`Dioxus.toml`)
Para aplicações gráficas e mobile construídas com Dioxus 0.8, o namespace reverso e o publisher são fixados como:

```toml
[application]
name = "<slug>"
default_platform = "web"

[bundle]
identifier = "in.2lp.<slug>"
publisher = "Lumen Pink"
icon = ["site/assets/icon.png"]
```

---

## 5. Distribuição e Catálogo da Banca (`banca.2lp.in` e Cloudflare R2)

A autoridade central de distribuição de executáveis e pacotes definida na Seção 4 do perfil Kollontai opera sob o modelo **Produtor–Curador Desacoplado** servido via Cloudflare R2 e publicado na borda:

* **Topologia Física de Storage (Cloudflare R2):**
  Todos os artefatos são mantidos sob o prefixo:
  ```text
  banca-artifacts/{app}/{channel}/
  ```
  - Artefatos versionados e hashes: `{app}-{versao}-{target}.{ext}` e `.sha256`.
  - Cópias físicas de aliases sobrescritas: `{app}-latest-{target}.{ext}` (em `stable`) ou `{app}-dev-latest-{target}.{ext}` (em `dev`), juntamente com seus arquivos `.sha256`.
* **Cotas Operacionais e Política de Retenção Estrita:**
  - **Canal `stable` (Versão Recomendada):** Retém estritamente as **5 versões mais recentes** (`x.y.z`), com alias canônico `*-latest.*`.
  - **Canal `dev` (Versão Experimental):** Retém estritamente as **10 versões mais recentes** (`x.y.z-dev.{n}+{YYYYMMDD}.{commit}`), com alias canônico `*-dev-latest.*`.
  - Artefatos e hashes `.sha256` excedentes que ultrapassarem essas cotas são permanentemente purgados do bucket R2 pela esteira da Banca.
* **Transição para o Gossnab e Banimento de Manifestos Estáticos:**
  - **Banimento de `catalog.json` e `kombinat.toml` em Repositórios:** É expressamente banido manter arquivos estáticos de manifesto como `catalog.json` ou `kombinat.toml` dentro de repositórios de código.
  - **SSOT Centralizado Vivo (`gossnab`):** O serviço vivo **`gossnab`** (`https://gossnab.2lp.in/api/v1`) é a Fonte Única da Verdade para metadados de release, canais, inventário de artefatos e versionamento do ecossistema.
  - Durante o período de transição, a Banca gera e sincroniza views de compatibilidade retroativa em `https://banca.2lp.in/downloads/<slug>/index.json` consumindo a API viva do Gossnab.
* **Comando Universal de Instalação e Vitrine:**
  - Vitrine: `https://banca.2lp.in/downloads/#<slug>` (design brutalista/cyberpunk em Vanilla HTML/CSS/JS com cópia rápida de comando `curl` e tabela de hashes SHA-256).
  ```sh
  curl -sSf https://banca.2lp.in/downloads/<slug>/install.sh | sh
  ```
* **Protocolo de Auto-Update XDG:** Os binários CLI consultam diretamente a API viva do Gossnab (`https://gossnab.2lp.in/api/v1/apps/<slug>/update`) para o protocolo de auto-atualização (`<slug> check-update` / `<slug> update`), com fallback para `index.json`.
* **Tolerância Zero a Links Quebrados:** Se qualquer artefato declarado (novo ou histórico ativo nas cotas 5/10) falhar na verificação de rede ou divergência de checksum SHA-256, o deploy na Cloudflare é abortado imediatamente e um ticket automático é aberto na forja do projeto de origem (`POST /api/v1/repos/{repo_slug}/issues`).

---

## 6. Segregação Arquitetural: Gambi (CI) vs. Kombinat (CD)

Para eliminar atritos operacionais e garantir desacoplamento total entre integração contínua e entrega contínua, o perfil Lilya estabelece fronteiras estritas de responsabilidade:

### 6.1 O Papel da Gambi: Integração Contínua (CI)
* **Escopo:** Compilação, execução de testes unitários e de integração, verificação de linters/formatação e empacotamento.
* **Publicação Efêmera de Staging:** A esteira de CI em `gambi.2lp.in` publica artefatos compilados exclusivamente no Generic Package Registry da própria forja via `PUT /api/packages/{owner}/generic/{package_name}/{version}/{filename}`.
* **Isolamento de Produção:** A CI da Gambi **NÃO** efetua conexões SSH em servidores de produção nem realiza uploads diretos para storages externos (Cloudflare R2 / Banca / Pratilera).

### 6.2 O Papel do Kombinat: Entrega Contínua (CD) e Propagação em Cascata
* **Ingestão e Despacho de Binários:** O Kombinat consome os artefatos efêmeros de staging da Gambi, valida assinaturas e hashes SHA-256, e realiza a publicação externa definitiva na Banca (Cloudflare R2) e Pratilera.
* **Purga Imediata de Staging:** Imediatamente após a confirmação do despacho com sucesso, o Kombinat remove os artefatos efêmeros do registro da Gambi emitindo `DELETE /api/packages/{owner}/generic/{package_name}/{version}/{filename}`, mantendo a forja limpa e sem acúmulo de storage.
* **Propagação Automatizada em Cascata:** Ao detectar uma nova versão canônica de desenvolvimento publicada a montante (upstream), o Kombinat orquestra a propagação dos bumps exatos nos manifestos de compilação dos módulos a jusante (downstream). A fixação em versões canônicas exatas é recomendada, sem impor qualquer trava mandatória contra o uso prático de aliases.
* **Eliminação de Tags Mutáveis em Build Determinístico:**
  - É expressamente proibida a utilização de tags mutáveis (como `:latest`, `latest` flutuante ou referências dinâmicas não rastreadas) em scripts de compilação, dependências internas e imagens de base em pipelines de build determinístico.
  - Todo build reprodutível e determinístico deve ancorar-se em tags SemVer imutáveis (`x.y.z` ou `x.y.z-dev.{n}+{YYYYMMDD}.{commit}`). Aliases de conveniência (`*-latest.*`) são reservados unicamente para links de consumo e vitrine externa da Banca / R2, nunca para acoplamento interno de compilação.
* **Contrato Universal de FFI / C-ABI de Versão:**
  - Toda crate Rust do ecossistema deve exportar a constante `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` em sua raiz pública de biblioteca (`src/lib.rs`).
  - Para projetos que compilam bibliotecas estáticas vinculadas via C-ABI (`crate-type = ["staticlib"]`), é mandatório expor o símbolo FFI externamente visível `#[no_mangle] pub extern "C" fn <crate_name>_version() -> *const std::os::raw::c_char` para introspecção determinística de linkagem estática.
* **Inspeção Canônica de Ecossistema no CLI:**
  - Binários de ponta com interface de comando devem implementar `--version --all` para listar a árvore completa de versões dos submódulos e bibliotecas estáticas C-ABI embutidas.
  - Devem também implementar `--version --json` para emitir a árvore de dependências e diagnóstico do ecossistema em formato estruturado legível por máquina para o Kombinat.
* **Entrega Contínua de Servidores (CD):** O Kombinat assume a responsabilidade de acionar as atualizações dos serviços conteinerizados via SSH nos servidores (`obizero`, `obione`, `obitwo`), disparando `docker compose pull && docker compose up -d` e restart de containers.

### 6.3 Webhooks Multi-Forja e Loop de Reconciliação no Gossnab
* **Ingestão Multi-Forja:** Repositórios registrados no Gossnab possuem webhooks ativos apontando para o Gossnab (suportando Forgejo/Gambi, GitHub e SourceHut).
* **Loop de Reconciliação Anti-Drift:** Para mitigar falhas ou perdas de entrega de webhooks sem retentativa nativa, o Gossnab executa uma rotina periódica de conciliação a cada $N$ minutos, utilizando cabeçalhos condicionais HTTP (`ETag` e `If-Modified-Since`) para evitar requisições redundantes e rate limits.

### 6.4 Registries Autônomos Integrados: Container Registry OCI e Cargo Registry (`git.2lp.in`)
Para garantir independência de upstream, imunidade a cotas e rate limits externos (Docker Hub, GHCR, crates.io) e latência de loopback no nó central, a forja autônoma disponibiliza registries integrados:

#### 1. Container Registry OCI / Docker
* **Endpoint Canônico:** `git.2lp.in/<owner>/<image>:<tag>` (com alias `gambi.2lp.in/<owner>/<image>:<tag>`).
* **Autenticação:** Autenticação padrão via `docker login git.2lp.in` utilizando Personal Access Token (PAT) com escopo de leitura/escrita em pacotes.
* **Persistência de Armazenamento:** Os pacotes e camadas de contêiner são montados no storage persistente em `/media/hd1/forgejo/data/packages`, preservando o SSD do nó central contra desgaste e saturação de espaço em disco.
* **Política de Tags:** Tags SemVer estritas para builds estáveis (`x.y.z`) e builds de desenvolvimento (`x.y.z-dev.{n}+{YYYYMMDD}.{commit}`).

#### 2. Cargo Registry (Rust)
* **Registro de Crates do Ecossistema:** Registro autônomo hospedado em `git.2lp.in` para distribuição e consumo de bibliotecas e crates compartilhadas entre os projetos do ecossistema.
* **Configuração Canônica em `.cargo/config.toml`:**
  ```toml
  [registries.gambi]
  index = "sparse+https://git.2lp.in/api/packages/lumen/cargo/"

  [registry]
  default = "gambi"
  ```
* **Publicação:** `cargo publish --registry gambi` autenticado via token canônico da forja.

---

## 7. Entrega Contínua de Servidores (CD em `obizero`, `obione` e `obitwo`)

Para serviços conteinerizados autohospedados, o perfil Lilya opera com **Continuous Deployment (CD) Determinístico** orquestrado pelo **Kombinat CD** (eliminando etapas de SSH e deploy direto da esteira de CI), com banimento total do Watchtower.

### 7.1 Padrão de Orquestração e Healthcheck Obrigatório
Todos os serviços residem em `~/stacks/<slug>/compose.yaml` (estritamente com extensão `.yaml`, sendo banida a extensão `.yml`) e declaram probe ativa de saúde:

```yaml
services:
  app:
    image: lumenpink/<slug>:${APP_VERSION:-latest}
    user: "1000:1000"
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "curl -fsS http://127.0.0.1:<port>/healthz || exit 1"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 10s
```

### 7.2 Topologia de Deploy por Nó de Infraestrutura

#### 1. Nó Central de Execução (`obizero` — Deploy Local via Socket)
* O `forgejo-runner` executa no host `obizero` com o socket Docker montado.
* A imagem recém-compilada na fila `build` já reside no daemon local (`lumenpink/<slug>:<version>`).
* O deploy ocorre sem tráfego externo redundante:
  ```sh
  docker compose -f /home/lumen/stacks/<slug>/compose.yaml up -d --no-build <slug>
  ```
* **Verificação Síncrona de Saúde e Rollback Automático (15s):**
  A esteira monitora o status do container por até 15 segundos. Caso o estado não atinja `healthy` (ou `running` estável), a esteira captura `docker logs --tail 50 <slug>`, reverte imediatamente para a tag estável anterior, executa `docker image prune -f` e falha o pipeline com código não-zero.

#### 2. Nós de Borda Remotos (`obione` e `obitwo` — Deploy via Mesh VPN Tailscale)
* Para serviços distribuídos nos nós de borda (acessíveis via rede privada WireGuard/Tailscale ou expostos via Cloudflare Tunnel `cloudflared` em `app.<slug>.2lp.in`):
* A esteira publica a imagem e aciona a atualização remotamente via SSH sobre a malha Tailscale:
  ```sh
  ssh -o StrictHostKeyChecking=accept-new lumen@<no-tailscale> \
    "docker compose -f ~/stacks/<slug>/compose.yaml pull && \
     docker compose -f ~/stacks/<slug>/compose.yaml up -d && \
     docker image prune -f"
  ```

### 7.3 Mandato de Conteinerização Universal em Stacks e Topologia de Rede
1. **Banimento de Daemons no Host:** É estritamente proibido rodar serviços, APIs ou daemons de backend diretamente instalados no sistema operacional hospedeiro dos servidores (`obizero`, `obione`, `obitwo`). Todo serviço roda isolado via `~/stacks/<slug>/compose.yaml`.
2. **Topologia de Segurança e Ingress:**
   - **Ponto Único de Ingress Público:** Apenas o Gossnab possui porta pública exposta (via Cloudflare Tunnel / Gateway reverso) para recepção de webhooks das forjas externas e atendimento à Banca.
   - **Isolamento do Kombinat:** O Kombinat opera estritamente confinado na rede privada interna, sem portas abertas para a internet pública, interagindo como cliente ativo unidirecional para tarefas de orquestração, build, assinatura, publicação externa e deploy via SSH.

---

## 8. Sobreposição de Regras de Conformidade (`lilya_rules.json`)

Quando o `cccp sync` compila as regras para um repositório sob o perfil Lilya, as seguintes parametrizações específicas sobrepõem (*deep merge*) o `rules.json` base da GOST:

```json
{
  "extends": "GOST-2.0.0",
  "profile": "lilya",
  "rules_override": [
    {
      "id": "GOST-09-SITE-SECURITY-HEADERS",
      "require_footer_links": [
        "https://travesti.2lp.in/",
        "https://lumen.pink"
      ]
    },
    {
      "id": "GOST-10-CI-HERMETIC-SINGLE-FILE",
      "allowed_uses_domains": ["gambi.2lp.in"],
      "allowed_run_domains": ["gambi.2lp.in", "crates.2lp.in", "banca.2lp.in"]
    },
    {
      "id": "GOST-25-REMOTES-AND-INBOX-QUARANTINE",
      "required_origin_pattern": "^git@gambi\\.2lp\\.in:lumen/[a-z0-9-]+\\.git$",
      "required_mirrors": {
        "codeberg": "https://codeberg.org/lumenpink/<slug>.git",
        "github": "https://github.com/lumenpink/<slug>.git"
      }
    }
  ]
}
```

---

## 9. Conformidade Operacional de Interfaces CLI (*Post-Build*)

Todo utilitário CLI ou ferramenta de linha de comando voltada a desenvolvedores e usuárias finais, quando submetido à validação no escopo canônico `post-build`, DEVE:
1. **Flag de Ajuda Informativa (`--help`):** Responder com código 0 e texto estruturado de instruções mínimas de operação.
2. **Fidelidade Canônica de Versão (`--version`):** Responder com código 0 e texto contendo a versão exata do manifesto `VERSION` ou `Cargo.toml`.
3. **Ciclo de Vida e Atualização:** Quando aplicável ao canal de distribuição, manter subcomando canônico de verificação ou atualização (`update` / `self-update`) integrado aos canais da Banca.


---

## 10. Portarias Operacionais da Fábrica Tulipa (LILYA-001 a LILYA-009)

O perfil Lilya consolida as regras operacionais idiossincráticas da fábrica de software Tulipa, subordinadas aos decretos da Kollontai e às diretrizes do GOSPLAN:

| ID | Nome Canônico | Escopo | Descrição Sumária |
| :--- | :--- | :--- | :--- |
| **LILYA-001** | `VERSION-NO-V-PREFIX` | `project` | Proíbe estritamente o uso do prefixo 'v' no arquivo canônico `VERSION`. |
| **LILYA-002** | `TRANSFEMINIST-IDENTITY-REQUIRED` | `project` | Exige a afirmação de identidade transfeminista (`travesti`, `lumen.pink`) na documentação ou site. |
| **LILYA-003** | `PRIMARY-FORGE-GAMBI-CANONICAL` | `project` | Declaração mandatória da forja canônica Tulipa `gambi.2lp.in` em `.cccprc`. |
| **LILYA-004** | `CCCPRC-FILE-REQUIRED` | `project` | Presença e configuração obrigatória do arquivo `.cccprc` na raiz do repositório. |
| **LILYA-005** | `STATIC-MANIFESTS-PROHIBITED` | `project` | Proibição estrita de manifestos estáticos `catalog.json` e `kombinat.toml`. |
| **LILYA-006** | `STRICT-COMPOSE-YAML-EXTENSION` | `project` | Proibição de `.yml` e `docker-compose.yml`, exigindo estritamente `compose.yaml`. |
| **LILYA-007** | `METADATA-MANIFEST-SCHEMA` | `project` | Validação de schema do manifesto canônico de metadados Gossnab `.gost/metadata.json`. |
| **LILYA-008** | `CLI-HELP-INFORMATIVE-POST-BUILD` | `post-build` | Validação pós-compilação de texto informativo e estruturado via flag `--help`. |
| **LILYA-009** | `CLI-VERSION-METADATA-PARITY-POST-BUILD` | `post-build` | Validação pós-compilação de paridade exata da saída de `--version` com o arquivo `VERSION`. |

---



<!-- ===================================================================== -->
<!-- ESPECIFICAÇÃO TÁTICA DA STACK: RUST -->
<!-- ===================================================================== -->

# GOST Language Specification: Rust
**Identificador da Stack:** `rust`  
**Padrão Base:** GOST `2.0.0`

---

## 1. Visão Geral e Filosofia
Esta especificação define os requisitos de engenharia, ferramentas e padrões para desenvolvimento em **Rust** (edição 2024 canônica, compilador `rustc >= 1.100.0`) no ecossistema GOST.

---

## 2. Ferramental Mandatório de Testes & Qualidade

| Requisito GOST | Ferramenta Padrão | Comando de Validação no CI |
|---|---|---|
| **Test Runner** | **Cargo Test** | `cargo test --all-targets --all-features` |
| **Cobertura (≥ 90%)** | **cargo-llvm-cov** / **tarpaulin** | `cargo llvm-cov --fail-under-lines 90` |
| **Testes de Mutação** | **cargo-mutants** | `cargo mutants --in-diff --no-shuffle` |
| **Linter (Zero Warnings)** | **Clippy** | `cargo clippy --all-targets --all-features -- -D warnings` |
| **Formatação Determinística** | **rustfmt** | `cargo fmt -- --check` |
| **Auditoria de Segurança** | **cargo-audit** | `cargo audit` |

---

## 3. Gestão de Dependências (Twelve-Factor)
1. **Lockfile Obrigatório:** O arquivo `Cargo.lock` **DEVE** estar commitado na raiz.
2. **Dependências Mínimas:** Evitar crates pesadas quando a biblioteca padrão for suficiente. Feature flags ativadas apenas sob necessidade estrita.
3. **Compilação Hermética:** Builds de produção usam `--locked` para garantir que nenhuma dependência transitória flutue:
   ```bash
   cargo build --release --locked
   ```

---

## 4. Arquitetura de Domínio & SOLID Pragmático (Regra GOST-27)
1. **Pureza de Domínio (Zero I/O no Core):** O diretório `src/domain/` é estritamente puro. É terminantemente proibido importar módulos de sistema ou I/O dentro do Core: `std::fs`, `std::net`, `std::process`, `std::env`, `tokio::fs`, `tokio::net`, `reqwest`, `rusqlite`, `sqlx` ou `clap`.
2. **Estrutura de Pastas Canônica:**
   ```text
   src/
   ├── domain/             # Modelos e regras de negócio puras (sem I/O)
   │   ├── mod.rs
   │   ├── entity.rs       # Invariantes e structs de dados
   │   ├── status.rs       # Enums nativos para máquinas de estado
   │   └── ports.rs        # Traits de I/O (Output Ports)
   ├── application/        # Use cases e orquestração
   │   ├── mod.rs
   │   └── use_cases.rs
   ├── adapters/           # Adaptadores de I/O
   │   ├── mod.rs
   │   ├── in_memory.rs    # Fake em memória com HashMap (testes herméticos)
   │   ├── sqlite.rs       # Persistência concreta
   │   └── cli.rs          # Parser de linha de comando (clap)
   └── main.rs             # Composition Root (bootstrap e injeção)
   ```
3. **In-Memory Fakes:** Testes unitários instanciam structs que implementam os traits de domínio armazenando dados em `HashMap` ou `RwLock<Vec<T>>` na memória, rodando em microssegundos sem dependência de SQLite ou sockets. Fakes e adaptadores reais compartilham os mesmos testes de contrato (LSP).
4. **Parse, Don't Validate:** Validação de entradas externas na borda (parsers `clap` ou DTOs `serde`). O Core opera com primitivos confiáveis ou structs transparentes.
5. **Estados com Enums Nativos:**
   ```rust
   pub enum TaskState {
       Pending,
       Running { started_at: u64 },
       Finished { exit_code: i32 },
       Failed { error: String },
   }
   ```

---

## 5. Padrões de Sistema e Containers OCI
1. **Conformidade XDG:**
   Resolução de diretórios respeita variáveis de ambiente com fallback para `$HOME`:
   ```rust
   pub fn config_dir(app: &str) -> std::path::PathBuf {
       std::env::var("XDG_CONFIG_HOME")
           .map(std::path::PathBuf::from)
           .unwrap_or_else(|_| {
               let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
               std::path::PathBuf::from(home).join(".config")
           })
           .join(app)
   }
   ```
2. **Dockerfile Canônico (Multi-stage + Scratch Estático musl com CA-Certs):**
   ```dockerfile
   FROM rust:1.99-alpine AS builder
   RUN apk add --no-cache musl-dev ca-certificates
   WORKDIR /build
   COPY Cargo.toml Cargo.lock ./
   COPY src ./src
   RUN cargo build --release --target x86_64-unknown-linux-musl --locked

   FROM scratch
   USER 1000:1000
   COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
   COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/app
   ENTRYPOINT ["/usr/local/bin/app"]
   ```
3. **Streams:** Saída pura de dados via `println!()`, logs e mensagens diagnósticas via `eprintln!()` ou crate `tracing`/`env_logger` configurada para `stderr`.

---

## 6. Otimização de Binário de Release & Linking Estático

### 1. Perfil de Release Canônico (`[profile.release]`)
Para assegurar a máxima redução de tamanho binário, eliminação de símbolos mortos e prevenção de overhead de desbobinamento de pilha (*stack unwinding*), projetos Rust no ecossistema GOST devem declarar o perfil canônico em seus manifests `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

### 2. Linking Estático musl (`crt-static`)
Para garantir independência total do carregador dinâmico (`ld-linux`) e máxima portabilidade em ambientes Linux glibc e musl, a compilação contra alvos `x86_64-unknown-linux-musl` e `aarch64-unknown-linux-musl` deve habilitar o flag `crt-static`:

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]
```
