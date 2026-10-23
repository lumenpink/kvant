# ☭ ГОСТ / GOSPLAN — Especificação Canônica Consolidada

> **Aviso de Integridade Gosplan:**
> Este documento foi montado automaticamente pelo CCCP (`cccp profile assemble`).
> A precedência é estrita de baixo para cima: cada camada subsequente sobrepõe as definições anteriores.

### 🌳 Árvore de Herança e Precedência Resolvida:
1. Camada 1: `https://gost.2lp.in/SPEC.md`
2. Camada 2: `https://kollontai.2lp.in/profile.md`
3. Camada 3: `https://gostlumen.2lp.in/profile.md`

---

<!-- ===================================================================== -->
<!-- CAMADA 1: https://gost.2lp.in/SPEC.md -->
<!-- ===================================================================== -->

# GOST — Padrão Técnico para Repositórios Abertos
**Versão da Norma:** `1.0.0-draft`  
**Status:** Oficial / Proposta Normativa  
**Referência:** ГОСТ (Государственный Стандарт / General Open Source Standard)

---

## 1. Escopo e Propósito

Esta especificação define os requisitos estruturais, metodológicos, de governança e de documentação para criação, manutenção e ciclo de vida de repositórios de software aberto.

O objetivo do GOST é eliminar atritos no surgimento de novos projetos, garantir rastreabilidade histórica completa, assegurar a máxima liberdade do código e impor rigor de engenharia orientado à documentação e testes.

