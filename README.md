# Kvant (kv)

> **Micro terminal text editor in pure Rust with full keyboard navigation**  
> *Zero external dependencies · Non-modal · Pure POSIX termios · Gosplan Minimalist Architecture*

[![CI](https://codeberg.org/lumenpink/kvant/actions/workflows/ci.yml/badge.svg)](https://codeberg.org/lumenpink/kvant/actions)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Website](https://img.shields.io/badge/docs-kvant.2lp.in-purple)](https://kvant.2lp.in)

---

## 1. Visão Geral

O **Kvant** (e seu atalho rápido **`kv`**) é um micro-editor de texto para terminal desenvolvido em **Rust puro**, concebido especificamente para ser ultraleve, imediato e universalmente acessível via teclado.

Diferente de editores modais (como o `vi`) ou editores com árvores pesadas de dependências, o Kvant:
* **Zero dependências externas:** Construído estritamente sobre a biblioteca padrão do Rust e syscalls diretas POSIX (`termios` / `ioctl`), compilando em menos de 1 segundo.
* **Binário ultraleve e estático:** Binário final minúsculo (< 1 MB), sem runtime overhead.
* **Navegação universal por teclado:** Suporta setas padrão e navegação completa por teclas `Ctrl+` (estilo Readline / Emacs / Nano) para terminais restritos onde setas não estão disponíveis.
* **Rodapé com atalhos permanentes:** Informações claras e barra de ajuda sempre visíveis na tela.

---

## 2. Instalação

### 2.1 A partir do Código-Fonte (Cargo)
```bash
git clone ssh://git@codeberg.org/lumenpink/kvant.git
cd kvant
cargo install --path .
```
Isso instalará os binários **`kvant`** e **`kv`** no seu `~/.cargo/bin` (ou `~/.local/bin`).

### 2.2 Compilação Manual Rápida
```bash
cargo build --release
cp target/release/kv ~/.local/bin/kv
cp target/release/kvant ~/.local/bin/kvant
```

---

## 3. Uso & Atalhos de Teclado

```bash
kv [arquivo]
# ou
kvant [arquivo]
```

### 3.1 Tabela de Atalhos

| Tecla / Atalho | Função | Descrição |
| :--- | :--- | :--- |
| **`Setas`** ou **`^P` / `^N`** | Mover Vertical | Linha Anterior (`^P` / Cima) / Próxima Linha (`^N` / Baixo) |
| **`Setas`** ou **`^B` / `^F`** | Mover Horizontal | Caractere Anterior (`^B` / Esq) / Próximo (`^F` / Dir) |
| **`Home`** ou **`^A`** | Início da Linha | Move o cursor para o primeiro caractere da linha |
| **`End`** ou **`^E`** | Fim da Linha | Move o cursor para o final da linha atual |
| **`PageUp` / `PageDn`** | Rolar Página | Rola uma página inteira para cima ou para baixo |
| **`^K`** | Cortar Linha | Apaga do cursor até o final da linha (ou une a próxima linha) |
| **`^U`** | Limpar Linha | Remove a linha atual por completo |
| **`^D`** ou **`Delete`** | Deletar Frente | Apaga o caractere à frente do cursor |
| **`Backspace`** ou **`^H`** | Deletar Trás | Apaga o caractere imediatamente atrás do cursor |
| **`Enter`** | Nova Linha | Insere uma quebra de linha |
| **`Tab`** | Tabulação | Insere 4 espaços de indentação |
| **`^S`** | Gravar | Salva o arquivo no disco |
| **`^X`** ou **`^Q`** | Sair | Encerra o editor (solicita confirmação caso haja alterações não salvas) |

---

## 4. Integração com o CCCP / Divã

O Kvant é o editor prioritário integrado ao fluxo interativo de edição manual de corpo de tickets (`[b]`) do **CCCP Divã**:
```bash
cccp diva
```
Caso `kv` ou `kvant` estejam no seu `$PATH`, o Divã abre o editor diretamente para uma edição fluida e amigável.

---

## 5. Licença & Matriz Modular

Em conformidade com a norma técnica [GOST v1.1.0](.gost/GOST.md):

* **Código-Fonte e Binário CLI (`src/`):** Licenciados sob a [Apache License 2.0 (Apache-2.0)](LICENSE).
* **Identidade Visual, Emblema e Insígnia (`site/assets/logo.svg`):** Licenciados sob a [Creative Commons Atribuição-SemDerivações 4.0 Internacional (CC-BY-ND 4.0)](LICENSE-LOGO).
* **Mídias e Ilustrações:** Licenciados sob a [Creative Commons Atribuição-CompartilhaIgual 4.0 Internacional (CC-BY-SA 4.0)](LICENSE-MEDIA).
