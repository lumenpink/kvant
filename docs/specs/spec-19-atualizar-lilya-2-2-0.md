# Especificação Técnica — Ticket #19
## Atualização da Metanorma GOST: Perfil Lilya 2.2.0 & Kollontai 2.2.0

### 1. Contexto e Motivação
O perfil operacional do ecossistema evoluiu para **Lilya 2.2.0**, que estende e herda normativamente o **Kollontai 2.2.0**.
Esta atualização alinha este repositório (`kvant`) com a base canônica, atualizando o baseline de compilador Rust para `rustc >= 1.100.0` e os endpoints normativos.

### 2. Árvore de Decisões (Registro para a Natasha)
- **Decisão 1: Atualização Cascata de Lilya 2.1.0 para 2.2.0**
  - *Alternativa descartada:* Manter Lilya 2.1.0 e tentar aplicar patches locais em `CUSTOM.md`.
  - *Motivo:* Natasha precisará de paridade uniforme entre todos os microsserviços e bibliotecas do ecossistema. Qualquer desvio gera divergência nas checagens estritas do `cccp check --strict`.
- **Decisão 2: Toolchain Baseline `rustc >= 1.100.0`**
  - *Alternativa descartada:* Manter `1.99.0`.
  - *Motivo:* Alinhamento mandatório com a edição 2024 estabilizada e convenções do perfil Kollontai 2.2.0.

### 3. Escopo das Alterações
1. `.gost/GOST.md`:
   - Atualização das URLs das camadas de precedência para `2.2.0`.
   - Atualização da versão declarada nos frontmatters de `2.1.0` para `2.2.0`.
   - Bump da tabela de requisitos de compilador Rust para `>= 1.100.0`.
2. `.gost/rules.toml`:
   - Atualização das `docs_url` das regras KOLLONTAI e LILYA para `2.2.0`.

### 4. Critérios de Aceite e Verificação
- [x] `.gost/GOST.md` e `.gost/rules.toml` sincronizados com Lilya 2.2.0.
- [x] `cccp check --strict` reporta conformidade integral (100% de adesão).
- [x] Testes e linters da stack executados com sucesso.