As palavras-chave **DEVE** (*MUST*), **NÃO DEVE** (*MUST NOT*), **DEVERIA** (*SHOULD*), **NÃO DEVERIA** (*SHOULD NOT*) e **PODE** (*MAY*) neste documento devem ser interpretadas conforme descrito na [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Filosofia e Licenciamento

Todo projeto em conformidade com o GOST **DEVE** declarar sua licença de forma explícita na raiz do repositório através de um arquivo `LICENSE`. A especificação técnica detalhada, perfis e matriz normativa encontram-se em [specs/legal/licenses.md](./specs/legal/licenses.md).

### 2.1 Princípio da Menor Permissividade como Padrão Raiz
* **Regra da Raiz:** O arquivo `LICENSE` na raiz **DEVE** ser a licença padrão e geral de todo o repositório.
* Em repositórios com múltiplos tipos de artefatos (como monorepos ou aplicações híbridas), o arquivo `LICENSE` **DEVE** conter a licença menos permissiva (mais restritiva) adotada (ex: `GPL-3.0`).
* Quaisquer licenças mais permissivas (ex: `Apache-2.0` para sub-bibliotecas ou `CC-BY-SA-4.0` para mídias) funcionam como exceções modulares declaradas.

### 2.2 Tipologia por Perfil de Repositório
1. **Aplicações Completas / Monorepos Híbridos (ex: Mayak, Navalha):** Adotam `GPL-3.0` (ou `AGPL-3.0` para serviços web) na raiz (`LICENSE`).
2. **Microapps & Comrades (ex: comrades/cli):** Adotam `Apache-2.0` na raiz (`LICENSE`) diretamente, sem necessidade de arquivos satélites supérfluos.
3. **Bibliotecas, Protocolos e SDKs:** Adotam `Apache-2.0` na raiz (`LICENSE`) para máxima interoperabilidade.
4. **Especificações e Normas (ex: GOST):** Adotam `CC-BY-SA-4.0` para os textos normativos na raiz (`LICENSE`), com esquemas e templates em `CC0-1.0` e marcas protegidas via satélite.

### 2.3 Arquivos Satélites Padronizados
Quando houver artefatos que exijam tratamentos jurídicos específicos, os seguintes arquivos canônicos **DEVEM** ser utilizados:
* `LICENSE-MEDIA`: Creative Commons Atribuição-CompartilhaIgual 4.0 Internacional (`CC-BY-SA-4.0`) para mídias, áudios, vídeos, fontes e ilustrações.
* `LICENSE-LOGO`: Creative Commons Atribuição-SemDerivações 4.0 Internacional (`CC-BY-ND-4.0`) para marcas, insígnias, ícones oficiais e logotipos.
* `LICENSE-APACHE`: Apache License 2.0 (`Apache-2.0`) para módulos, sub-crates ou bibliotecas desacopladas contidas em monorepos de aplicações.

### 2.4 Mapeamento Obrigatório no `README.md`
Repositórios com múltiplos tipos de artefatos ou licenças satélites **DEVEM** incluir em seu `README.md` uma seção ou tabela de mapeamento discriminando expressamente os caminhos e arquivos cobertos por cada licença.

### 2.5 Princípio Anti-Boilerplate
Cada repositório implementa exclusivamente as licenças condizentes com seu perfil real. É vedado copiar blocos genéricos de licenciamento e arquivos satélites vazios para projetos simples (como comrades ou microutilitários).

---

## 3. Anatomia Mínima da Raiz

Todo repositório GOST **DEVE** conter em sua raiz a seguinte estrutura mínima independente da stack ou linguagem utilizada:

```
.
├── LICENSE                     # Termo legal de licenciamento padrão (Seção 2)
├── LICENSE-MEDIA               # (Opcional) Licença CC-BY-SA para mídias e assets
├── LICENSE-LOGO                # (Opcional) Licença CC-BY-ND para marcas e logos
├── LICENSE-APACHE              # (Opcional) Licença Apache-2.0 para bibliotecas internas
├── VERSION                     # Versão semântica em texto plano (Seção 4)
├── CHANGELOG.md                # Registro cronológico de mudanças (Seção 4)
├── README.md                   # Apresentação executiva e guia rápido
├── .gitignore                  # Higiene do repositório e arquivos ignorados
├── .editorconfig               # Configuração canônica de formatação para editores
├── site/                       # Portal estático de documentação pública (Seção 8)
│   ├── index.html              # Ponto de entrada da documentação
│   └── _headers                # Cabeçalhos HTTP de segurança
└── .forgejo/ / .github/        # Automação de CI/CD (Seção 7)
    └── workflows/ci.yml
```

Arquivos de configuração e metadados específicos de ferramentas **NÃO DEVEM** poluir a raiz sem necessidade e devem ser isolados em pastas de configuração ou perfis.

---

## 4. Versionamento e Histórico

### 4.1 Arquivo `VERSION`
* O repositório **DEVE** manter um arquivo `VERSION` na raiz contendo exclusivamente a versão atual do projeto em formato de texto puro UTF-8, finalizado por uma quebra de linha.
* O formato de versionamento **DEVE** aderir estritamente ao [Semantic Versioning 2.0.0](https://semver.org/) (`MAJOR.MINOR.PATCH[-PRERELEASE]`).
* Durante o estágio de desenvolvimento inicial, a versão **DEVERIA** seguir o padrão `0.1.0-dev.N`.

### 4.2 Arquivo `CHANGELOG.md`
* O repositório **DEVE** manter um histórico de modificações legível por humanos e baseado no padrão [Keep a Changelog](https://keepachangelog.com/).
* As seções de cada versão **DEVEM** categorizar alterações em: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`.

---

## 5. Ciclo de Desenvolvimento GOST (DD-TDD)

O fluxo de trabalho em repositórios GOST **DEVE** seguir o ciclo rigoroso de **Documentation-Driven & Test-Driven Development (DD-TDD)** em 9 fases sequenciais e indivisíveis, governado pela regra de rastreabilidade estrita de tickets e isolamento em branches.

### 5.1 Cláusulas Pétreas do Ciclo

1. **Inviolabilidade das 9 Fases:** Todas as 9 etapas **DEVEM** ser seguidas em ordem sequencial estrita para toda e qualquer alteração no repositório (funcionalidade, bugfix, refatoração ou manutenção). É expressamente proibido pular etapas, invertê-las ou executá-las em lote.
2. **Zero Código sem Ticket e Documentação Prévia:** **NENHUMA ALTERAÇÃO EM CÓDIGO DE PRODUÇÃO É PERMITIDA SEM TICKET ABERTO E ESPECIFICAÇÃO PRELIMINAR DE DOCUMENTAÇÃO/CONTRATO**. Nenhuma linha funcional nasce sem rastreabilidade documental prévia.
3. **Isolamento Estrito da Fase de Documentação:** Na etapa de documentação/especificação (Fase 3), é **TERMINANTEMENTE PROIBIDO alterar qualquer linha de código de produção**. Apenas contratos de interface, tipos, assinaturas, diagramas e especificações funcionais são permitidos.
4. **Rastreabilidade Estrita de Commits e Tag Canônica de Fechamento:**
   - Todo trabalho **DEVE** nascer de uma issue formal registrada no issue tracker antes de qualquer modificação.
   - Todo commit intermediário **DEVE** incluir a referência ao ticket no formato `(ref #<id>)`.
   - O commit final de encerramento da Fase 9 **DEVE** conter rigorosamente a tag canônica `(closes #<id>)` (em minúsculas, envolta em parênteses com cerquilha e ID numérico) para liquidar formalmente o ticket. Variações sem parênteses ou sinônimos informais são rejeitadas.
   - **Trava de Merge Mandatória no CCCP:** O maestro `cccp` ou mecanismo de integração **NÃO DEVE** permitir o merge da branch para a linha principal (`main`/`develop`) se a branch não contiver o commit de fechamento com a tag canônica `(closes #<id>)` associada ao ID do ticket da branch.
   - Hooks de commit (`commit-msg`) e o pipeline de CI/CD **DEVEM** rejeitar imediatamente qualquer commit sem referência a ticket.
5. **Proibição Absoluta de Commit na Linha Base:** É estritamente **PROIBIDO** commitar diretamente no branch principal (`main` ou `develop`). Todo o trabalho reside exclusivamente no branch dedicado do ticket até sua aprovação e merge.
6. **Soberania do CCCP na Gestão de Tickets:** Toda e qualquer operação de ciclo de vida de tickets (criação, consulta, triagem com o Divan e encerramento) **DEVE** ser realizada exclusivamente através do binário soberano `cccp` (`cccp ticket`, `cccp divan`). É **TERMINANTEMENTE PROIBIDO** criar ou fechar tickets diretamente através de ferramentas ad-hoc de terceiros (como `tea`, `gh`, `glab`) ou por meio de manipulação manual avulsa na interface web do forge.

### 5.2 Estratégia de Branches por Ticket e Governança Local-First
* Todo ticket **DEVE** ser desenvolvido em um branch dedicado derivado de `develop` (ou `main`), com nomenclatura padronizada contendo o ID do ticket:
  - `feat/<id>-<slug-curto>` (ex: `feat/42-rate-limiting`)
  - `fix/<id>-<slug-curto>` (ex: `fix/55-jwt-timestamp-overflow`)
  - `docs/<id>-<slug-curto>` (ex: `docs/12-api-spec-update`)
  - `refactor/<id>-<slug-curto>` (ex: `refactor/18-extract-auth-module`)
  - `test/<id>-<slug-curto>`
  - `chore/<id>-<slug-curto>`
* **Governança Local-First de Branches:** As branches dedicadas de tickets residem e são desenvolvidas estritamente no ambiente **local**. É expressamente **PROIBIDO** enviar (*push*) branches de trabalho (`feat/*`, `fix/*`, etc.) para a forja remota.
* **Exceção Taxativa para Desenvolvimento Multidev:** O envio de uma branch de trabalho para a forja remota é admitido **única e exclusivamente sob solicitação expressa e taxativa**, justificada pela necessidade imperativa de colaboração simultânea de múltiplos desenvolvedores na mesma frente de trabalho. Fora desse pedido inequívoco, qualquer push de branch de trabalho é considerado anti-padrão.
* Todo o ciclo das 9 fases e a integração via `cccp merge <branch>` ocorrem localmente. Para a forja remota é enviada **exclusivamente a branch principal (`main`) já integrada e validada** (além das tags semânticas e do branch efêmero de deploy `pages`).

### 5.3 As 9 Fases Canônicas do Ciclo DD-TDD e Matriz de Commits

```text
[1. Ticket] 
    └──> [2. New Branch] 
            └──> [3. Doc / Spec Contract] 
                    └──> [4. Red: Failing Test] 
                            └──> [5. Green: Implement] 
                                    └──> [6. Refactor to Excellence] 
                                            └──> [7. Mutation Test & Fortify] 
                                                    └──> [8. Quality Gate & Local Audit] 
                                                            └──> [9. Finish Docs & Close]
```

#### Matriz Canônica de Commits por Fase

| Fase | Nome da Fase | Geração de Commits | Formato Canônico e Convenção |
|:---:|---|:---:|---|
| **1** | **Ticket / Issue** | ❌ **PROIBIDO** | *Zero commits no repositório (operação no issue tracker).* |
| **2** | **New Branch** | ❌ **NÃO** | *Zero commits (apenas checkout local da branch).* |
| **3** | **Doc / Spec Contract** | ✅ **OBRIGATÓRIO** | `docs: specify <comportamento> (ref #<id>)` *(zero código de produção).* |
| **4** | **Red: Failing Test** | ✅ **OBRIGATÓRIO** | `test: add failing test for <requisito> (ref #<id>)` *(teste falho comprovado).* |
| **5** | **Green: Implement** | ✅ **OBRIGATÓRIO** | `feat: implement <funcionalidade> (ref #<id>)` ou `fix: resolve <bug> (ref #<id>)`. |
| **6** | **Refactor to Excellence** | 🟡 **CONDICIONAL / OBRIGATÓRIO** | `refactor: optimize and polish <escopo> (ref #<id>)` *(se houver refatoração).* |
| **7** | **Mutation Test & Fortify** | 🟡 **CONDICIONAL / ATÔMICO DUPLO** | `test: eliminate surviving mutants in <escopo> (ref #<id>)` *(testes primeiro)*;<br>`refactor: eliminate dead code and redundant logic in <escopo> (ref #<id>)` *(código depois)*. |
| **8** | **Quality Gate & Local Audit** | ⚠️ **CONDICIONAL / RARO** | `fix: lint and formatting hygiene (ref #<id>)` *(apenas se ajustes forem necessários).* |
| **9** | **Finish Docs & Close** | ✅ **OBRIGATÓRIO** | `docs: finalize docs and changelog (closes #<id>)` *(ou commit final do ticket com closes).* |

1. **Fase 1: Ticket / Issue (Planejamento & Escopo)**
   - Abertura formal da issue no tracker com: Contexto, Comportamento Esperado e Critérios de Aceite detalhados.
   - Proibido qualquer commit ou alteração no repositório nesta fase.
2. **Fase 2: New Branch (Isolamento Estrito)**
   - Criação da branch dedicada a partir da base (`develop` ou `main`) no formato `<tipo>/<id>-<slug>`.
   - Nenhuma linha é alterada na branch base.
3. **Fase 3: Doc / Spec Contract (Design e Contrato Preliminar)**
   - Formalização prévia do contrato, interfaces públicas, assinaturas, diagramas e comportamento esperado.
   - **Regra Inviolável:** Zero alterações em código de produção nesta etapa. Apenas documentação e contratos.
   - Commit: `docs: specify <comportamento> (ref #<id>)`
4. **Fase 4: Red - Failing Test (Comprovação Automatizada)**
   - Criação de testes automatizados unitários ou funcionais exercitando o contrato definido na Fase 3.
   - O teste **DEVE falhar**, comprovando a ausência da funcionalidade ou reproduzindo fielmente o bug.
   - Commit: `test: add failing test for <requisito> (ref #<id>)`
5. **Fase 5: Green - Implement / Fix (Implementação Funcional)**
   - Primeira escrita em código de produção: implementação estritamente necessária e suficiente para fazer a suíte passar.
   - Commit: `feat: implement <funcionalidade> (ref #<id>)` ou `fix: resolve <bug> (ref #<id>)`
6. **Fase 6: Refactor to Excellence (Polimento e Qualidade)**
   - Refatoração do código de produção e testes: eliminação de duplicações, nomes idiomáticos, separação de responsabilidades e Zero Warnings. Testes continuam 100% verdes.
   - Commit: `refactor: optimize and polish <escopo> (ref #<id>)`
7. **Fase 7: Mutation Test & Fortify (Blindagem contra Mutantes e Ordem Estrita de Commits)**
   - Execução da ferramenta de testes de mutação. Cada mutante sobrevivente exige investigação minuciosa:
     - **Caso A (Asserções Fracas / Casos de Borda Faltantes - mais comum):** Altera-se **exclusivamente os testes**, adicionando asserções rigorosas e valores limite para matar o mutante.
       - *Commit inicial de testes:* `test: eliminate surviving mutants in <escopo> (ref #<id>)`
     - **Caso B (Código Morto / Lógica Redundante Revelada):** Se o mutante revelar código inalcançável ou overengineering, simplifica-se o **código de produção**.
       - *Commit subsequente de refatoração:* `refactor: eliminate dead code and redundant logic in <escopo> (ref #<id>)`
     - **Ordem Obrigatória:** Se houver alterações em ambos, **os testes devem ser fortalecidos e commitados primeiro**; a refatoração do código de produção ocorre e é commitada logo em seguida, mantendo a blindagem verde.
8. **Fase 8: Quality Gate & Local Audit (Auditoria Local)**
   - Execução e validação dos gates locais: linter rigoroso com `-D warnings` (Zero Warnings), formatação determinística (`fmt`), compilação/tipos limpos e piso de cobertura ≥ 90% (mirando 100%).
   - *(Validação local estrita; commit de ajuste se necessário: `fix: lint and formatting hygiene (ref #<id>)`)*
9. **Fase 9: Finish Docs & Close Ticket (Consolidação e Encerramento)**
   - Atualização da documentação para usuário (`./site`, `README.md`), consolidação do `CHANGELOG.md` e commit final de encerramento formal contendo obrigatoriamente a diretiva de fechamento canônica `(closes #<id>)`.
   - Commit: `docs: finalize documentation and changelog (closes #<id>)`

### 5.4 Tratamento para Casos Específicos
* **Em Bugfixes (`fix`):** Na Fase 3 (Spec), documenta-se a violação do contrato anterior e o comportamento correto esperado; na Fase 4 (Red), o teste falho é o teste de regressão exato que reproduz o defeito.
* **Em Refatorações (`refactor`):** Na Fase 3 (Spec), definem-se as invariantes de negócio que não podem ser quebradas; na Fase 4 (Red), adicionam-se testes cobrindo lacunas ou casos de borda antes não testados garantindo as invariantes.
* **Em Documentação Pura (`docs`):** Quando a tarefa for exclusivamente documental, as etapas de código funcional (4 a 7) registram `N/A` explícito no checklist do ticket, mantendo-se obrigatórias a branch dedicada (Fase 2), a referência intermediária `(ref #<id>)` e o fechamento `(closes #<id>)` na Fase 9.

### 5.5 Critérios de Conclusão / Definition of Done (DoD)
Um ticket só pode ser considerado concluído e encerrado via `(closes #<id>)` se satisfizer integralmente o seguinte checklist:
- [ ] Ticket aberto previamente detalhando o problema e critérios (Fase 1).
- [ ] Branch dedicado criado no formato `<tipo>/<id>-<slug>` (Fase 2).
- [ ] Documentação preliminar de contrato registrada antes do código (Fase 3).
- [ ] Commit com teste falho registrado no histórico antes da implementação (Fase 4).
- [ ] Código funcional implementado e todos os testes passando (Fase 5).
- [ ] Refatoração concluída com código limpo e idiomático (Fase 6).
- [ ] Ausência de mutantes críticos sobreviventes (Fase 7).
- [ ] Quality gates locais 100% verdes: cobertura ≥ 90%, zero warnings, formatação determinística (Fase 8).
- [ ] Documentação do `./site`, `README.md` e `CHANGELOG.md` atualizadas (Fase 9).
- [ ] Pipeline de CI/CD 100% verde.

### 5.6 Protocolo de Comandos para Agentes de IA (*Agent Dispatch*)
Em repositórios GOST desenvolvidos ou mantidos em colaboração com agentes de inteligência artificial (LLM/CLI), a interação entre o humano e o agente **DEVE** aderir a um vocabulário determinístico de comandos operacionais para mitigar ações não autorizadas (como commits arbitrários, quebra do fluxo de tickets ou merges indevidos):

| Comando | Significado Canônico | Escopo de Autorização Concedida ao Agente |
|:---:|---|---|
| **`DCPD`** | **Do, Commit, Push, Deploy** | Autorização total de ponta a ponta: implementa código e testes, valida linters/gates locais, cria commit convencional com `(closes #<id>)`, executa merge local na `main` via `cccp merge`, fecha o ticket e envia o push da `main` para acionar o pipeline de deploy. |
| **`DCP`** | **Do, Commit, Push** | Implementa, testa, commita e envia o push **exclusivamente sob demanda taxativa para colaboração multidev**, deixando a branch e o Pull Request abertos na forja para trabalho conjunto prévio. |
| **`PLAN`** | **Plan & Propose** | Modo estritamente consultivo e investigativo: pesquisa o código, elabora diagnóstico e formula proposta detalhada de plano/ticket, **sem editar arquivos e sem commitar**. |
| **`TDD`** | **Ticket, Branch, Doc, Red, Green, Refactor, Mutate, Gate, Close** | Execução rigorosa e sequencial das **9 fases canônicas** do ciclo DD-TDD: criação/validação de ticket, criação da branch dedicada, contrato preliminar em docs (**sem tocar em código**), teste falho, código funcional, refatoração limpa, eliminação de mutantes, validação dos gates e commit final de fechamento. |
| **`AUDIT`** | **Audit & Check** | Inspeção passiva do repositório: executa linters em severidade máxima (`-D warnings`), verifica cobertura (≥90%), testa mutantes, audita vulnerabilidades em dependências e checa integridade de links sem alterar o código. |
| **`FIX`** | **Fix & Clean** | Saneamento e polimento técnico: correção exclusiva de avisos de linters, eliminação de mutantes sobreviventes e aplicação de formatadores determinísticos, preservando o comportamento funcional. |
| **`RELEASE`** | **Release & Tag** | Consolidação de marcos semânticos: consolidação do `CHANGELOG.md`, bump semântico no `VERSION` (`patch`, `minor` ou `major`) via `cccp release`, geração da tag Git correspondente e push de tags. |

* **Regra de Ouro:** Agentes de IA **NUNCA DEVEM** presumir autorização para commits, merges ou modificações funcionais amplas sem instrução inequívoca ou uso dos comandos deste protocolo.

---

## 6. Estratégia e Qualidade de Testes

A integridade do software no padrão GOST baseia-se na verificação automatizada contínua com alta densidade de asserções e cobertura quantitativa e qualitativa.

### 6.1 Taxonomia de Testes Obrigatória
Todo projeto **DEVE** estruturar sua validação nas seguintes camadas, conforme aplicável à natureza do componente:

1. **Testes Unitários:**
   - Testam funções, métodos, tipos e módulos em isolamento absoluto.
   - Devem ter execução instantânea (milissegundos) e zero dependências de I/O externo.
2. **Testes Funcionais:**
   - Validam regras de negócio, transformações de dados e casos de uso específicos (dada uma entrada X, a saída deve ser Y).
3. **Testes de Integração:**
   - Validam a interoperabilidade entre componentes internos, persistência em disco (ex: SQLite), variáveis de ambiente ou protocolos de comunicação local (IPC/sockets).
4. **Testes Ponta a Ponta (E2E):**
   - Para aplicações com interface com usuário (web ou desktop), os testes **DEVEM** exercitar o sistema completo simulando a interação real de usuários em navegadores headless utilizando **Playwright**.

### 6.2 Política de Cobertura (*Code Coverage*)
* **Piso Mínimo Inviolável:** A cobertura global de código automatizada **DEVE** ser mantida permanentemente em no mínimo **`90%`**.
* **Meta de Excelência:** Todo projeto **DEVE** focar ativamente na meta de **`100%`** de cobertura nos módulos centrais de domínio e utilitários.
* **Critério de Bloqueio:** Qualquer alteração, pull request ou commit que rebaixe a cobertura global abaixo de 90% **DEVE** falhar o pipeline de integração contínua.

### 6.3 Validação Qualitativa: Testes de Mutação
* A cobertura percentual pura não atesta a eficácia das verificações. Para evitar testes cosméticos (linhas executadas sem asserções de valor real), o projeto **DEVERIA** empregar **Testes de Mutação** (*Mutation Testing*).
* O teste de mutação introduz falhas sintéticas no código (mutantes); se a suíte de testes continuar passando, o teste é considerado deficiente.

### 6.4 Hermeticidade e Determinismo
* **Isolamento de Rede:** As suítes de testes unitários, funcionais e de integração **NÃO DEVEM** depender de acesso à internet pública ou serviços remotos. Chamadas externas devem ser simuladas via *mocks*, *stubs* ou *fixtures*.
* **Limpeza de Estado:** Todo teste que instanciar arquivos, diretórios temporários ou registros em banco **DEVE** destruí-los ao término da execução, garantindo que os testes sejam estritamente idempotentes e executáveis em qualquer ordem.

---

## 7. Análise Estática e Tolerância Zero a Avisos (*Zero Warnings*)

A excelência e longevidade do código exigem rigor estrito antes de qualquer execução:

### 7.1 Tolerância Zero a Warnings
* Todo projeto **DEVE** configurar seus linters e analisadores estáticos com nível de severidade máximo.
* Avisos (*warnings*) **DEVEM** ser tratados como erros fatais (ex: flags equivalentes a `-D warnings`). Qualquer commit ou PR com alertas pendentes **DEVE quebrar o CI**.

### 7.2 Filosofia de Dependências Mínimas e Auditoria de Segurança
* **Princípio Anti-Bloat:** A adição de bibliotecas de terceiros **DEVE** ser estritamente justificada no ticket do projeto. Favorece-se soluções leves, bibliotecas padrão e código autônomo.
* **Auditoria de Vulnerabilidades:** O pipeline de CI **DEVE** executar verificações automatizadas de vulnerabilidades em dependências (*dependency auditing*).

### 7.3 Formatação Determinística Obrigatória (*Zero Bikeshedding*)
* **Autoridade Automatizada:** Todo repositório **DEVE** adotar um formatador oficial determinístico para cada linguagem do projeto (ex: `rustfmt`, `pint`, `shfmt`, `prettier`/`biome`).
* **Gate de Verificação no CI:** A conformidade de formatação **DEVE** ser verificada no pipeline de CI com flags equivalentes a `--check` ou `--test`. Qualquer inconsistência visual ou de espaçamento quebra o build.
* **Fim das Discussões de Estilo:** Nenhuma revisão de código humana ou automatizada deve debater tabs, espaços ou quebras de linha; o formatador é a única fonte da verdade.

### 7.4 Higiene Universal de Editores (`.editorconfig`)
* Todo repositório **DEVE** conter na raiz o arquivo `.editorconfig` padronizando:
  - Codificação de caracteres: `charset = utf-8`
  - Fim de linha: `end_of_line = lf`
  - Inserção de quebra ao final: `insert_final_newline = true`
  - Remoção de espaços no final de linhas: `trim_trailing_whitespace = true`
  - Indentação canônica por extensão de arquivo.

### 7.5 Validação de Integridade Documental e Hiperlinks
* Repositórios com documentação pública **DEVERIAM** integrar no pipeline de CI utilitários de auditoria de links (ex: `lychee`) para prevenir referências quebradas (404), links canônicos `hreflang` inconsistentes ou âncoras órfãs.

---

## 8. Vitrine Pública e Documentação Web

Todo repositório GOST **DEVE** manter uma vitrine pública de documentação estática no diretório `./site`.

1. **Acessibilidade Universal:** O site de documentação **NÃO DEVE** depender de etapas de compilação complexas para visualização básica; HTML/CSS/JS puros ou geradores ultra-leves são encorajados.
2. **Segurança HTTP:** O diretório de publicação **DEVE** conter cabeçalhos HTTP mínimos de segurança (arquivo `_headers` ou equivalente):
   - `X-Content-Type-Options: nosniff`
   - `X-Frame-Options: DENY`
   - `Referrer-Policy: strict-origin-when-cross-origin`
3. **Publicação Contínua:** A documentação **DEVE** ser publicada automaticamente via pipeline de CI/CD a cada push no branch principal (`main`).

### 8.1 Tipologia de Vitrines Web (`site_type: doc` vs `site_type: app`)
A norma GOST reconhece a distinção categórica entre repositórios cujo artefato público é a documentação técnica e repositórios cuja vitrine hospeda a própria aplicação gráfica/interativa:

1. **Repositórios de Documentação / Bibliotecas / CLI (`site_type: doc`):**
   - O diretório `./site/` serve diretamente a vitrine de documentação do software.
   - O arquivo `./site/index.html` apresenta a documentação técnica, tutoriais ou especificação do projeto.
2. **Repositórios de Aplicação Web / SPA / WASM (`site_type: app`):**
   - O diretório `./site/` hospeda o próprio software em execução no cliente (ex: bundles WASM em Dioxus, SPAs interativas como Navalha, Joaju ou KokoroSim).
   - O arquivo `./site/index.html` atua como ponto de montagem da interface gráfica do usuário (`<div id="main"></div>`).
   - A documentação de usuário, manuais, guias de presets e help **DEVEM** residir sob o subdiretório `./site/doc/` (ex: `https://<app>.2lp.in/doc`), preservando a raiz limpa para a experiência da aplicação.

### 8.2 Single Source of Truth para Documentação e Proibição de Duplicação
Para eliminar a divergência crônica entre READMEs, arquivos de especificação e páginas web estáticas:
1. **Redação Exclusiva em Markdown:** Todo conteúdo documental, manuais de usuário, presets e especificações de engenharia **DEVEM** ser mantidos e versionados exclusivamente em arquivos Markdown (`README.md`, `SPEC.md`, `profile.md`, `CUSTOM.md` ou diretório `docs/`).
2. **Proibição de Duplicação Manual em HTML:** É expressamente vedado manter cópias manuais de textos longos em arquivos HTML estáticos. A documentação web (seja em `site/index.html` para `site_type: doc` ou em `site/doc/` para `site_type: app`) **DEVE** consumir, referenciar ou compilar automaticamente os arquivos Markdown fonte através do pipeline do CCCP ou de micro-renderizadores client-side ultra-leves.
3. **README da Raiz como Portal Enxuto:** Em repositórios com documentação web abrangente, o `README.md` na raiz do repositório deve operar como portal conciso (até 50 linhas com propósito, badges, como iniciar e links para a vitrine oficial), evitando duplicar manuais completos que já habitam a vitrine.

### 8.3 Segregação Canônica: Vitrine Estática vs. Serviços Dinâmicos
* **Papel do `./site`:** A vitrine pública estática (publicada em `*.2lp.in` via Cloudflare Workers / Pages) destina-se estritamente à documentação, tutoriais e interfaces de demonstração estáticas.
* **Isolamento de Backends:** Aplicações dinâmicas, APIs, REPLs e daemons locais **NÃO DEVEM** expor credenciais, chaves ou sockets diretamente no diretório estático.
* **Guia de Conectividade:** A documentação estática de projetos dinâmicos **DEVE** conter uma seção clara descrevendo como acessar o serviço vivo (portas padrão, variáveis de ambiente e topologia de acesso).

### 8.2 Padrão de Resiliência para Clientes Móveis e Redes Instáveis
Sistemas que exponham interfaces web para uso em smartphones ou sob redes suscetíveis a oscilações (4G/5G, trocas de antenas, conexões móveis fracas) **DEVEM** adotar o padrão assíncrono resiliente:
1. **Idempotência no Envio:** Cada requisição do cliente deve portar um identificador exclusivo (`client_id`). Retentativas automáticas sob perda de sinal nunca devem duplicar ações no servidor.
2. **Desacoplamento de Requisição:** O backend deve persistir o job na fila e devolver resposta imediata (`HTTP 200/202`) em milissegundos, liberando a conexão de rede móvel.
3. **Short Polling Indestrutível:** Preferência por sondagem curta periódica sobre conexões persistentes contínuas (WebSockets/SSE), as quais tendem a quebrar silenciosamente durante trocas de torres de celular ou transições de rede.

### 8.3 Rodapé Canônico em Duas Linhas (Autoria, Identidade & Soberania)
Todo aplicativo com interface gráfica, vitrine pública ou documentação estática (`./site`) **DEVE** exibir o rodapé canônico formatado em **duas linhas rigorosamente estruturadas**:
* **Linha 1 — Afirmação Identitária & Transfeminista:**  
  `Orgulhosamente produzido por uma [travesti latina](https://travesti.2lp.in/<lang>/) 🏳️‍⚧️ :)`  
  - O termo *travesti* permanece rigorosamente sem tradução em todos os idiomas; o adjetivo adapta-se gramaticalmente à língua nativa com link direcionado à definição canônica.
* **Linha 2 — Metadados de Soberania, Autoria e Licença:**  
  `{Nome do App} v{Versão} • por [Lumen Pink](https://lumen.pink) • {País / Brasil} 🇧🇷 • [{Licença}](LICENSE)`  
  - Identifica o nome da aplicação, versão extraída de `VERSION`, autoria soberana, bandeira e link direto para o arquivo de licença.

---

## 9. Integração e Validação Contínua (CI/CD)

### 9.1 Separação Obrigatória de Jobs: Testes vs. Publicação
Todo pipeline de CI **DEVE** declarar seus estágios em jobs estritamente separados:

* **Job `test`** (obrigatório em todo repositório):
  - Lint, análise estática e Zero Warnings
  - Execução completa da suíte de testes (unitários, funcionais, integração, E2E)
  - Gate de cobertura (≥ 90%)
  - Auditoria de dependências
* **Job `deploy-*`** (somente quando aplicável):
  - **DEVE** declarar `needs: [test]` — nunca roda sem testes aprovados
  - **DEVE** condicionar execução ao branch principal (`if: github.ref == 'refs/heads/main'`)
  - Publica artefatos, documentação estática ou binários de release

A separação explícita garante que os estágios de validação ocorram de forma isolada, e que cada etapa seja inspecionável nos logs de CI.

* **Compilação Soberana Exclusiva na Esteira e Proibição de Builds de Release Locais:** Todo e qualquer binário final de distribuição, pacote executável ou artefato de release **DEVE** ser compilado exclusivamente através da esteira automatizada de CI/CD (Forgejo Actions / GitHub Actions) em containers herméticos a partir da branch principal integrada. É **TERMINANTEMENTE PROIBIDO** compilar releases pesadas (`cargo build --release`, cross-compilação ou empacotamento para distribuição) no ambiente local de desenvolvimento. A estação local de trabalho destina-se estritamente às fases ágeis de especificação, testes unitários rápidos, formatação (`fmt`), linter (`clippy`) e inspeção de tipos, operando obrigatoriamente com limite conservador de concorrência (`jobs = 1` no compilador).
* **Deploy Exclusivo via Esteira Automatizada de CI/CD:** Todo e qualquer deploy de aplicações, serviços, daemons e documentação **DEVE** ocorrer exclusivamente através da esteira remota de CI/CD da forja a partir da branch principal devidamente integrada. É **TERMINANTEMENTE PROIBIDO** realizar deploys manuais ou ad-hoc a partir da máquina local do desenvolvedor ou de agentes de inteligência artificial.

### 9.2 Gate Leve de Pre-Push e Gate Pré-Tag com `act`
Para harmonizar agilidade no desenvolvimento local com rigor máximo nas entregas:

1. **Pre-Push Leve e Instantâneo:** O hook `pre-push` executado pelo Git **NÃO DEVE** disparar executores de containers pesados (`act`) durante o fluxo diário de trabalho. O `pre-push` atua estritamente como um gate de conformidade semântica leve: valida a sincronia do arquivo `VERSION`, integridade de referências a tickets e consistência de tags Git, permitindo `git push` imediato.
2. **Gate Pré-Tag com `act` (Auditoria de Release):** O gate de validação em container via `act` (com infraestrutura idêntica ao runner remoto) **DEVE** ser acionado exclusivamente como **Gate Pré-Tag** no comando de criação de release (`cccp tag`). Nenhuma tag de release estável pode ser selada ou assinada sem aprovação integral prévia na auditoria do container.

### 9.3 Degradação Graciosa do Gate Pré-Tag
Se o ambiente de execução de containers (Docker daemon ou socket) não estiver disponível no momento do `cccp tag`, o gate **DEVE**:
- Emitir aviso claro indicando a ausência do executor local.
- Permitir bypass consciente exclusivamente mediante confirmação explícita do desenvolvedor (`--force` ou `--skip-ci`), registrando a exceção nos logs.

### 9.4 Distribuição Canônica Soberana na Banca e Armazenamento em R2

Para preservar as forjas Git (Codeberg/Forgejo) contra inchaço (*bloat*) e garantir distribuição atômica resiliente:

1. **A Banca como Vitrine e Repositório Soberano Oficial:**
   - A **Banca** (`https://banca.2lp.in/downloads/`) é a autoridade canônica soberana para catálogo, consulta e distribuição de binários, ferramentas CLI e instaladores do ecossistema.
   - Todo software executável distribuído **DEVE** disponibilizar subcomando de auto-atualização atômica (`<app> update` e `<app> check-update`) que consome o catálogo oficial da Banca.
2. **Proibição Absoluta de Bloat na Forja Git:**
   - Binários compilados, executáveis estáticos (`musl`), bibliotecas, instaladores e tarballs de release **NUNCA DEVEM** ser hospedados nas releases da forja Git nem comitados no histórico do repositório.
3. **Armazenamento Canônico em Object Storage (Cloudflare R2):**
   - Todos os artefatos de release compilados pela esteira **DEVEM** ser publicados diretamente em Object Storage (Cloudflare R2) sob o domínio unificado `*.2lp.in`.
   - Acesso e distribuição ocorrem via CDN global sem cobrança de taxa de saída (*egress-free*), com suporte a *HTTP Range Requests* (RFC 7233) e hashes criptográficos (`sha256`).
4. **Política de Retenção Estrita (5 Stables e 10 Nightlies):**
   - As esteiras de CI/CD **DEVEM** impor rotina de limpeza automatizada mantendo estritamente no storage:
     - **Canal Estável (`stable`)**: Até **5 versões estáveis** mais recentes, mantendo um alias simbólico fixo apontando para a última versão (`stable-latest`).
     - **Canal Nightly (`nightly`)**: Até **10 compilações atômicas de commit** mais recentes, mantendo um alias simbólico fixo apontando para a última compilação (`nightly-latest`).
   - Versões que ultrapassem o teto de retenção devem ser expurgadas automaticamente do bucket.
5. **Manifesto Atômico `index.json`:**
   - A esteira **DEVE** gerar e atualizar atomicamente um manifesto `index.json` por aplicação na raiz de seu diretório no storage (`downloads/<app>/index.json`), contendo metadados de versões, datas, arquiteturas suportadas e hashes, servindo como fonte da verdade para updaters de terminal e interfaces web.
6. **Higiene de Branches de Publicação / Pages (Stateless Single-Commit & Force Push):**
   - Quando a publicação de documentação estática envolver espelhamento em branch Git dedicado (ex: branch `pages`), o pipeline **NÃO DEVE** acumular commits históricos de builds sucessivos.
   - O pipeline **DEVE** purgar commits antigos criando uma árvore órfã isolada (`--orphan`), contendo rigorosamente um único commit limpo do snapshot atual (`docs(site): deploy [skip ci]`), publicado via `git push --force origin <temp>:pages`.

---

## 10. Internacionalização (Intl)

A norma GOST impõe disciplina linguística em camadas para conciliar cooperação global com acessibilidade multilíngue irrestrita.

### 10.1 Língua Primária e Segregação por Camadas
1. **Língua Primária do Repositório (Padrão CCCP):** Cada repositório **DEVE** declarar sua língua de trabalho primária na configuração do CCCP (`agent_var_lang`). Ela atua como idioma canônico para especificação de requisitos, fonte primária para as traduções e **fallback obrigatório** em runtime caso uma chave de tradução em outro idioma não esteja presente.
2. **Código-Fonte e Identificadores:** **DEVEM** ser escritos estritamente em **Inglês** (nomes de variáveis, funções, tipos, constantes e endpoints de API).
3. **Histórico de Commits e Tags:** **DEVEM** ser redigidos em **Inglês** seguindo a especificação de Conventional Commits (`feat:`, `fix:`, `docs:`, etc.).
4. **Triagem de Tickets:** Usuários **PODEM** relatar problemas e descrever features em qualquer idioma; ferramentas automatizadas de triagem (ex: `diva`) **DEVEM** estruturar o ticket na língua primária configurada no CCCP.

### 10.2 A Matriz Canônica das 10 Línguas Oficiais (App e Documentação)
**TODO O APLICATIVO** (interfaces de usuário, CLI, menus, mensagens de erro e logs amigáveis) e **TODA A DOCUMENTAÇÃO PÚBLICA** (`./site`) **DEVEM** fornecer paridade completa de tradução nas **10 Línguas Oficiais da Norma GOST**:

| Código ISO | Idioma | Endônimo Nativo | Direção | Papel / Escopo |
|:---:|---|---|:---:|---|
| **`en`** | English | English | LTR | Baseline técnico universal e de engenharia |
| **`pt`** | Portuguese | Português (Brasil / Portugal) | LTR | Língua nativa do ecossistema central |
| **`es`** | Spanish | Español | LTR | Integração ibérica e América Latina |
| **`fr`** | French | Français | LTR | Comunidade internacional e difusão francófona |
| **`de`** | German | Deutsch | LTR | Engenharia e ecossistema de software europeu |
| **`it`** | Italian | Italiano | LTR | Comunidade técnica e polo de computação mediterrâneo |
| **`ru`** | Russian | Русский | LTR | Referência estética e técnica da linhagem soviética / GOST |
| **`zh`** | Chinese | 中文 (简体中文) | LTR | Maior base populacional e polos tecnológicos da Ásia |
| **`ja`** | Japanese | 日本語 | LTR | Vanguarda técnica, comunidade open source e estética |
| **`ar`** | Arabic | العربية | **RTL** | Grande bloco linguístico global e representação do Oriente Médio |

### 10.3 Requisito de Direcionalidade (RTL) no App e na Doc
* Tanto a interface gráfica/CLI do aplicativo quanto as páginas de documentação para o Árabe (`ar`) **DEVEM** implementar suporte nativo a layouts **Right-to-Left** (`dir="rtl"`).
* O design de interface e CSS **DEVE** utilizar propriedades lógicas bidirecionais (ex: `margin-inline-start`, `padding-inline`, `start`/`end`) para alternância fluida entre LTR e RTL sem quebras visuais.

### 10.4 Arquitetura de Localização no Aplicativo e no Site
1. **No Aplicativo:** Pacotes de tradução mantidos em formato estruturado (ex: `locales/<lang>.json` ou catálogos nativos da linguagem) contendo 100% de paridade de chaves com relação à língua primária do CCCP. Em caso de ausência de chave em tempo de execução, o sistema **DEVE** recorrer ao texto na língua primária configurada no CCCP.
2. **Na Documentação:** Subdiretórios dedicados `./site/<lang>/index.html`. A raiz `./site/index.html` atua como roteador cliente:
   - Detecta preferência salva em `localStorage.getItem('doc_lang')`.
   - Detecta idioma do cliente via `navigator.language`.
   - Redireciona com fallback seguro para a língua primária do CCCP (ou `en/`).
   - Expõe links canônicos `rel="alternate" hreflang="<lang>"` para indexação de motores de busca.

---

## 11. Metadados Sociais e Cards de Compartilhamento (Padrão KokoroSim)

O compartilhamento de links de repositórios em plataformas sociais e mensageiros (WhatsApp, Telegram, Twitter/X, Discord, LinkedIn, iMessage) exige blindagem técnica contra recortes arbitrários e truncamento de texto.

### 11.1 Dimensões e Formato do Banner Gráfico
* **Resolução Obrigatória:** Exatamente **1200 × 630 pixels** (proporção 1.91:1, formato PNG otimizado, RGB).
* **Localização Padrão:** `./site/assets/og_preview.png`.

### 11.2 Safe Zone 4:3 Centralizada (Anti-Recorte)
Plataformas móveis (notadamente o WhatsApp) costumam recortar o banner em **1:1 (quadrado)** ou em balões estreitos de chat. Para garantir legibilidade absoluta:
* **Miolo Central Útil:** Todo o conteúdo textual, logotipos, badges e selos **DEVE** estar contido dentro de um retângulo central com largura entre **720px e 840px** e altura de **500px**.
* **Sangrias Laterais de Segurança:** As faixas de **180px a 240px** em cada lateral **NÃO DEVEM** conter texto nem elementos essenciais, servindo unicamente para gradiente ou textura de fundo.
* **Margens Verticais:** Top padding mínimo de **52px** e Bottom padding mínimo de **85px** (para evitar que carimbos de hora, URLs de rodapé ou botões de sobreposição encubram a mensagem).

### 11.3 Limites Rígidos de Caracteres (Anti-Truncamento)
Para impedir que redes sociais apliquem elipses (`...`) decepando palavras:

1. **Meta Tag `<meta property="og:title">` e `twitter:title`:**
   - **Faixa Segura:** **50 a 65 caracteres** (limite absoluto: 70 caracteres).
2. **Meta Tag `<meta property="og:description">` e `twitter:description`:**
   - **Faixa Segura:** **120 a 155 caracteres** (limite absoluto: 160 caracteres).
   - Deve ser auto-contida e evitar redundância com o título.
3. **Badge Superior Impresso na Imagem:**
   - **Máximo:** **35 caracteres**, em caixa alta (ex: `OPEN SOURCE // RUST + WASM`).
4. **Título Principal na Imagem:**
   - **Máximo:** **35 a 45 caracteres** (distribuído em 1 ou 2 linhas nítidas, tipografia bold de alto contraste).
5. **Subtítulo / Linha de Apoio na Imagem:**
   - **Máximo:** **50 a 60 caracteres** em linha única.
6. **Rodapé Integrado na Imagem:**
   - Indicador de URL canônica limpa (`<repo>.2lp.in`) e termo de licença contidos na largura de 720px.

---

## 12. Arquitetura de Domínio e SOLID Pragmático

A longevidade, manutenibilidade e velocidade de evolução de um projeto dependem do desacoplamento estrito entre as regras centrais de negócio e os subsistemas voláteis de entrada/saída (I/O). O GOST adota uma abordagem de arquitetura hexagonal (Portas e Adaptadores) pragmática, orientada à hermeticidade e avessa a overengineering.

### 12.1 Desacoplamento de Domínio e DIP (Ports & Adapters)
1. **Inversão de Dependência (DIP):** Módulos centrais de alto nível (*Core / Domínio*) **NÃO DEVEM** depender de módulos de baixo nível (bancos de dados, frameworks web, bibliotecas de rede, filesystem, drivers de terminal). Ambos **DEVEM** depender de abstrações (interfaces ou traits).
2. **Direção Unidirecional de Dependências:** A dependência aponta invariavelmente para dentro. O Core **NUNCA DEVE** importar ou referenciar adaptadores de infraestrutura.
3. **Portas de Entrada (*Driving / Input Ports*):** Interfaces expostas pelo Core que definem os casos de uso do sistema (invocadas por controllers HTTP, comandos CLI, consumidores de fila).
4. **Portas de Saída (*Driven / Output Ports*):** Contratos (interfaces/traits) declarados pelo Core para suprir suas necessidades de comunicação externa (persistência, envio de eventos, relógio do sistema, clientes HTTP). Os adaptadores de infraestrutura implementam esses contratos.

### 12.2 Hermeticidade Absoluta e Testes em Memória
1. **Determinismo em Milissegundos:** A suíte de testes unitários do Core **DEVE** executar em isolamento absoluto de I/O, rodando em microssegundos ou milissegundos sem depender de sockets, containers, daemons externos ou arquivos em disco.
2. **Adoção Canônica de In-Memory Fakes:** Para satisfazer as Portas de Saída nos testes de domínio, o projeto **DEVE** priorizar o uso de **Adaptadores em Memória** (*In-Memory Fakes* ou *stubs* leves) em vez de bibliotecas pesadas de mock dinâmico.
3. **Repositórios em Memória:** Estruturas simples baseadas em arrays, listas ou mapas nativos da linguagem que simulam o armazenamento preservando a semântica da porta. Isso elimina complexidade de mocking frágil, facilita refatorações seguras e garante hermeticidade total.

### 12.3 Filosofia de Tipos Primitivos e Validação na Borda (*Parse, Don't Validate*)
1. **Princípio Anti-Bloat:** O GOST rejeita o excesso de cerimônia de *Value Objects* anêmicos (proliferação de classes invólucro vazias para cada string ou número básico sem comportamento real associado).
2. **Validação e Parse na Borda (*Edge Validation*):** Toda entrada de dados brutos (requisições HTTP, argumentos de linha de comando, mensagens de fila, payloads JSON) **DEVE** ser sanitizada, validada e tipada na borda do sistema (adaptadores de entrada, DTOs de request, validadores de borda).
3. **Primitivos Confiáveis no Core:** Uma vez que os dados ultrapassam a fronteira da borda e adentram o Core, este opera com tipos primitivos nativos da linguagem (`string`, `int`, `float`, `bool`, tuplas, structs transparentes) ou tipos agregados claros, confiando nos dados já parsed. Tipos dedicados são reservados para conceitos que realmente encerram invariantes complexas ou regras de negócio vivas.

### 12.4 Modelagem de Estados Finitos com Enums Nativos
1. **Proibição de Tipos Fracos para Estado:** É estritamente **PROIBIDO** o uso de strings soltas (*stringly-typed*) ou inteiros arbitrários para representar estados de processos, status de ciclo de vida ou categorias fixas.
2. **Enums Nativos da Linguagem:** Estados de entidades e máquinas de estado finitas **DEVEM** ser expressos através de `enum` nativo da linguagem (ex: *backed enums* em PHP 8.1+, `enum` em Rust).
3. **Coesão Comportamental:** Métodos de verificação de transições válidas, predicados e propriedades derivadas **DEVEM** ser encapsulados no próprio enum ou no modelo de domínio, permitindo verificação estática de exaustividade em tempo de compilação ou análise estática (Zero Warnings).

### 12.5 Princípio de Gradação Arquitetural para Ferramentas e Serviços
A estrutura arquitetural do repositório **DEVE** ser proporcional à sua responsabilidade:
1. **Micro-ferramentas e Scripts Únicos:** Utilitários de CLI de escopo restrito ou filtros de terminal **NÃO DEVEM** adotar camadas excessivas de indireção. Devem manter estrutura modular direta (ex: arquivo único ou módulo único contendo funções puras e ponto de entrada).
2. **Aplicações e Serviços:** Sistemas com regras de domínio ricas, múltiplos casos de uso ou múltiplos adaptadores de transporte **DEVEM** implementar a segregação estrita entre Core (Domínio/Aplicação) e Infraestrutura (Adaptadores/Configuração).

---

## 13. Aderência a Padrões de Sistema e Operação (XDG, 12-Factor & OCI)

A integridade e interoperabilidade do software exigem conformidade com convenções maduras do ecossistema Unix/Linux e da computação em nuvem moderna.

### 13.1 Diretórios e Sistema de Arquivos (XDG Base Directory)
Todo software em conformidade com o GOST que manipule configurações, dados persistentes, caches ou sockets em disco **DEVE** seguir a especificação [XDG Base Directory](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html).
* **Proibição de Poluição da `$HOME`:** É estritamente proibido criar arquivos e diretórios ocultos soltos na raiz de `$HOME` (ex: `~/.app` ou `~/.app_history`).
* **Mapeamento de Variáveis e Fallbacks Canônicos:** Caso a variável de ambiente não esteja definida ou esteja em branco, o software **DEVE** aplicar obrigatoriamente os seguintes caminhos padrão:
  - **Configurações:** `$XDG_CONFIG_HOME` ➔ Fallback: `~/.config/<app>/`
  - **Dados Persistentes:** `$XDG_DATA_HOME` ➔ Fallback: `~/.local/share/<app>/`
  - **Estado Dinâmico & Histórico:** `$XDG_STATE_HOME` ➔ Fallback: `~/.local/state/<app>/`
  - **Cache Volátil:** `$XDG_CACHE_HOME` ➔ Fallback: `~/.cache/<app>/`
  - **Sockets e IPC em Runtime:** `$XDG_RUNTIME_DIR` ➔ Fallback: `/run/user/$UID/<app>/` (ou `/tmp/<app>-$UID/` com permissão restrita).
* **Permissões de Segurança:** Diretórios e arquivos contendo credenciais, tokens ou certificados sob o padrão XDG **DEVEM** ser criados com permissões restritas de acesso (`chmod 0700` para diretórios e `chmod 0600` para arquivos).

### 13.2 Precedência de Configuração e Formato de Parâmetros
O software **DEVE** resolver parâmetros operacionais seguindo a seguinte hierarquia estrita de precedência (da **maior** para a **menor** prioridade):
1. **Flags de Linha de Comando (CLI):** Parâmetros passados explicitamente na chamada (`--port 8080`).
2. **Variáveis de Ambiente do Processo:** Variáveis exportadas no shell ou container (`APP_PORT=8080`).
3. **Arquivo `.env` Local:** Arquivo de conveniência no diretório atual de execução (*cwd*).
4. **Configuração Local do Projeto:** Arquivos de projeto no repositório (`./config.env` ou `./.apprc`).
5. **Configuração de Usuário no XDG:** `$XDG_CONFIG_HOME/<app>/config.env`.
6. **Configuração Global do Sistema:** `/etc/<app>/config.env`.
7. **Defaults Embutidos no Binário:** Valores padrão seguros compilados internamente.

* **Formato de Configuração Plano (`CHAVE=VALOR`):** Para garantir interoperabilidade direta com scripts POSIX Shell sem dependências pesadas de compilação ou parsing (anti-bloat), arquivos de configuração editáveis **DEVEM** adotar formato de pares chave-valor compatíveis com Shell (`KEY="value"`). Para metadados hierárquicos complexos que ultrapassem chave-valor simples, o formato canônico é o **JSON**.

### 13.3 Filosofia Universal Twelve-Factor (12-Factor)
O manifesto [Twelve-Factor App](https://12factor.net/) **DEVE** orientar o design não apenas de microsserviços web, mas universalmente de qualquer serviço, daemon, bot ou ferramenta de linha de comando:
1. **I. Codebase Único:** Rastreado em controle de versão com histórico rastreável via tickets (Seção 5).
2. **II. Dependências Explícitas:** Declaradas em manifestos e congeladas em arquivos de lock (`Cargo.lock`, `composer.lock`), sem dependências implícitas do sistema hospedeiro.
3. **III. Configurações no Ambiente:** Toda configuração que varie entre execuções ou ambientes (dev, test, prod) reside estritamente em variáveis de ambiente.
4. **IV. Backing Services como Recursos Anexados:** Bancos de dados, filas e serviços de terceiros são vinculados por endereços/URLs configuráveis via ambiente.
5. **V. Segregação de Build, Release, Run:** O processo de compilação produz artefato imutável; a execução apenas injeta o ambiente.
6. **VI. Processos Stateless:** O processo pode ser encerrado ou replicado a qualquer momento; estado persistente reside em backing services ou no caminho XDG correto.
7. **VII. Port & Socket Binding:** A aplicação provê serviços conectando-se diretamente a portas TCP ou a Unix Domain Sockets locais.
8. **VIII. Concorrência via Modelo de Processos:** Escala através da multiplicação de processos independentes.
9. **IX. Descartabilidade (*Disposability*):** Inicialização instantânea e encerramento gracioso reagindo a `SIGTERM` e `SIGINT`.
10. **X. Paridade Dev/Prod:** Minimização de divergências entre ambiente local e produção (validada via gate de pre-push com `act`).
11. **XI. Logs como Fluxos de Eventos (*Event Streams*):** O software **NUNCA** gerencia ou rotaciona arquivos de log internamente. Ele escreve eventos contínuos em `stdout`/`stderr` e delega a coleta ao ambiente (systemd journal, docker logging).
12. **XII. Processos de Administração Pontuais:** Migrações e tarefas de manutenção rodam como processos pontuais efêmeros.

### 13.4 Disciplina de Streams Unix, Sinais POSIX e `NO_COLOR`
1. **Segregação de Streams:**
   - `stdout` é reservado **exclusivamente para a saída de dados útil** (payloads puros para encadeamento em pipes `|`).
   - `stderr` **DEVE** concentrar todos os logs, avisos, spinners interativos de progresso e mensagens de erro.
2. **Aderência ao Padrão `NO_COLOR`:** O software **DEVE** aderir à especificação [no-color.org](https://no-color.org/). Se a variável de ambiente `NO_COLOR` estiver presente (com qualquer valor), todas as sequências de escape ANSI de cores **DEVEM** ser desativadas.
3. **Detecção Automática de TTY:** Quando `stdout` ou `stderr` não estiverem conectados a um terminal interativo (`!isatty()`), cores, animações e spinners interativos devem ser desabilitados automaticamente, prevenindo poluição em logs de automação e pipelines CI.
4. **Tratamento de Sinais:** Daemons **DEVEM** interceptar `SIGINT` e `SIGTERM` para graceful shutdown (finalizando transações e destruindo sockets/locks) e `SIGHUP` para recarregar arquivos de configuração em tempo de execução.
5. **Códigos de Saída (Exit Codes):** `0` para sucesso; `1` para falhas gerais; `2` ou `64` (`EX_USAGE`) para erros de argumentos de CLI; `130` quando interrompido por `SIGINT` (Ctrl+C).

### 13.5 Protocolos de Rede, Criptografia e Transporte
1. **Banimento de Protocolos Obsoletos:** É estritamente **PROIBIDO** o uso ou suporte a SSLv2, SSLv3, TLS 1.0 e TLS 1.1 (conforme RFC 8996). O piso aceito é **TLS 1.2**, com padrão mandatório em **TLS 1.3**.
2. **Hierarquia de Transporte:** Comunicações de rede e APIs web **DEVEM** priorizar a pilha moderna: **QUIC / HTTP/3** > **HTTP/2 (Multiplexado sobre TLS 1.3)** > **HTTP/1.1**.
3. **Unix Domain Sockets para IPC Local:** Serviços e daemons comunicando-se na mesma máquina **DEVEM** priorizar **UNIX Domain Sockets** em vez de binds em loopback TCP `127.0.0.1`, reduzindo latência de rede e aplicando controle de acesso nativo baseado no UID/GID do processo chamador.

### 13.6 Padrões Normativos para Docker e OCI Containers
* **Containerização Mandatória Universal e Proibição de Bare Metal:** Todo aplicativo, daemon, microsserviço ou API em ambientes de desenvolvimento, homologação e produção **DEVE** rodar estritamente containerizado em conformidade com as normas OCI (Docker / Podman / compose.yaml). É expressamente **PROIBIDO** executar aplicações de serviço diretamente em *bare metal* (no sistema operacional hospedeiro sem isolamento de container), prevenindo conflitos de dependências, mutabilidade de estado e vazamento de portas/sockets no host.

Projetos que distribuem imagens ou executam via containers OCI **DEVEM** satisfazer:
1. **Multi-Stage Builds:** Todo `Dockerfile` **DEVE** separar explicitamente a etapa de compilação/build da imagem final de distribuição.
2. **Imagens Finais Mínimas:** Imagens de runtime **NÃO DEVEM** conter compiladores, headers ou utilitários dispensáveis. Binários compilados (Rust/Go) devem utilizar bases `scratch` ou `distroless`; stacks interpretadas (PHP) devem utilizar `alpine` ou `debian-slim`.
3. **Execução Obrigatória Sem Root (*Non-Root User*):** É proibido executar o serviço como `root`. O `Dockerfile` **DEVE** declarar explicitamente um usuário não privilegiado (`USER 1000:1000`, `USER app` ou `USER nobody`) antes do ponto de entrada.
4. **Banimento da Tag `:latest`:** Imagens base **DEVEM** fixar versões imutáveis semânticas (ex: `debian:bookworm-slim`, `rust:1.80-alpine3.20`), prevenindo quebras silenciosas por mutação externa de tags.
5. **Arquivo `.dockerignore`:** Presente na raiz, bloqueando a cópia acidental de `.git`, `.env*`, credenciais locais, caches e diretórios de build para as camadas da imagem.
6. **Gerenciamento de PID 1:** Aplicações em container devem utilizar gerenciadores de init leves (ex: `tini` ou flag `--init`) caso o processo não trate diretamente sinais de encerramento do kernel.
7. **Padrão `compose.yaml`:** A orquestração local para múltiplos serviços deve adotar o nome canônico aberto `compose.yaml` (especificação aberta do Compose).

### 13.7 Autenticação Single-User com Chave de API
Para microsserviços, APIs e daemons sob operação de usuário único:
1. **Esquema de Autorização:** A autenticação HTTP **DEVE** aceitar o cabeçalho `Authorization: Bearer <CHAVE>` (com suporte a `X-Api-Key: <CHAVE>` para flexibilidade).
2. **Injeção de Credenciais e Padrão `_FILE`:** A chave deve ser lida a partir de variável de ambiente (`<APP>_API_KEY`) ou via arquivo através do sufixo `_FILE` (`<APP>_API_KEY_FILE=/run/secrets/key`), garantindo suporte nativo a Docker e Systemd Secrets.
3. **Comparação Timing-Safe:** A verificação da chave de API em memória **DEVE** utilizar comparação em tempo constante (`hash_equals` em PHP, `subtle::ConstantTimeEq` em Rust), blindando contra ataques de temporização (*timing attacks*).

---

## 14. Perfis de Implementação (*Profiles*)

Esta norma GOST estabelece requisitos agnósticos a fornecedores e tecnologias. Ambientes específicos (organizações, indivíduos ou ecossistemas) **DEVEM** declarar suas escolhas concretas de infraestrutura em documentos de perfil desacoplados da norma raiz, referenciados através de `GOST_PROFILE_URL` no arquivo `GOSTVER` e baseados no modelo canônico disponível em `profiles/template.md`.

Um perfil de implementação define:
- Qual forja de código hospeda os repositórios (Codeberg, GitHub, GitLab, Forgejo auto-hospedado, etc.).
- Qual infraestrutura serve o portal de documentação e sites públicos (Cloudflare Workers, Pages, Nginx, etc.).
- Quais ferramentas e CLI gerenciam versionamento, tickets e hooks (ex: `cccp`).
- Quais frameworks e utilitários executam a suíte de testes, medem cobertura/mutação e aplicam linters com Zero Warnings.
- Como são gerados e validados os banners com Safe Zone 4:3 e os pacotes de internacionalização.
- Onde se localiza o catálogo central de descoberta de projetos.

### 14.1 Herança em Cascata de Perfis e Diretiva `extends`
Para conciliar padrões organizacionais com necessidades personalizadas de projetos ou autores individuais sem duplicar definições, os perfis de implementação suportam **herança em cascata (*profile inheritance*)**:

1. **Declaração do Pai (`extends`):** Um perfil derivado **PODE** declarar uma URL pai através do campo `extends:` em seus metadados (no cabeçalho YAML/Markdown ou tabela de identificação):
   ```yaml
   ---
   id: lumen
   name: "Perfil de Implementação Lumen"
   extends: "https://kollontai.2lp.in/profile.md"
   target_gost: "1.4.0"
   maintainer: "Lumen Pink <hi@lumen.pink>"
   ---
   ```
2. **Resolução e Precedência em Cascata:**
   - O perfil filho herda integralmente todas as seções, diretivas e convenções do perfil pai.
   - Qualquer seção, chave ou diretiva declarada no perfil filho possui **precedência estrita e soberana (*deep override*)**, sobrescrevendo a definição correspondente herdada do perfil pai.
   - Caso uma diretiva não seja declarada no filho, a regra do pai é aplicada automaticamente.
3. **Resolução na Ponta:** Projetos declaram em seu arquivo `GOSTVER` exclusivamente a URL do perfil folha (`GOST_PROFILE_URL`). As ferramentas de auditoria e conformidade (`cccp gost sync`, `gost-conform.sh`) resolvem a cadeia recursiva de herança garantindo conformidade uniforme.

### 14.2 Cascata de Especificações por Linguagem e Matriz de Regras (`rules.json`)
A herança em cascata estende-se simetricamente aos guias de linguagem e às matrizes de regras:

1. **Especificações de Linguagem (`.gost/languages/<lang>.md`):**
   - A especificação base é herdada da norma central (`specs/languages/<lang>.md`).
   - Perfis intermediários (Kollontai) e derivados (Lumen) especializam ferramentas, compiladores e linters (ex: Rust 1.99, PHP 8.5/Pest, Biome, Shellspec).
   - O compilador do perfil anexa e sobrepõe essas camadas em ordem de precedência.
2. **Matriz de Regras de Conformidade (`.gost/rules.json`):**
   - O arquivo de regras é fundido via *deep merge* por identificador (`id`).
   - Se um perfil filho redefinir uma regra existente (ex: alterando `threshold` de cobertura de testes em `GOST-06-TEST-COVERAGE` de 90% para 95%), a definição do perfil filho sobrepõe o pai.
   - Regras inéditas adicionadas por perfis são integradas ao conjunto executado pela auditoria do CCCP.

### 14.3 Sobreposição Soberana Local do Repositório (`CUSTOM.md` e `custom_rules.json`)
Quando um repositório necessitar de regras estritas exclusivas que não pertencem a nenhum perfil compartilhado:
1. **Regras Documentais (`CUSTOM.md`):** Qualquer arquivo `CUSTOM.md` presente na raiz do projeto é anexado como camada final soberana na compilação do `.gost/GOST.md`.
2. **Exceções e Calibragens Locais:** Orientações presentes no `CUSTOM.md` (como idioma oficial do projeto, exceções justificadas de arquitetura ou diretrizes específicas de empacotamento) possuem precedência máxima sobre todos os perfis herdados.
3. **Regras de Validação Locais (`custom_rules.json`):** Se presente, aplica o merge final sobre o `.gost/rules.json`.

---

<!-- ===================================================================== -->
<!-- CAMADA 2: https://kollontai.2lp.in/profile.md -->
<!-- ===================================================================== -->

# Perfil de Implementação: Kollontai
**Identificador do Perfil:** `kollontai`  
**Norma Base:** GOST `1.5.0`  
**Autor:** lumenpink  
**Status:** Ativo / Produção

---

## 1. Visão Geral

Este documento define as especificações concretas, serviços de nuvem, ferramentas de automação e infraestrutura adotadas no perfil **Kollontai** (homenagem à revolucionária soviética, feminista e diplomata Alexandra Kollontai) para satisfazer todos os requisitos normativos do padrão **GOST**.

---

## 2. Forja de Código e Repositórios Remotos

### 2.1 Forja Primária
* **Instância:** [Codeberg](https://codeberg.org/) (Software Forgejo).
* **Organização / Usuário:** `lumenpink`.
* **URL do Repositório:** `https://codeberg.org/lumenpink/<nome-do-repo>`.
* **Remote Git SSH Padrão:** `ssh://git@codeberg.org/lumenpink/<nome-do-repo>.git`.
* **Branch Padrão:** `main`.

### 2.2 Criação Automatizada via API
A criação de repositórios remotos utiliza a API v1 do Codeberg autenticada via `$CODEBERG_TOKEN`:

```bash
curl -X POST \
  -H "Authorization: token $CODEBERG_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "<nome-do-repo>",
    "description": "<Descrição curta e objetiva do projeto>",
    "private": false,
    "auto_init": false
  }' \
  "https://codeberg.org/api/v1/user/repos"
```

### 2.3 Redundância Mandatória: Repositório de Backup / Espelho (*Mirror*)
Todo repositório no perfil Kollontai deve implementar redundância geográfica e resiliência contra indisponibilidade de forjas através de um repositório secundário de backup:

1. **Forja de Backup Padrão:**
   * **Padrão Canônico:** **GitHub** (`https://github.com/lumenpink/<nome-do-repo>`), operando como espelho secundário de contingência e preservação.
   * **Exceções Declaradas no `.cccprc`:** Repositórios com propósitos específicos (como notas/vaults do Obsidian em `tildegit.org` ou instâncias próprias) podem utilizar forjas alternativas, desde que declaradas explicitamente na chave `backup_forge` ou `backup_url` do `.cccprc`.
2. **Paridade Estrita de Metadados & Sinalização de Espelho:**
   * **Metadados Idênticos:** Nome do repositório, tópicos (*topics/tags*), homepage/website (`https://<nome-do-repo>.2lp.in`), licença e visibilidade (público/privado) **DEVEM** ser estritamente iguais entre o repositório principal e o backup.
   * **Distinção Mandatória:** No repositório de backup, a descrição oficial (*About / Description*) **DEVE** declarar explicitamente que se trata de uma cópia de contingência, apontando diretamente para o repositório principal no Codeberg:
     ```text
     [MIRROR / BACKUP] <Descrição do projeto>. Repositório canônico e issues em: https://codeberg.org/lumenpink/<nome-do-repo>
     ```
   * Issues, pull requests e contribuições devem ser mantidos e tratados exclusivamente na forja canônica no Codeberg.
3. **Estratégia de Sincronização e Espelhamento Soberano (Zero Dual Push Local):**
   * **Push Unidirecional Local:** O ambiente local do desenvolvedor mantém um único destino de push configurado no `origin`, apontando exclusivamente para a forja primária no Codeberg:
     ```bash
     git remote set-url origin ssh://git@codeberg.org/lumenpink/<nome-do-repo>.git
     ```
     O desenvolvedor **NUNCA** faz push manual duplo local para ambas as forjas.
   * **Push Secundário Automatizado a partir da Forja Principal:** É responsabilidade estrita da infraestrutura do repositório canônico no Codeberg replicar as alterações para o espelho secundário:
     - **Push Mirror Nativo do Codeberg / Forgejo (Recomendado):** Configurado diretamente nas opções do repositório no Codeberg (*Configurações > Repositório > Espelhamento de Envio / Push Mirror*), onde a própria forja replica commits, branches e tags para o GitHub automaticamente após qualquer recebimento de push.
     - **Job de Espelhamento no Forgejo Actions (CI):** Como alternativa ou contingência, um workflow dedicado no CI acionado a cada push em branches e tags que executa o push espelhado para o GitHub autenticado via secret (`GITHUB_MIRROR_TOKEN` ou chave SSH).

---

## 3. Gestão de Commits e Hooks (`cccp`)

O ecossistema Kollontai utiliza a CLI **`cccp`** como ferramenta central de versionamento, hooks do Git e integridade semântica.

### 3.1 Arquivo `.cccprc`
Todo repositório no perfil Kollontai **DEVE** conter na raiz o arquivo `.cccprc`:

```ini
# Forja Canônica Primária (Codeberg)
codeberg_owner = lumenpink
codeberg_repo = <nome-do-repo>

# Redundância & Repositório de Backup (Mandatório)
# backup_forge padrão: github (opções: github, tildegit, gitlab, custom)
backup_forge = github
backup_owner = lumenpink
backup_repo = <nome-do-repo>
# Ou declaração direta de URL de transporte:
# backup_url = git@github.com:lumenpink/<nome-do-repo>.git

# Diretivas de Agentes e Idioma
agent_md_url = https://gist.githubusercontent.com/lumenpink/ebaded4f8bfb956346c28b64f48754fb/raw/c48a2f9cfb4fd73dcb944c4e47a5b17b8f3a3a2e/AGENTS.md
agent_var_lang = english
```

### 3.2 Ativação e Bloqueios dos Hooks
Após criar ou clonar o projeto, os hooks **DEVEM** ser inicializados via:
```bash
cccp install
```
Isso instala:
* **`commit-msg`:**
  - Validador rigoroso de Conventional Commits.
  - **Enforcement de Tickets:** Rejeita qualquer commit que não contenha a referência explícita ao ticket da issue no padrão `(ref #<id>)` ou `(closes #<id>)`. Bloqueia commits sem ticket na raiz.
* **`post-commit`:** Sincronizador atômico de versão (`VERSION`), atualização do `CHANGELOG.md` e metadados.
* **`pre-push`:** Gate de CI local com `act` (ver Seção 5.8).

---

## 4. Publicação Web e Documentação (`*.2lp.in`)

### 4.1 Infraestrutura: Cloudflare Workers
A vitrine de documentação pública (`./site`) é publicada na borda da Cloudflare sob o domínio unificado `*.2lp.in`.

### 4.2 Arquivo `wrangler.jsonc`
```jsonc
{
  "$schema": "node_modules/wrangler/config-schema.json",
  "name": "<nome-do-projeto>",
  "compatibility_date": "2024-09-23",
  "routes": [
    {
      "pattern": "<nome-do-projeto>.2lp.in",
      "custom_domain": true
    }
  ],
  "assets": {
    "directory": "./site"
  }
}
```

### 4.3 Cabeçalhos de Segurança (`site/_headers`)
```http
/*
  X-Content-Type-Options: nosniff
  X-Frame-Options: DENY
  Referrer-Policy: strict-origin-when-cross-origin
```

### 4.4 Fallback: Codeberg Pages (Stateless Orphan Force Push)
Como contingência e redundância estática, o pipeline de CI publica a pasta `./site` também no Codeberg Pages (`https://lumenpink.codeberg.page/<nome-do-repo>/`).
Para evitar acúmulo de histórico de builds e inchaço do repositório no Codeberg, o branch `pages` é tratado como estritamente efêmero: a publicação gera uma árvore órfã isolada com commit único do snapshot atual e executa `git push --force origin pages`.

### 4.5 Armazenamento Canônico de Artefatos & Releases (Cloudflare R2)
No ecossistema Kollontai, binários compilados (executáveis `musl`, APKs Android, bundles `.tar.gz`) **NÃO** são acumulados nem hospedados em releases no Codeberg.
Todos os artefatos de release são enviados diretamente para buckets Cloudflare R2 via `npx wrangler r2 object put` e distribuídos na borda sob `*.2lp.in` com suporte nativo a streaming HTTP Range (RFC 7233) e zero taxa de saída.

### 4.6 Topologia de Serviços Dinâmicos no Perfil Kollontai
Para repositórios que implementam serviços web dinâmicos, daemons ou REPLs (como o Matraca):

### 4.7 Tipologia de Vitrines (`site_type: doc` vs `site_type: app`)
No perfil Kollontai:
- **Repositórios de Bibliotecas, Daemons, Ferramentas e Perfis (`site_type: doc`):** A raiz `site/index.html` expõe diretamente a vitrine de documentação do software.
- **Repositórios de Aplicações Gráficas / Dioxus / WASM (`site_type: app`):** A raiz `site/index.html` hospeda o binário/SPA da aplicação (ex: Navalha, Joaju). A documentação, guias de usuário e presets residem obrigatoriamente sob `site/doc/`.

### 4.8 Single Source of Truth para Documentação
- Todo conteúdo autoral reside exclusivamente em Markdown (`docs/`, `SPEC.md`, `profile.md`, `CUSTOM.md`).
- A documentação em `site/` consome ou compila os arquivos Markdown fonte automaticamente, proibindo edição manual de HTML concorrente.
- O `README.md` funciona como portal conciso (até 50 linhas), delegando manuais extensos para a vitrine oficial `*.2lp.in`.
1. **Camada Privada Primária (Mesh VPN):**
   - Acesso via **Tailscale** (`http://100.x.y.z:<porta>`).
   - Todo tráfego entre smartphones, laptops e estações de desenvolvimento permanece criptografado ponto a ponto na rede WireGuard privada sem abertura de portas no firewall.
2. **Camada de Borda Autenticada (Cloudflare Tunnel):**
   - Quando for conveniente o acesso web com certificado SSL público sem a VPN ativa no cliente móvel, utiliza-se o **`cloudflared`** direcionando para o subdomínio `app.<repo>.2lp.in` com autenticação obrigatória no aplicativo.

---

## 5. Stack e Ferramental Padrão de Testes

Para atender aos requisitos de alta cobertura (>90%, foco em 100%) e mutação da norma GOST, o ecossistema Kollontai adota as seguintes ferramentas padronizadas por linguagem:

### 5.1 Shell Scripting / CLI de Sistema
* **Dialeto Canônico:** POSIX Shell estrito (`/bin/sh`) como baseline universal portável; Bash 5.2+ (`#!/usr/bin/env bash`) quando recursos avançados (arrays, controle fino de processos) forem expressamente justificados.
* **Runner de Testes:** **`shellspec`** (BDD framework para POSIX shell / bash).
* **Estrutura:** Especificações organizadas sob `spec/` (ex: `spec/*_spec.sh`).
* **Cobertura:** Mensuração via `kcov` integrada ao shellspec (`shellspec --kcov`), mantendo meta de bloqueio $\ge 90\%$.
* **Linter & Formatação:** `shellcheck` (zero avisos tolerados) e `shfmt -d -i 2 -ci -bn .`.

### 5.2 Rust (Sistemas, Milters, Daemons, CLI e Clientes)
* **Compilador Canônico:** **`rustc >= 1.99.0`** (Estado da arte).
* **Edição Canônica Obrigatória:** **`edition = "2024"`** e `rust-version = "1.99"` declarados em todo `Cargo.toml`.
* **Runner de Testes:** **`cargo test`** (testes unitários in-source em `src/` e testes de integração em `tests/`).
* **Cobertura:** **`cargo-tarpaulin`** ou **`cargo-llvm-cov`** (com flag obrigatória de bloqueio `--fail-under 90`).
* **Testes de Mutação:** **`cargo-mutants`** para detecção de testes ineficazes e mutantes sobreviventes.
* **Target Estático de Distribuição:** `x86_64-unknown-linux-musl` e `aarch64-unknown-linux-musl` (compilação estática pura sem glibc para distribuição hermética).

### 5.3 PHP (IndieWeb, APIs e Aplicações Web)
* **Versão Canônica:** **`PHP >= 8.5.0`** (Estado da arte).
* **Suporte Reverso Mandatório:** Compatibilidade garantida com **`PHP >= 8.2`** (`"php": "^8.2 || ^8.3 || ^8.4 || ^8.5"` no `composer.json`), proibindo quebras de sintaxe ou dependências exclusivas de versões superiores sem fallback seguro.
* **Tipagem Estrita:** `declare(strict_types=1);` mandatório em 100% dos arquivos PHP.
* **Runner de Testes:** **`pest`** (PHP Testing Framework elegante e veloz).
* **Cobertura:** Execução contínua com verificação estrita:
  ```bash
  vendor/bin/pest --coverage --min=90
  ```
* **Testes de Mutação:** **`infection`** (`vendor/bin/infection --min-msi=80 --min-covered-msi=85`).
* **Análise Estática & Estilo:** `vendor/bin/phpstan analyse --level=max` (configurado com target 8.2 no baseline de compatibilidade) e `vendor/bin/pint --test`.

### 5.4 Node.js & TypeScript (Serviços, Workers e Tooling)
* **Runtime Canônico:** **`Node.js >= 24.16.0 LTS`** (Estado da arte).
* **Padrão de Módulos:** **ESM nativo obrigatório** (`"type": "module"` no `package.json`).
* **TypeScript:** **`TypeScript >= 5.8`** sob modo estrito total (`"strict": true`).
* **Linter & Formatação:** **`biome`** (`npx biome check .` / `npx biome format .`) como ferramenta canônica e ultrarrápida baseada em Rust.
* **Runner de Testes:** **`vitest`** ou **`node:test`** nativo com cobertura v8 $\ge 90\%$.

### 5.5 Interfaces de Usuário, Web & Mobile: Dioxus 0.8
O framework declarativo universal adotado no perfil Kollontai para desenvolvimento de aplicações gráficas reativas (Desktop, Web WASM e Mobile) é o **Dioxus 0.8** (`dioxus = "0.8"`):

* **Multiplataforma Nativa:**
  - **Desktop:** Renderização rápida via Tao/Wry.
  - **Web (WASM):** Single Page Apps estáticas com renderização client-side ultra-rápida sem NodeJS em runtime.
  - **Mobile:** Android NDK e iOS compilados diretamente com Cargo.
* **Manifesto Canônico (`Dioxus.toml`):**
  Todo projeto Dioxus deve manter seu manifesto configurado com o identificador canônico `in.2lp.<app>` e metadados de empacotamento:
  ```toml
  [application]
  name = "<app>"
  default_platform = "web"

  [bundle]
  identifier = "in.2lp.<app>"
  publisher = "Lumen Pink"
  icon = ["assets/icon.png"]
  ```
* **Ergonomia Mobile e Layout Adaptativo:**
  - Safe Area Insets respeitando entalhes e barras gestuais (`env(safe-area-inset-top)` e `env(safe-area-inset-bottom)`).
  - Alvos de toque com `touch-action: manipulation` e dimensão mínima de **48px**.
  - Responsividade automática: navegação por barra inferior no celular (< 640px) e barra lateral compacta (*rail*) em tablets (640px – 1024px).
* **Componente de Rodapé Canônico:**
  Aplicações Dioxus devem embutir o componente reativo do rodapé oficial com tradução dinâmica para as 10 línguas normativas.

### 5.6 Web, Interfaces e Ponta a Ponta (E2E)
* **Runner E2E:** **`playwright`** (`npx playwright test`).
* **Escopo:** Navegação automatizada *headless* em Chromium, Firefox e WebKit simulando as jornadas reais do usuário e garantindo ausência de erros de console, layout e renderização.

### 5.7 Análise Estática, Formatação Determinística & Zero Warnings por Stack
O ecossistema adota um pipeline estrito onde **erros de linter e desvios de formatação quebram o build**:

* **Rust:**
  - Linter: `cargo clippy --all-targets --all-features -- -D warnings`
  - Formatação: `cargo fmt -- --check`
  - Auditoria de Dependências: `cargo audit`
* **PHP:**
  - Análise Estática: `vendor/bin/phpstan analyse --level=max`
  - Formatação: `vendor/bin/pint --test`
  - Auditoria de Dependências: `composer audit`
* **POSIX Shell:**
  - Linter: `shellcheck` (zero avisos tolerados em todos os scripts)
  - Formatação: `shfmt -d -i 2 -ci -bn .` (2 espaços, recuo de case, operadores no início)
* **Web, JS/TS, JSON, YAML e Markdown:**
  - Formatação e Linter: `npx biome check .` (canônico) ou `npx prettier --check "**/*.{json,yaml,md,js,ts,html,css}"`
* **Higiene Universal de Editores:**
  - Arquivo `.editorconfig` na raiz de todo repositório para impor LF, UTF-8, ausência de trailing whitespaces e indentação correta.

### 5.8 Gate de Pre-Push com `act` (CI Local Idêntico)
O ecossistema Kollontai usa **`act`** — executor local de Forgejo/GitHub Actions baseado em Docker — para rodar o job `test` do pipeline em um ambiente **bit-for-bit idêntico** ao do Forgejo antes de cada `git push`.

#### Ferramenta
* **`act`** ([`nektos/act`](https://github.com/nektos/act)): lê `.forgejo/workflows/ci.yml` e executa os jobs dentro dos mesmos containers Docker usados pelo Forgejo Actions.
* **Pré-requisito:** Docker em execução na máquina de desenvolvimento (sempre presente no ecossistema Kollontai).

#### Comportamento do Hook `pre-push`
```
git push
  └─► pre-push hook (instalado por cccp install)
        ├─ Verifica se act + Docker estão disponíveis
        │    └─ Não disponíveis: emite aviso, NÃO bloqueia push
        ├─ Executa: act -j test --no-pull
        │    └─ Saída completa → .git/act-logs/<data>_<hora>_<branch>_<sha-curto>.log
        ├─ Mostra: spinner vivo + step atual + tempo decorrido
        └─ Exit 0: push prossegue  |  Exit 1: push abortado + resumo + path do log
```

#### Formato dos Logs
```
.git/act-logs/
  2026-09-24_09-41_feat-42-rate-limit_a4b3c2d.log
  2026-09-24_08-17_main_cd62fd5.log
```
* Diretório `.git/act-logs/` é automaticamente ignorado pelo git.
* Consulta rápida ao log mais recente: `cccp act log`

#### UX do Gate no Terminal
```
⠸ [pre-push] Running CI gate locally...
  Job: test › cargo clippy -- -D warnings     [0:12]
  Log: .git/act-logs/2026-09-24_09-41_feat-42_a4b3c2d.log
```
Sucesso:
```
✅ CI gate passed (1m 34s) — pushing to origin/main
```
Falha:
```
❌ CI gate failed (0m 47s) — push aborted
   → Step failed: cargo test (3 failures)
   → Log: .git/act-logs/2026-09-24_09-41_feat-42_a4b3c2d.log
   → Bypass consciente: git push --no-verify
```

#### Bypass Explícito
```bash
git push --no-verify  # bypass consciente e documentado — não use por hábito
```

---

## 6. Pipeline de CI/CD (`.forgejo/workflows/ci.yml`)

O workflow no Forgejo Actions declara **obrigatoriamente** jobs separados: `test` (roda também localmente via `act`) e `deploy-site` (exclusivo do Forgejo, após push):

```yaml
name: "CI / CD Pipeline"

on:
  push:
    branches: [main, develop]
  pull_request:

jobs:
  test:
    name: "Run Test Suite & Enforce Coverage (>90%)"
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      # Passos da stack (ex: pest --coverage --min=90, cargo llvm-cov --fail-under 90, etc.)

  deploy-site:
    name: "Deploy Documentation Site"
    runs-on: ubuntu-latest
    needs: [test]
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - name: "Deploy to Cloudflare Workers"
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
        run: |
          if [ -n "$CLOUDFLARE_API_TOKEN" ]; then
            npx wrangler deploy
          fi
      - name: "Deploy to Codeberg Pages (Stateless Orphan Force Push)"
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          git config user.name "actions-user"
          git config user.email "actions@codeberg.org"
          WORKTREE_DIR=$(mktemp -d)
          git worktree add --orphan -b pages "$WORKTREE_DIR"
          rm -rf "${WORKTREE_DIR:?}"/*
          cp -r site/* "$WORKTREE_DIR/"
          cd "$WORKTREE_DIR"
          git add -A
          if git diff --staged --quiet; then
            echo "No site changes to deploy."
          else
            git commit -m "docs(site): deploy documentation snapshot [skip ci]"
            git push --force origin pages
          fi
          cd - >/dev/null
          git worktree remove "$WORKTREE_DIR" --force || true
```

---

## 7. Registro no Catálogo Central (`~/src/catalog.json`)

Todo repositório sob o perfil Kollontai deve ser registrado no arquivo central de descoberta `~/src/catalog.json`:

```json
{
  "name": "<nome-do-projeto>",
  "forge": "codeberg",
  "slug": "lumenpink/<nome-do-projeto>",
  "description": "<Descrição objetiva em uma linha>",
  "topics": ["palavra-chave-1", "palavra-chave-2"],
  "license": "CC0-1.0",
  "website": "https://<nome-do-projeto>.2lp.in",
  "is_upstream": false
}
```

Isso garante indexação pelo `cccp catalog`, ferramentas de busca local e dashboards do ecossistema.

---

## 8. Mapeamento do Ciclo DD-TDD no Perfil Kollontai

No dia a dia do ecossistema Kollontai, o ciclo DD-TDD opera integrado ao Codeberg Issues, branches dedicados e ao CLI `cccp`:

### 8.1 Passo a Passo Operacional:
1. **Abertura do Ticket:**
   - Criado via interface do Codeberg ou via CLI:
     ```bash
     tea issue create --title "Rate limiting no endpoint de autenticação"
     # Issue #42 criada
     ```
2. **Criação do Branch Dedicado:**
   - Nunca commitar na `main`:
     ```bash
     git checkout -b feat/42-rate-limit
     ```
3. **Doc in develop (Commit):**
   - Documentação prévia dos contratos e schemas:
     ```bash
     git commit -m "docs(auth): document rate limit headers and error payloads (ref #42)"
     ```
4. **Failing Test (Commit):**
   - Teste automatizado que falha comprovando a ausência da proteção:
     ```bash
     git commit -m "test(auth): add failing integration test for 61st request (ref #42)"
     ```
5. **Fix / Code (Commit):**
   - Código que faz o teste passar com linter limpo (Zero Warnings):
     ```bash
     git commit -m "feat(auth): implement token bucket rate limiter (ref #42)"
     ```
6. **Finish Docs & Close (Commit):**
   - Atualização da documentação no `./site`, `CHANGELOG.md` e fechamento formal:
     ```bash
     git commit -m "docs(auth): update security guide and finalize tickets (closes #42)"
     ```

### 8.2 Gestão de Releases e Publicação Semântica
Quando um conjunto de tickets/milestones atinge maturidade:
```bash
cccp release minor   # ou patch / major
git push --tags origin main
```
Isso consolida o `CHANGELOG.md`, atualiza `VERSION` para versão estável e cria a tag Git correspondente.

### 8.3 Comandos Operacionais para Agentes de IA no Perfil Kollontai
No dia a dia com agentes de IA (assistentes CLI/IDE), o perfil Kollontai adota o protocolo canônico de comandos curtos:
* **`DCPD` (Do, Commit, Push, Deploy):** Autorização expressa para implementação da demanda, validação local com testes e Zero Warnings, commit no padrão Conventional Commits com fechamento de issue (`closes #ID`), push da branch do ticket, merge na branch principal (`main`) e push remoto disparando o deploy contínuo no Cloudflare Workers / R2 e Codeberg Pages (via stateless single-commit force push).
* **`DCP` (Do, Commit, Push):** Implementação, testes locais e push restritos à branch de trabalho (`feat/<id>-*`), gerando a URL do Pull Request no Codeberg para revisão humana prévia.
* **`PLAN` (Plan & Propose):** Pesquisa e diagnóstico aprofundado sem qualquer alteração em arquivos de código nem commits.
* **`TDD` (Ticket, Doc, Test, Do):** Execução estrita das 5 etapas do ciclo GOST com commits separados para documentação (`docs: ... ref #ID`), teste quebrado (`test: ... ref #ID`) e código funcional.
* **`AUDIT` (Audit & Check):** Auditoria técnica passiva rodando linters no máximo (`-D warnings`), cobertura (`llvm-cov` / `pest --coverage`), testes de mutação (`cargo-mutants` / `infection`) e `composer/cargo audit`.
* **`FIX` (Fix & Clean):** Saneamento direcionado para eliminar alertas de análise estática e corrigir formatação com `cargo fmt`, `pint` ou `shfmt`.
* **`RELEASE` (Release & Tag):** Execução automatizada de `cccp release <patch|minor|major>` e sincronização com tags remotas.

---

## 9. Internacionalização no Perfil Kollontai

O ecossistema Kollontai adota o padrão de arquitetura multilíngue validado no `indieinabox` com governança de idioma centralizada pelo `cccp`:

### 9.1 Configuração da Língua Primária (`.cccprc`)
A autoridade canônica do idioma padrão de cada repositório é configurada no `.cccprc`:
```ini
agent_var_lang = english   # ou portuguese, etc.
```
- Serve de baseline para o ciclo DD-TDD e triagem de tickets via `diva`.
- Define o idioma de fallback automático do aplicativo caso alguma chave de tradução em outro idioma não esteja presente.

### 9.2 Paridade das 10 Línguas no Aplicativo (`resources/locales/` ou `locales/`)
Todo software deve manter dicionários estruturados para as 10 línguas oficiais:
`en.json`, `pt-BR.json`, `es.json`, `fr.json`, `de.json`, `it.json`, `ru.json`, `zh.json`, `ja.json`, `ar.json`.
A suíte de testes de integração (ex: Pest, Cargo, ShellSpec) deve validar a paridade de 100% de chaves em relação ao arquivo de baseline.

### 9.3 Roteador Estático Cliente (`site/index.html`)
Na raiz de documentação `./site/index.html`, inclui-se o script de redirecionamento automático com detecção e persistência:
```html
<script>
    (function() {
        var supported = ['en', 'pt', 'es', 'fr', 'de', 'it', 'ru', 'zh', 'ja', 'ar'];
        var saved = localStorage.getItem('doc_lang');
        if (saved && supported.indexOf(saved) !== -1) {
            window.location.replace(saved + '/');
            return;
        }
        var navLang = (navigator.language || navigator.userLanguage || 'en').toLowerCase().substring(0, 2);
        if (supported.indexOf(navLang) !== -1) {
            window.location.replace(navLang + '/');
        } else {
            window.location.replace('en/');
        }
    })();
</script>
```

### 9.4 Suporte Obrigatório a RTL para o Árabe (`site/ar/`)
As páginas sob `site/ar/` e as interfaces da aplicação utilizam:
```html
<html lang="ar" dir="rtl">
```

---

## 10. Banners e Cards Sociais (Padrão KokoroSim)

Todo repositório Lumen que publique site estático deve manter a automação para geração do banner Open Graph em `./site/assets/og_preview.png`.

### 10.1 Script de Geração (`scripts/generate_og_image.py`)
Utiliza Python + Pillow para renderizar o banner em 1200x630 com Safe Zone 4:3 centralizada (largura de 720px a 840px), evitando cortes em pré-visualizações móveis e do WhatsApp:
- **Resolução:** 1200x630 px
- **Safe Zone:** Miolo central com sangria lateral mínima de 180px.
- **Tipografia:** Fonte condensada geométrica para títulos de alto impacto.

### 10.2 Preview de Cards (`site/card_preview.html`)
Para validação visual fiel antes do deploy, recomenda-se incluir em `site/card_preview.html` a simulação de renderização em WhatsApp (Dark Mode) e Twitter/X (Summary Large Image), conforme arquitetura do KokoroSim.

---

## 11. Arquitetura Pragmática & Estrutura de Código no Perfil Kollontai

O perfil Kollontai materializa os princípios de Inversão de Dependência (DIP), hermeticidade e anti-bloat através de padrões objetivos adaptados às linguagens centrais do ecossistema (**Rust** e **PHP**).

### 11.1 Regra de Gradação: Micro-ferramenta vs. Aplicação
1. **Micro-ferramentas e Scripts Utilitários:** Ferramentas de escopo pontual (ex: CLI de 100 linhas ou filtro de texto) **NÃO DEVEM** ser fracionadas em múltiplas pastas cerimoniais. Ficam contidas diretamente em `src/main.rs` (Rust) ou arquivo executável único (PHP/Bash).
2. **Aplicações e Serviços:** Projetos que envolvam persistência, múltiplos protocolos ou casos de uso complexos **DEVEM** adotar a divisão canônica entre Domínio e Adaptadores.

### 11.2 Padrão de Diretórios para Rust
Em sistemas, daemons e milters:

```text
src/
├── domain/             # Regras de negócio puras (sem dependências de I/O)
│   ├── mod.rs
│   ├── entity.rs       # Modelos e invariantes de domínio
│   ├── status.rs       # Enums nativos para máquinas de estado
│   └── ports.rs        # Traits de I/O (Output Ports: ex: MessageStore, ClientApi)
├── application/        # Orquestração e Casos de Uso
│   ├── mod.rs
│   └── use_cases.rs    # Lógica que opera sobre os traits de domínio
├── adapters/           # Implementações de I/O e interfaces de transporte
│   ├── mod.rs
│   ├── in_memory.rs    # Fake em memória para testes unitários herméticos
│   ├── sqlite.rs       # Adaptador real de persistência
│   └── cli.rs          # Entrada via linha de comando (ex: clap)
└── main.rs             # Composition Root: instancia adaptadores e orquestra o bootstrap
```

#### Exemplo Canônico de Fake em Memória (Rust):
```rust
// src/domain/ports.rs
pub trait MessageStore {
    fn save(&mut self, id: &str, content: &str) -> Result<(), String>;
    fn find(&self, id: &str) -> Option<String>;
}

// src/adapters/in_memory.rs
use std::collections::HashMap;
use crate::domain::ports::MessageStore;

#[derive(Default)]
pub struct InMemoryMessageStore {
    items: HashMap<String, String>,
}

impl MessageStore for InMemoryMessageStore {
    fn save(&mut self, id: &str, content: &str) -> Result<(), String> {
        self.items.insert(id.to_string(), content.to_string());
        Ok(())
    }
    fn find(&self, id: &str) -> Option<String> {
        self.items.get(id).cloned()
    }
}
```
*Zero mocks dinâmicos:* Os testes unitários do caso de uso instanciam `InMemoryMessageStore` diretamente, rodando em microssegundos sem rede nem arquivos.

### 11.3 Padrão de Diretórios para PHP
Em APIs, aplicações IndieWeb e serviços web:

```text
src/
├── Domain/             # Núcleo de domínio (PHP puro, zero dependências externas)
│   ├── Models/         # Entidades de negócio
│   ├── Enums/          # Backed Enums nativos (estados e categorias)
│   └── Ports/          # Contratos e Interfaces (Input/Output Ports)
├── Application/        # Casos de uso e orquestração
│   └── UseCases/       # Handlers executando fluxos de domínio
└── Infrastructure/     # Adaptadores concretos
    ├── Persistence/    # Implementações reais (SQLite/Pdo) e In-Memory Fakes
    ├── Http/           # Controllers e Middlewares
    └── Console/        # Comandos CLI
tests/
├── Unit/               # Testes herméticos usando In-Memory Fakes (milissegundos)
└── Integration/        # Testes com fixtures de banco SQLite em arquivo temporário
```

#### Exemplo Canônico de Fake em Memória e Pest (PHP):
```php
// src/Domain/Ports/TokenStorageInterface.php
namespace App\Domain\Ports;

interface TokenStorageInterface {
    public function store(string $token, string $userId): void;
    public function find(string $token): ?string;
}

// src/Infrastructure/Persistence/InMemoryTokenStorage.php
namespace App\Infrastructure\Persistence;

use App\Domain\Ports\TokenStorageInterface;

final class InMemoryTokenStorage implements TokenStorageInterface {
    /** @var array<string, string> */
    private array $tokens = [];

    public function store(string $token, string $userId): void {
        $this->tokens[$token] = $userId;
    }

    public function find(string $token): ?string {
        return $this->tokens[$token] ?? null;
    }
}
```
No teste Pest (`tests/Unit/AuthenticateUserTest.php`):
```php
test('autentica usuário com storage em memória', function () {
    $storage = new \App\Infrastructure\Persistence\InMemoryTokenStorage();
    $storage->store('tok-123', 'usr-456');

    expect($storage->find('tok-123'))->toBe('usr-456')
        ->and($storage->find('inexistente'))->toBeNull();
});
```

### 11.4 Filosofia Anti-Bloat: Tipos Primitivos & Validação na Borda
* **Parse, Don't Validate na Entrada:** A validação e desinfecção de strings, limites numéricos e decodificação JSON ocorrem no adaptador de entrada (ex: validação de request HTTP ou parser do CLI).
* **Primitivos Confiáveis no Domínio:** Evita-se criar Value Objects cerimoniais (ex: `UsernameString`, `IdInt`) quando tipos primitivos nativos (`string`, `int`) com tipagem estrita da linguagem atendem com clareza.
* **Modelagem com Enums Nativos:**
  - Rust: `pub enum TaskState { Pending, Running, Finished { exit_code: i32 } }`
  - PHP: `enum OrderStatus: string { case Pending = 'pending'; case Approved = 'approved'; case Cancelled = 'cancelled'; }`

---

## 12. Padrões de Sistema e Containers no Perfil Kollontai

O ecossistema Kollontai adota padrões rígidos de interoperabilidade Unix, priorizando ferramentas leves e compatíveis com shell scripting e ambientes conteinerizados seguros.

### 12.1 Resolução Canônica de Diretórios XDG
Todo utilitário ou serviço Lumen implementa o mapeamento estrito da especificação XDG com fallbacks universais.

#### Em Shell Scripting (POSIX):
```sh
APP_NAME="meu-app"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/$APP_NAME"
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/$APP_NAME"
STATE_DIR="${XDG_STATE_HOME:-$HOME/.local/state}/$APP_NAME"
CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/$APP_NAME"
RUNTIME_DIR="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}/$APP_NAME"

mkdir -p "$CONFIG_DIR" "$DATA_DIR" "$STATE_DIR" "$CACHE_DIR" 2>/dev/null
```

#### Em Rust:
```rust
pub fn config_path(app: &str) -> std::path::PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home).join(".config")
        });
    base.join(app)
}
```

### 12.2 Configuração Plana Baseada em Shell e Env (`KEY=VALOR`)
Para eliminar dependências pesadas de parsing (como bibliotecas TOML/YAML) em scripts de automação, os arquivos de configuração locais e de usuário adotam formato plano compatível com shell:

```bash
# $XDG_CONFIG_HOME/meu-app/config.env
MATRACA_PORT="8080"
MATRACA_BIND="127.0.0.1"
MATRACA_LOG_LEVEL="info"
```

#### Carregamento Nativo em Shell:
```sh
if [ -f "$CONFIG_FILE" ]; then
    set -a
    # shellcheck source=/dev/null
    . "$CONFIG_FILE"
    set +a
fi
```

### 12.3 Dockerfiles Canônicos no Perfil Kollontai

#### Template Rust (Multi-stage + `scratch` estático):
```dockerfile
# Estágio de compilação
FROM rust:1.99-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# Imagem final de produção (Zero Inchaço)
FROM scratch
USER 1000:1000
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]
```

#### Template PHP (Multi-stage + `alpine` com Non-root):
```dockerfile
# Estágio de dependências
FROM composer:2 AS vendor
WORKDIR /app
COPY composer.json composer.lock ./
RUN composer install --no-dev --no-interaction --prefer-dist --optimize-autoloader

# Imagem final de execução
FROM php:8.3-cli-alpine
RUN adduser -D -u 1000 appuser
WORKDIR /app
COPY --chown=appuser:appuser . /app
COPY --chown=appuser:appuser --from=vendor /app/vendor /app/vendor
USER 1000:1000
ENTRYPOINT ["php", "/app/bin/console"]
```

### 12.4 Higiene de Containers (`.dockerignore` e `compose.yaml`)
Todo repositório com suporte a Docker **DEVE** incluir `.dockerignore` rigoroso na raiz:
```dockerignore
.git
.env*
!*.example
target/
vendor/
tests/
site/
*.log
```

E para orquestração de desenvolvimento local, adotar `compose.yaml`:
```yaml
services:
  app:
    build: .
    user: "1000:1000"
    env_file:
      - .env
    ports:
      - "${APP_PORT:-8080}:8080"
    restart: unless-stopped
```

---

## 13. Rodapé Canônico Multilíngue (Padrão de Autoria e Soberania)

Todo aplicativo com interface gráfica, vitrine pública, documentação (`site/`) ou CLI com rodapé formatado no ecossistema Kollontai **DEVE** exibir o rodapé canônico em **duas linhas rigorosamente estruturadas**, com paridade de tradução nas **10 Línguas Oficiais da Norma GOST**.

### 13.1 Arquitetura do Rodapé em Duas Linhas

* **Linha 1 — Afirmação Identitária & Transfeminista:**  
  `Orgulhosamente produzido por uma [travesti latina] 🏳️‍⚧️ :)`  
  - O substantivo **"travesti"** permanece rigorosamente **sem tradução** em todos os idiomas, preservando sua singularidade geopolítica e conceitual sul-americana.
  - O adjetivo **"latina"** é adaptado à gramática nativa de cada idioma.
  - A expressão composta `travesti [latina]` **DEVE** atuar como hiperlink obrigatório direcionando para o artigo canônico de definição em `https://travesti.2lp.in/<lang>/`.
  - A bandeira trans (`🏳️‍⚧️`) sucede a expressão antes do sorriso `:)`.

* **Linha 2 — Metadados de Soberania, Autoria e Licenciamento:**  
  `{Nome do App} v{Versão} • [Lumen Pink](https://lumen.pink) • {Data/Hora ISO UTC} • {País / Brasil} 🇧🇷 • [{Licença}]`  
  - `{Nome do App}`: Identificador público da aplicação ou biblioteca.
  - `v{Versão}`: Versão extraída diretamente do arquivo `VERSION` da raiz do repositório (ex: `v0.1.0-dev.1` ou `v1.0.0`).
  - `Lumen Pink`: Com link canônico para `https://lumen.pink`.
  - `{Data/Hora ISO UTC}`: Carimbo exato de data e hora da compilação/montagem no formato ISO 8601 UTC (ex: `2026-09-28T22:20:00Z`), no lugar do ano estático.
  - `{País / Brasil} 🇧🇷`: Nome do país adaptado ao idioma (ex: `Brasil 🇧🇷`, `Brazil 🇧🇷`, `Brésil 🇧🇷`, `Brasilien 🇧🇷`, `Brasile 🇧🇷`, `Бразилия 🇧🇷`, `巴西 🇧🇷`, `ブラジル 🇧🇷`, `البرازيل 🇧🇷`), acompanhado da bandeira nacional brasileira.
  - `[{Licença}]`: Identificador SPDX da licença (ex: `GPL-3.0-or-later`, `CC-BY-SA-4.0`, etc.) com link para o arquivo `LICENSE` do repositório.

### 13.2 Matriz Canônica das 10 Línguas Oficiais (Rodapé)

| Idioma | ISO | Linha 1 (Afirmação Identitária) | Linha 2 (Metadados de Soberania) |
|:---:|:---:|---|---|
| **Português** | `pt` | Orgulhosamente produzido por uma [travesti latina](https://travesti.2lp.in/pt/) 🏳️‍⚧️ :) | {App} v{Versão} • por [Lumen Pink](https://lumen.pink) • {ISO_UTC} • Brasil 🇧🇷 • [{Licença}] |
| **Inglês** | `en` | Proudly produced by a [Latin American travesti](https://travesti.2lp.in/en/) 🏳️‍⚧️ :) | {App} v{Versão} • by [Lumen Pink](https://lumen.pink) • {ISO_UTC} • Brazil 🇧🇷 • [{Licença}] |
| **Espanhol** | `es` | Orgullosamente producido por una [travesti latinoamericana](https://travesti.2lp.in/es/) 🏳️‍⚧️ :) | {App} v{Versão} • por [Lumen Pink](https://lumen.pink) • {ISO_UTC} • Brasil 🇧🇷 • [{Licença}] |
| **Francês** | `fr` | Fièrement produit par une [travesti latino-américaine](https://travesti.2lp.in/fr/) 🏳️‍⚧️ :) | {App} v{Versão} • par [Lumen Pink](https://lumen.pink) • {ISO_UTC} • Brésil 🇧🇷 • [{Licença}] |
| **Alemão** | `de` | Mit Stolz entwickelt von einer [lateinamerikanischen Travesti](https://travesti.2lp.in/de/) 🏳️‍⚧️ :) | {App} v{Versão} • von [Lumen Pink](https://lumen.pink) • {ISO_UTC} • Brasilien 🇧🇷 • [{Licença}] |
| **Italiano** | `it` | Orgogliosamente prodotto da una [travesti latinoamericana](https://travesti.2lp.in/it/) 🏳️‍⚧️ :) | {App} v{Versão} • da [Lumen Pink](https://lumen.pink) • {ISO_UTC} • Brasile 🇧🇷 • [{Licença}] |
| **Russo** | `ru` | С гордостью создано [латиноамериканской travesti](https://travesti.2lp.in/ru/) 🏳️‍⚧️ :) | {App} v{Versão} • от [Lumen Pink](https://lumen.pink) • {ISO_UTC} • Бразилия 🇧🇷 • [{Licença}] |
| **Chinês** | `zh` | 由一位[拉美 travesti](https://travesti.2lp.in/zh/) 自豪制作 🏳️‍⚧️ :) | {App} v{Versão} • 来自 [Lumen Pink](https://lumen.pink) • {ISO_UTC} • 巴西 🇧🇷 • [{Licença}] |
| **Japonês** | `ja` | [ラテンアメリカの travesti](https://travesti.2lp.in/ja/) によって誇りを持って制作されました 🏳️‍⚧️ :) | {App} v{Versão} • 制作: [Lumen Pink](https://lumen.pink) • {ISO_UTC} • ブラジル 🇧🇷 • [{Licença}] |
| **Árabe (RTL)** | `ar` | صُنع بكل فخر بواسطة [travesti أمريكية لاتينية](https://travesti.2lp.in/ar/) 🏳️‍⚧️ :) | {App} v{Versão} • بواسطة [Lumen Pink](https://lumen.pink) • {ISO_UTC} • البرازيل 🇧🇷 • [{Licença}] |

---

## 14. Atualização Contínua de Containers Docker (Watchtower)

Para garantir segurança, integridade e atualização contínua de imagens de containers em infraestruturas Docker / Docker Compose autohospedadas (como nos nós `obione`, `obitwo` e `baquara`), a automação adota o **Watchtower** sob as seguintes diretrizes obrigatórias:

1. **Opt-in Rigoroso por Labels (`--label-enable`):**  
   O Watchtower **NUNCA** monitora ou reinicia containers de modo indiscriminado. Apenas containers declarando explicitamente a label:
   ```yaml
   labels:
     - "com.centurylinklabs.watchtower.enable=true"
   ```
   serão atualizados. Containers com dependências sensíveis de migração de banco de dados ou volumes críticos sem backup permanecem intocados.
2. **Limpeza Compulsória de Imagens Órfãs (`WATCHTOWER_CLEANUP=true`):**  
   Imagens antigas substituídas são purgadas imediatamente para evitar exaustão de armazenamento no host.
3. **Modos de Acionamento Homologados:**
   - **Agendamento Periódico:** Polling programado com intervalo seguro (ex: diário ou a cada 2h).
   - **Acionamento por Webhook (`WATCHTOWER_HTTP_API_TOKEN`):** Disparo autenticado via POST HTTP disparado automaticamente pelo pipeline de CI/CD do Codeberg após a publicação da nova imagem no registry.

---

<!-- ===================================================================== -->
<!-- CAMADA 3: https://gostlumen.2lp.in/profile.md -->
<!-- ===================================================================== -->

# Perfil de Implementação GOST: Lumen

```yaml
---
id: lumen
name: "Perfil de Implementação Lumen"
extends: "https://kollontai.2lp.in/profile.md"
target_gost: "1.5.0"
maintainer: "Lumen Pink <hi@lumen.pink>"
canonical_url: "https://gostlumen.2lp.in/profile.md"
---
```

> **Sobre este Perfil:**  
> O perfil **Lumen** é a especialização pessoal e autoral do padrão **[GOST](https://codeberg.org/lumenpink/gost)**.  
> Ele estende diretamente o perfil base **[Kollontai](https://kollontai.2lp.in)** (`extends: https://kollontai.2lp.in/profile.md`), herdando integralmente todas as suas decisões de infraestrutura (Codeberg, `cccp`, deploy em `*.2lp.in`, esteira de CI/CD, runners OCI e testes herméticos), aplicando regras de precedência soberana (*deep override*) para estilização, identidade, Open Graph e ergonomia de documentação.

---

## 1. Identidade e Herança (*Cascading Profile*)

* **Perfil Pai:** `https://kollontai.2lp.in/profile.md` (Kollontai)
* **Precedência:** Qualquer regra explicitada neste documento tem precedência estrita sobre o perfil pai. Todas as regras não mencionadas aqui são herdadas automaticamente do Kollontai e da norma central GOST.
* **Domínio Canônico:** `https://gostlumen.2lp.in`
* **Forja Primária Soberana:** Forgejo em `git.2lp.in` (`https://git.2lp.in/lumen/<repo>`)
* **Remote Git SSH Padrão:** `git@git.2lp.in:lumen/<repo>.git` (Porta 22 padrão via SSH shim do host).
* **Política de Push Mirrors Duplos (Vitrine Aberta):**
  - Todo repositório público hospedado no `git.2lp.in` opera com **Push Mirror** automatizado imediato a cada commit (`sync_on_commit: true`) para:
    1. **Espelho FOSS:** Codeberg (`https://codeberg.org/lumenpink/<repo>`)
    2. **Espelho Comunitário:** GitHub (`https://github.com/lumenpink/<repo>`)
  - O desenvolvedor realiza commits e pushes locais exclusivamente para o `origin` (`git@git.2lp.in:lumen/<repo>.git`). A replicação para os espelhos externos é responsabilidade da forja soberana.

---

## 2. Exibição Estrita de Versões Semânticas

Para garantir legibilidade instantânea em cabeçalhos de tela sem abrir mão do rastreamento rigoroso de builds de desenvolvimento, projetos sob o perfil Lumen **DEVEM** aderir à seguinte regra de apresentação de versões:

1. **Título, Badges e Cabeçalhos (Versão Short):**
   - **Formato:** `v{MAJOR}.{MINOR}.{PATCH}`
   - **Regra de Truncamento:** A versão exibida em tags visuais, badges e títulos principais **DEVE** ser truncada antes de qualquer metadado de build (cortada imediatamente antes do caractere `+` ou preservada limpa se tag de release final).
   - *Exemplo:* Se o arquivo `VERSION` contiver `0.1.0+20261013`, o título e badges exibem estritamente `v0.1.0`.

2. **Rodapé Canônico (Versão Long / Completa):**
   - **Formato:** String integral idêntica ao conteúdo do arquivo `VERSION` na raiz do repositório.
   - *Exemplo:* `Joaju v0.1.0+20261013 • por Lumen Pink • Brasil 🇧🇷 • CC0-1.0`

---

## 3. Rodapé Canônico em Duas Linhas (Afirmação Identitária & Soberania)

Todo projeto, interface gráfica, vitrine pública ou documentação estática (`./site`) **DEVE** exibir o rodapé canônico formatado em **duas linhas rigorosamente estruturadas**:

```html
<footer class="footer">
  <p>Orgulhosamente produzido por uma <a href="https://travesti.2lp.in/pt/" target="_blank" rel="noopener">travesti latina</a> 🏳️‍⚧️ :)</p>
  <p>{Nome do Projeto} v{Versão Completa} • por <a href="https://lumen.pink" target="_blank" rel="noopener">Lumen Pink</a> • Brasil 🇧🇷 • <a href="LICENSE">LICENSE</a></p>
</footer>
```

### Regras do Rodapé:
* **Linha 1 — Afirmação Identitária Transfeminista:**  
  `Orgulhosamente produzido por uma [travesti latina](https://travesti.2lp.in/<lang>/) 🏳️‍⚧️ :)`  
  - O termo *travesti* é mantido sem tradução em todas as versões idiomáticas (PT, EN, ES, RU, etc.).
  - O adjetivo concorda gramaticalmente com a língua de exibição e inclui link com o código de idioma ativo (`/pt/`, `/en/`, etc.).
* **Linha 2 — Metadados de Soberania, Autoria e Licença:**  
  `{Nome do Projeto} v{Versão Completa} • por [Lumen Pink](https://lumen.pink) • Brasil 🇧🇷 • [{Licença}](LICENSE)`  
  - Exibe a versão integral de `VERSION`, o link autoral e o link relativo para o arquivo de licença no repositório.

---

## 4. Banners, Open Graph & Cards Sociais (Padrão KokoroSim)

O compartilhamento de links de projetos em redes sociais e mensageiros (WhatsApp, Telegram, Discord, Twitter/X, LinkedIn) **DEVE** aderir à arquitetura visual e técnica anti-recorte KokoroSim:

### 4.1 Especificações da Imagem Open Graph (`./site/assets/og_preview.png`)
* **Resolução Obrigatória:** Exatamente `1200 × 630 pixels` (proporção 1.91:1, formato PNG otimizado, espaço de cores sRGB).
* **Safe Zone 4:3 Centralizada (Anti-Recorte):**
  - **Largura Central Útil:** Entre `720px` e `840px`.
  - **Altura Central Útil:** `500px`.
  - **Sangrias Laterais de Segurança:** Mínimo de `180px` a `240px` em cada lateral (reservadas exclusivamente a fundos, gradientes ou texturas decorativas, sem texto).
  - **Margens Verticais de Segurança:** Top padding mínimo de `52px`; bottom padding mínimo de `85px` (evita sobreposição por carimbos de hora ou balões de chat).

### 4.2 Limites Rigorosos de Caracteres (Anti-Truncamento)
* **`<meta property="og:title">` / `twitter:title`:** Limite seguro entre **50 e 65 caracteres** (teto máximo absoluto: 70 caracteres).
* **`<meta property="og:description">` / `twitter:description`:** Limite seguro entre **120 e 155 caracteres** (teto máximo absoluto: 160 caracteres).
* **Badge Superior Impresso no Banner:** Máximo de **35 caracteres** em caixa alta.
* **Título Principal no Banner:** Máximo de **35 a 45 caracteres** em tipografia bold condensada.
* **Subtítulo no Banner:** Máximo de **50 a 60 caracteres** em linha única.

### 4.3 Suporte Obrigatório a Dark Mode
* Todas as páginas e componentes web **DEVEM** prover suporte nativo de primeira classe a Dark Mode:
  - Definido via tokens CSS com variáveis nativas (`--bg`, `--text`, `--accent`, `--border`).
  - Respeito preferencial à diretiva de mídia `@media (prefers-color-scheme: dark)` por padrão.
  - Alternância amigável ou padrão escuro com paleta de alto contraste sem fadiga visual.
* **Página de Teste de Cards (`site/card_preview.html`):**
  - Todo site com vitrine social deve disponibilizar `site/card_preview.html` contendo simulador interativo dos cards em WhatsApp (Dark Mode) e Twitter/X (Summary Large Image).

---

## 5. Publicação Web na Borda (`gostlumen.2lp.in`)

* **Domínio Principal:** `https://gostlumen.2lp.in`
* **Hospedagem:** Cloudflare Pages / Workers através de `wrangler.jsonc` com assets estáticos servidos em `./site`.
* **Redundância:** Deploy em Codeberg Pages via branch órfã `pages` com commit stateless único e force push.

---

## 6. Governança de Documentação & Vitrine (`site_type`)

* **Tipologia Canônica:**
  - **`site_type: doc`:** Vitrines de bibliotecas, perfis e daemons (onde `site/index.html` exibe a documentação diretamente).
  - **`site_type: app`:** Aplicações web completas (como Navalha, Joaju), onde `site/index.html` carrega a SPA/WASM e toda documentação e help do usuário reside sob `site/doc/`.
* **Single Source of Truth para Documentação:**
  - Todo texto documental e tutorial vive estritamente em Markdown. É proibido duplicar textos manualmente em arquivos HTML estáticos.
  - O `README.md` da raiz atua como portal enxuto (até 50 linhas), delegando aprofundamentos para a vitrine oficial `*.2lp.in`.

---

## 7. Protocolo Soberano de Atualização e Conformidade OpenDesktop (XDG Base Directory)

Todas as aplicações de usuário final (CLI e desktop) desenvolvidas sob o perfil Lumen DEVEM incorporar o protocolo de atualização soberano e gerenciamento de configurações em conformidade com as especificações da freedesktop.org (OpenDesktop).

### 7.1 Resolução Canônica de Caminhos XDG

1. **Arquivo de Configuração do Usuário:**
   - Variável de ambiente: `$XDG_CONFIG_HOME`
   - Resolução: `${XDG_CONFIG_HOME:-~/.config}/<app>/config.toml`
   - Fallback implícito: `~/.config/<app>/config.toml`
   - Garantia de escrita atômica com permissões seguras (`0755` ou `0700` para diretórios).

2. **Arquivo de Estado de Runtime e Cache de Checagem:**
   - Variável de ambiente: `$XDG_STATE_HOME`
   - Resolução: `${XDG_STATE_HOME:-~/.local/state}/<app>/update_check.json`
   - Fallback implícito: `~/.local/state/<app>/update_check.json`
   - Isolamento total entre dados de runtime em cache e preferências editadas pelo usuário.

### 7.2 Esquema Declarativo de Configuração (`config.toml`)

O arquivo de configuração do aplicativo DEVE suportar a seção `[update]`:

```toml
[update]
# Canal de distribuição ativo: "stable" ou "nightly" (padrão: "stable")
channel = "stable"

# Trava de versão (pinning): se true, ignora avisos de atualização e bloqueia update automático (padrão: false)
pinned = false

# Versão específica fixada quando pinned = true (ex: "0.5.0")
# Se vazio e pinned = true, assume a versão atualmente instalada
pin = ""

# Verificação periódica automática de novas versões em segundo plano ao executar a CLI (padrão: true)
auto_check = true

# Intervalo mínimo em horas entre consultas remotas de rede (padrão: 24 horas)
check_interval_hours = 24

# Auto-atualização transparente em segundo plano para a próxima execução (padrão: false)
auto_update = false
```

#### Precedência de Resolução de Canal:
`--channel <flag>` > variável de ambiente `<APP>_CHANNEL` > `config.toml` (`update.channel`) > `"stable"`

### 7.3 Esquema de Cache de Estado (`update_check.json`)

Armazenado em `$XDG_STATE_HOME/<app>/update_check.json`:

```json
{
  "last_check_timestamp": 1729411200,
  "latest_remote_version": "0.5.1",
  "channel": "stable"
}
```

### 7.4 Protocolo de Checagem Periódica e Alerta em `stderr`

Ao executar qualquer comando padrão da CLI ou ao abrir a aplicação desktop:

1. **Short-Circuit:** Se `update.auto_check == false` ou `update.pinned == true`, a checagem é desativada sem qualquer consulta de rede.
2. **Fast Cache Check (Aviso Imediato):**
   - Caso `update_check.json` indique que `latest_remote_version > current_version` (conforme SemVer 2.0.0):
   - A aplicação DEVE imprimir uma notificação sutil exclusivamente no canal **`stderr`** (preservando pipes e redirecionamentos `stdout`):
     ```text
     💡 [<app>] Nova versão disponível: vX.Y.Z (atual: vA.B.C).
        Execute '<app> update' para atualizar.
     ```
3. **Disparo de Checagem Remota Não-Bloqueante:**
   - Se `(agora - last_check_timestamp) >= check_interval_hours * 3600`:
   - Uma consulta HTTP ao endpoint `https://banca.2lp.in/downloads/<app>/<channel>-latest.version` DEVE ser executada de forma desacoplada ou com timeout síncrono ultra-rápido ($\le 1$ segundo).
   - Se bem-sucedida, atualiza `update_check.json`.
   - Se falhar por timeout ou indisponibilidade de rede, a falha DEVE ser descartada silenciosamente, apenas postergando o `last_check_timestamp` para a próxima janela.

### 7.5 Política de Travamento de Versão (Pinning)

1. Quando `pinned = true` ou `pin` contém uma versão fixada:
   - `<app> check-update` informa: `📌 Versão pinada em v<ver>. Atualizações automáticas ignoradas.`
   - `<app> update` bloqueia a substituição a menos que a flag `--force` seja explicitamente passada.
   - O aviso periódico no arranque permanece em silêncio.

### 7.6 Contrato Universal de CLI

Todo aplicativo sob o perfil Lumen DEVE fornecer os seguintes subcomandos:

- `<app> update [--channel <c>] [--pin <v>] [--force] [--dry-run]`
- `<app> check-update [--channel <c>]`
- `<app> config show` / `<app> config get`
- `<app> config set-channel <stable|nightly>`
- `<app> config pin [<version>]`
- `<app> config unpin`
- `<app> config set-auto-check <true|false>`
- `<app> config set-auto-update <true|false>`

### 7.7 Mecanismo de Auto-Atualização Transparente (auto_update)

1. **Consentimento Estrito:** Desabilitado por padrão (`auto_update = false`). Nenhuma atualização automática ocorre sem ativação explícita.
2. **Salvaguarda de Pinning:** Caso `pinned = true` ou se houver versão fixada em `pin`, qualquer rotina de auto-update é sumariamente ignorada.
3. **Substituição Atômica Não-Destrutiva:** O download e a substituição do binário ocorrem de forma atômica no sistema de arquivos para a próxima inicialização, mantendo a integridade total do processo em execução na RAM.
4. **Notificação Informativa em `stderr`:**
   ```text
   ✨ [<app>] Atualização automática aplicada: vX.Y.Z -> vA.B.C.
      A nova versão estará ativa na sua próxima execução.
   ```

---

## 8. Arquitetura de Distribuição Soberana na Banca (`banca.2lp.in`)

Para garantir soberania tecnológica, imunidade a bloqueios corporativos e eliminação total de binários pesados no Git e no Codeberg, todo artefato compilado destinado ao usuário final DEVE ser publicado e gerido exclusivamente através da **Banca**.

### 8.1 Topologia de Endpoints e Convenção de Nomes

A estrutura canônica de rotas servida pela vitrine pública (`https://banca.2lp.in/`) e pelo bucket Cloudflare R2 DEVE seguir a seguinte hierarquia por aplicativo (`<app>`):

| Recurso / Endpoint | Descrição e Finalidade |
| :--- | :--- |
| `https://banca.2lp.in/downloads/` | Vitrine interativa de downloads com seletor de apps, deep linking e checksums. |
| `https://banca.2lp.in/downloads/<app>/install.sh` | Script instalador universal autônomo (POSIX sh) executável via curl/pipe. |
| `https://banca.2lp.in/downloads/<app>/stable-latest` | Ponteiro/alias atômico para o binário mais recente do canal estável. |
| `https://banca.2lp.in/downloads/<app>/nightly-latest` | Ponteiro/alias atômico para o binário mais recente de desenvolvimento contínuo. |
| `https://banca.2lp.in/downloads/<app>/stable-latest.version` | Arquivo de texto puro contendo estritamente a versão estável atual (ex: `0.5.0\n`). |
| `https://banca.2lp.in/downloads/<app>/nightly-latest.version` | Arquivo de texto puro contendo a versão nightly atual (ex: `0.5.0-dev.216+20261020.ba9aaf0\n`). |
| `https://banca.2lp.in/downloads/<app>/index.json` | Manifesto atômico estruturado detalhando versões ativas, hashes SHA-256 e timestamps. |
| `https://banca.2lp.in/downloads/<app>/<app>-<version>-linux-x86_64` | Binário imutável versionado correspondente à release específica. |

### 8.2 Política Estrita de Retenção e Expurgos (Cloudflare R2)

A esteira de automação da Banca DEVE executar rotação atômica (`scripts/rotate-downloads.py`) a cada publicação, mantendo o armazenamento rigorosamente enxuto:
1. **Canal Estável (`stable`):** Retenção de **estritamente as 5 versões mais recentes**.
2. **Canal Noturno / Rolling (`nightly`):** Retenção de **estritamente as 10 versões mais recentes**.
3. Versões que ultrapassem essa janela temporal são expurgadas automaticamente do storage e do manifesto `index.json`, garantindo consumo previsível e custo mínimo.

### 8.3 Manifesto Estruturado Atômico (`index.json`)

Cada aplicativo mantém seu arquivo `index.json` atualizado pela esteira com o seguinte esquema:

```json
{
  "app": "navalha",
  "name": "Navalha",
  "updated_at": "2026-10-20T06:00:00Z",
  "channels": {
    "stable": {
      "latest_version": "0.5.0",
      "versions": [
        {
          "version": "0.5.0",
          "filename": "navalha-0.5.0-linux-x86_64",
          "url": "https://banca.2lp.in/downloads/navalha/navalha-0.5.0-linux-x86_64",
          "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
          "size_bytes": 14285700,
          "published_at": "2026-10-19T22:00:00Z"
        }
      ]
    },
    "nightly": {
      "latest_version": "0.5.0-dev.216+20261020.ba9aaf0",
      "versions": []
    }
  }
}
```

### 8.4 Script Instalador Universal (`install.sh`)

O script instalador hospedado na Banca DEVE:
1. Ser puramente POSIX compliant (`#!/bin/sh` sem bashismos).
2. Detectar o destino de instalação padrão:
   - Preferencialmente `~/.local/bin/` (sem privilégios de root).
   - Se executado como root ou com flag global: `/usr/local/bin/`.
3. Respeitar a variável de ambiente `CHANNEL` (`stable` por padrão; ou `CHANNEL=nightly`).
4. Verificar integridade via SHA-256 antes de tornar o binário executável (`chmod +x`).
5. Fornecer comando de execução rápida copiado em um clique:
   ```bash
   curl -sSf https://banca.2lp.in/downloads/<app>/install.sh | sh
   ```

### 8.5 Apresentação Visual e Vitrine Web (`site/downloads/`)

A interface web da Banca para o público DEVE:
1. Adotar design brutalista/cyberpunk sem frameworks pesados (Vanilla HTML5, CSS Grid/Flexbox e Vanilla JS).
2. Fornecer um seletor rápido com dropdown contendo todas as aplicações disponíveis.
3. Suportar deep linking direto via URL hash (ex: `https://banca.2lp.in/downloads/#navalha`), selecionando e focando o aplicativo imediatamente.
4. Exibir botões interativos para cópia de comando `curl` com feedback visual imediato.
5. Apresentar tabela comparativa de canais com tamanho, data e SHA-256 auditável.
6. Exibir o rodapé canônico em duas linhas de afirmação identitária transfeminista e soberania.

### 8.6 Veto Absoluto a Binários no Codeberg / Git

1. Nenhuma release no Codeberg conterá binários executáveis ou pacotes tarball nos anexos de release.
2. O repositório Git armazena exclusivamente código-fonte legível, testes e especificações.
3. Todas as compilações de release geradas pelo Forgejo Runner despacham diretamente os binários para a Banca via Cloudflare R2.

---

## 9. Arquitetura de Entrega Contínua de Containers (Continuous Deployment de Servidores)

Para assegurar atualização contínua, determinística e resiliente de aplicações conteinerizadas em servidores autohospedados (como os nós `obizero`, `obione` e `obitwo`), o perfil Lumen adota o protocolo de **Continuous Deployment (CD) Soberano**.

### 9.1 Descomissionamento Compulsório do Watchtower

1. O uso de agentes opacos de auto-atualização sem auditoria (como Watchtower) é **terminantemente vedado**.
2. É proibido depender de polling cego ou disparos de webhooks HTTP desprovidos de verificação síncrona de integridade.
3. Todas as diretivas e rótulos de containers legados `com.centurylinklabs.watchtower.enable` DEVEM ser removidos dos manifestos Docker Compose.

### 9.2 Tipologia de Alvos e Topologia de Deploy

A automação de entrega contínua reconhece duas categorias de nós de infraestrutura:

#### 1. Nó Central de Execução (`obizero` / Deploy Local Imediato)
- O executor da esteira de CI/CD (`forgejo-runner`) opera diretamente no host `obizero` com o socket do Docker montado (`docker_host: "automount"`).
- O processo de build (`docker build`) injeta a imagem compilada diretamente no Docker daemon local do host (`lumenpink/<app>:latest` e `:<version>`).
- O deploy local ocorre **imediatamente e sem qualquer download redundante da internet**:
  ```bash
  docker compose -f /home/lumen/stacks/<app>/compose.yml up -d --no-build <app>
  ```
- **Sondagem de Healthcheck Ativo:**
  O job de CD DEVE aguardar de forma síncrona até 15 segundos para confirmar que o container atingiu estado operacional:
  ```bash
  for i in $(seq 1 15); do
    STATUS=$(docker inspect --format='{{.State.Health.Status}}' <app> 2>/dev/null || echo "running")
    if [ "$STATUS" = "healthy" ] || [ "$STATUS" = "running" ]; then
      echo "✅ Container <app> operacional em produção!"
      break
    fi
    sleep 1
  done
  ```
- **Protocolo de Rollback em Falha:**
  Se o container falhar ou entrar em loop de reinício:
  1. A esteira emite os últimos registros de execução (`docker logs --tail 50 <app>`);
  2. Executa a restauração da versão estável anterior;
  3. Aborta o workflow com código de saída diferente de zero.
- **Limpeza de Armazenamento:**
  Após o reload bem-sucedido, executa-se compulsòriamente `docker image prune -f` para purgar camadas órfãs.

#### 2. Nós de Borda Remotos (`obione` e `obitwo` / Deploy via Tailscale)
- Para serviços distribuídos em nós de borda (ex.: `fuxico` e `tatupeba` no `obitwo`, ou `dns` e proxy reverso no `obione`):
- A esteira publica a imagem oficial no Docker Hub (`lumenpink/<app>:latest`).
- O deploy remoto é disparado de forma autenticada via SSH através da VPN segura Tailscale:
  ```bash
  ssh -o StrictHostKeyChecking=accept-new lumen@<no-tailscale> \
    "docker compose -f ~/stacks/<app>/compose.yml pull && \
     docker compose -f ~/stacks/<app>/compose.yml up -d && \
     docker image prune -f"
  ```
- O workflow confirma a saída com sucesso do comando remoto antes de encerrar o job.

### 9.3 Contrato Obrigatório de Healthchecks

Todo container de servidor publicado no ecossistema Lumen DEVE conter probes de integridade declarados no `Dockerfile` ou no respectivo `compose.yml`:
```yaml
healthcheck:
  test: ["CMD-SHELL", "curl -f http://localhost:<port>/healthz || exit 1"]
  interval: 30s
  timeout: 3s
  retries: 3
  start_period: 10s
```

---

